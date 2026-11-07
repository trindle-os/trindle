use std::{
    ffi::CStr,
    fs::File,
    io::Write,
    os::unix::io::{AsFd, AsRawFd, BorrowedFd, RawFd},
};

#[derive(Debug)]
pub struct SealedFile {
    file: File,
    size: usize,
}

impl SealedFile {
    pub fn with_content(name: &CStr, contents: &CStr) -> Result<Self, std::io::Error> {
        Self::with_data(name, contents.to_bytes_with_nul())
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "android"))]
    pub fn with_data(name: &CStr, data: &[u8]) -> Result<Self, std::io::Error> {
        use rustix::fs::{MemfdFlags, SealFlags};
        use std::io::Seek;

        let fd = rustix::fs::memfd_create(name, MemfdFlags::CLOEXEC | MemfdFlags::ALLOW_SEALING)?;

        let mut file: File = fd.into();
        file.write_all(data)?;
        file.flush()?;

        file.seek(std::io::SeekFrom::Start(0))?;

        rustix::fs::fcntl_add_seals(
            &file,
            SealFlags::SEAL | SealFlags::SHRINK | SealFlags::GROW | SealFlags::WRITE,
        )?;

        Ok(Self {
            file,
            size: data.len(),
        })
    }

    #[cfg(not(any(target_os = "linux", target_os = "freebsd", target_os = "android")))]
    pub fn with_data(name: &CStr, data: &[u8]) -> Result<Self, std::io::Error> {
        use rand::{distr::Alphanumeric, Rng};
        use rustix::{
            io::Errno,
            shm::{self, Mode},
        };

        let mut rng = rand::rng();

        let mut n = 0;
        let (shm_name, mut file) = loop {
            let mut shm_name = name.to_bytes().to_owned();
            shm_name.push(b'-');
            shm_name.extend((0..7).map(|_| rng.sample(Alphanumeric)));
            let fd = shm::open(
                shm_name.as_slice(),
                shm::OFlags::RDWR | shm::OFlags::CREATE | shm::OFlags::EXCL,
                Mode::RWXU,
            );
            if !matches!(fd, Err(Errno::EXIST)) || n > 3 {
                break (shm_name, File::from(fd?));
            }
            n += 1;
        };

        let fd_rdonly = shm::open(shm_name.as_slice(), shm::OFlags::RDONLY, Mode::empty())?;
        let file_rdonly = File::from(fd_rdonly);

        let _ = shm::unlink(shm_name.as_slice());

        file.write_all(data)?;
        file.flush()?;

        Ok(Self {
            file: file_rdonly,
            size: data.len(),
        })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl AsRawFd for SealedFile {
    fn as_raw_fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

impl AsFd for SealedFile {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.file.as_fd()
    }
}
