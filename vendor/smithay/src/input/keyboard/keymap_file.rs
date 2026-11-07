use std::ffi::CString;
use std::os::unix::io::{AsFd, BorrowedFd};

use sha2::{Digest, Sha256};
use tracing::error;
use xkbcommon::xkb::{self, Keymap, KEYMAP_FORMAT_TEXT_V1};

use crate::utils::SealedFile;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct KeymapFileId([u8; 32]);

impl KeymapFileId {
    fn for_keymap(keymap: &str) -> Self {
        Self(Sha256::digest(keymap).as_slice().try_into().unwrap())
    }
}

#[derive(Debug)]
pub struct KeymapFile {
    sealed: Option<SealedFile>,
    keymap: String,
    id: KeymapFileId,
}

impl KeymapFile {
    pub fn new(keymap: &Keymap) -> Self {
        let name = c"smithay-keymap";
        let keymap = keymap.get_as_string(KEYMAP_FORMAT_TEXT_V1);
        let sealed = SealedFile::with_content(name, &CString::new(keymap.as_str()).unwrap());

        if let Err(err) = sealed.as_ref() {
            error!("Error when creating sealed keymap file: {}", err);
        }

        Self {
            id: KeymapFileId::for_keymap(&keymap),
            sealed: sealed.ok(),
            keymap,
        }
    }

    #[cfg(feature = "wayland_frontend")]
    pub(crate) fn change_keymap(&mut self, keymap: &Keymap) {
        let keymap = keymap.get_as_string(xkb::KEYMAP_FORMAT_TEXT_V1);

        let name = c"smithay-keymap-file";
        let sealed = SealedFile::with_content(name, &CString::new(keymap.clone()).unwrap());

        if let Err(err) = sealed.as_ref() {
            error!("Error when creating sealed keymap file: {}", err);
        }

        self.id = KeymapFileId::for_keymap(&keymap);
        self.sealed = sealed.ok();
        self.keymap = keymap;
    }

    #[cfg(feature = "wayland_frontend")]
    pub fn with_fd<F>(&self, supports_sealed: bool, cb: F) -> Result<(), std::io::Error>
    where
        F: FnOnce(BorrowedFd<'_>, usize),
    {
        use std::{io::{Seek, Write}, path::PathBuf};

        if let Some(file) = supports_sealed.then_some(self.sealed.as_ref()).flatten() {
            cb(file.as_fd(), file.size());
        } else {
            let dir = std::env::var_os("XDG_RUNTIME_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(std::env::temp_dir);

            let mut file = tempfile::tempfile_in(dir)?;
            file.write_all(self.keymap.as_bytes())?;
            file.write_all(&[0])?;
            file.flush()?;
            file.rewind()?;

            cb(file.as_fd(), self.keymap.len() + 1);
        }
        Ok(())
    }

    pub fn send(
        &self,
        keyboard: &wayland_server::protocol::wl_keyboard::WlKeyboard,
    ) -> Result<(), std::io::Error> {
        use wayland_server::{protocol::wl_keyboard::KeymapFormat, Resource};

        self.with_fd(keyboard.version() >= 7, |fd, size| {
            keyboard.keymap(KeymapFormat::XkbV1, fd, size as u32);
        })
    }

    pub(crate) fn id(&self) -> KeymapFileId {
        self.id
    }
}
