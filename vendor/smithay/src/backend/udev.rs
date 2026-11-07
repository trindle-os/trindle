//! `udev` related functionality for automated device scanning.

use libc::dev_t;
use rustix::fs::stat;
use std::{
    collections::HashMap,
    ffi::OsString,
    fmt, io,
    os::unix::io::{AsFd, BorrowedFd},
    path::{Path, PathBuf},
};
use udev::{Enumerator, EventType, MonitorBuilder, MonitorSocket};

use calloop::{EventSource, Interest, Mode, Poll, PostAction, Readiness, Token, TokenFactory};

use tracing::{debug, debug_span, info, warn};

pub struct UdevBackend {
    devices: HashMap<dev_t, PathBuf>,
    monitor: MonitorSocket,
    token: Option<Token>,
    span: tracing::Span,
}

impl fmt::Debug for UdevBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use udev::AsRaw;
        f.debug_struct("UdevBackend")
            .field("devices", &self.devices)
            .field("monitor", &format!("MonitorSocket ({:?})", self.monitor.as_raw()))
            .finish()
    }
}

impl AsFd for UdevBackend {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.monitor.as_fd()
    }
}

impl UdevBackend {
    pub fn new<S: AsRef<str>>(seat: S) -> io::Result<UdevBackend> {
        let seat = seat.as_ref();
        let span = debug_span!("backend_udev", seat = seat.to_string());
        let _guard = span.enter();

        let devices = all_gpus(seat)?
            .into_iter()
            .flat_map(|path| match stat(&path) {
                Ok(stat) => Some((stat.st_rdev, path)),
                Err(err) => {
                    warn!("Unable to get id of {:?}, Error: {:?}. Skipping", path, err);
                    None
                }
            })
            .collect();

        let monitor = MonitorBuilder::new()?.match_subsystem("drm")?.listen()?;

        drop(_guard);
        Ok(UdevBackend {
            devices,
            monitor,
            token: None,
            span,
        })
    }

    pub fn device_list(&self) -> impl Iterator<Item = (dev_t, &Path)> {
        self.devices.iter().map(|(&id, path)| (id, path.as_ref()))
    }
}

impl EventSource for UdevBackend {
    type Event = UdevEvent;
    type Metadata = ();
    type Ret = ();
    type Error = io::Error;

    #[profiling::function]
    fn process_events<F>(
        &mut self,
        _: Readiness,
        token: Token,
        mut callback: F,
    ) -> std::io::Result<PostAction>
    where
        F: FnMut(UdevEvent, &mut ()),
    {
        if Some(token) != self.token {
            return Ok(PostAction::Continue);
        }

        let _guard = self.span.enter();
        for event in self.monitor.iter() {
            debug!(
                "Udev event: type={}, devnum={:?} devnode={:?}",
                event.event_type(),
                event.devnum(),
                event.devnode()
            );
            match event.event_type() {
                EventType::Add => {
                    if let (Some(path), Some(devnum)) = (event.devnode(), event.devnum()) {
                        info!("New device: #{} at {}", devnum, path.display());
                        if self.devices.insert(devnum, path.to_path_buf()).is_none() {
                            callback(
                                UdevEvent::Added {
                                    device_id: devnum,
                                    path: path.to_path_buf(),
                                },
                                &mut (),
                            );
                        }
                    }
                }
                EventType::Remove => {
                    if let Some(devnum) = event.devnum() {
                        info!("Device removed: #{}", devnum);
                        if self.devices.remove(&devnum).is_some() {
                            callback(UdevEvent::Removed { device_id: devnum }, &mut ());
                        }
                    }
                }
                EventType::Change => {
                    if let Some(devnum) = event.devnum() {
                        info!("Device changed: #{}", devnum);
                        if self.devices.contains_key(&devnum) {
                            callback(UdevEvent::Changed { device_id: devnum }, &mut ());
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(PostAction::Continue)
    }

    fn register(&mut self, poll: &mut Poll, factory: &mut TokenFactory) -> calloop::Result<()> {
        self.token = Some(factory.token());
        // Safety: the fd is owned by the UdevBackend and cannot be closed before it is removed from the event loop
        unsafe { poll.register(self.as_fd(), Interest::READ, Mode::Level, self.token.unwrap()) }
    }

    fn reregister(&mut self, poll: &mut Poll, factory: &mut TokenFactory) -> calloop::Result<()> {
        self.token = Some(factory.token());
        poll.reregister(self.as_fd(), Interest::READ, Mode::Level, self.token.unwrap())
    }

    fn unregister(&mut self, poll: &mut Poll) -> calloop::Result<()> {
        self.token = None;
        poll.unregister(self.as_fd())
    }
}

#[derive(Debug)]
pub enum UdevEvent {
    Added {
        device_id: dev_t,
        path: PathBuf,
    },
    Changed {
        device_id: dev_t,
    },
    Removed {
        device_id: dev_t,
    },
}

pub fn primary_gpu<S: AsRef<str>>(seat: S) -> io::Result<Option<PathBuf>> {
    let mut enumerator = Enumerator::new()?;
    enumerator.match_subsystem("drm")?;
    enumerator.match_sysname("card[0-9]*")?;

    if let Some(path) = enumerator
        .scan_devices()?
        .filter(|device| {
            let seat_name = device
                .property_value("ID_SEAT")
                .map(|x| x.to_os_string())
                .unwrap_or_else(|| OsString::from("seat0"));
            if seat_name == *seat.as_ref() {
                if let Ok(Some(pci)) = device.parent_with_subsystem(Path::new("pci")) {
                    if let Some(id) = pci.attribute_value("boot_vga") {
                        return id == "1";
                    }
                }
            }
            false
        })
        .flat_map(|device| device.devnode().map(PathBuf::from))
        .next()
    {
        Ok(Some(path))
    } else {
        all_gpus(seat).map(|all| all.into_iter().next())
    }
}

pub fn all_gpus<S: AsRef<str>>(seat: S) -> io::Result<Vec<PathBuf>> {
    let mut enumerator = Enumerator::new()?;
    enumerator.match_subsystem("drm")?;
    enumerator.match_sysname("card[0-9]*")?;
    let mut gpus = enumerator
        .scan_devices()?
        .filter(|device| {
            device
                .property_value("ID_SEAT")
                .map(|x| x.to_os_string())
                .unwrap_or_else(|| OsString::from("seat0"))
                == *seat.as_ref()
        })
        .flat_map(|device| device.devnode().map(PathBuf::from))
        .collect::<Vec<_>>();
    gpus.sort();
    Ok(gpus)
}

pub fn driver(dev: dev_t) -> io::Result<Option<OsString>> {
    let mut enumerator = Enumerator::new()?;
    enumerator.match_subsystem("drm")?;
    enumerator.match_sysname("card[0-9]*")?;
    Ok(enumerator
        .scan_devices()?
        .filter(|device| device.devnum() == Some(dev))
        .flat_map(|dev| {
            let mut device = Some(dev);
            while let Some(dev) = device {
                if dev.driver().is_some() {
                    return dev.driver().map(std::ffi::OsStr::to_os_string);
                }
                device = dev.parent();
            }
            None
        })
        .next())
}
