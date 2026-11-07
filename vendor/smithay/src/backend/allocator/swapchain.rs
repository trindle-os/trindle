use std::{
    fmt,
    ops::Deref,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc,
    },
};

use tracing::instrument;

use crate::backend::allocator::{Allocator, Buffer, Fourcc, Modifier};
use crate::utils::user_data::UserDataMap;

use super::dmabuf::{AsDmabuf, Dmabuf};

pub const SLOT_CAP: usize = 4;

pub struct Swapchain<A: Allocator> {
    pub allocator: A,

    width: u32,
    height: u32,
    fourcc: Fourcc,
    modifiers: Vec<Modifier>,

    slots: [Arc<InternalSlot<A::Buffer>>; SLOT_CAP],
}

impl<A: Allocator> fmt::Debug for Swapchain<A> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Swapchain")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("fourcc", &self.fourcc)
            .field("modifiers", &self.modifiers)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub struct Slot<B: Buffer>(Arc<InternalSlot<B>>);

#[derive(Debug)]
struct InternalSlot<B: Buffer> {
    buffer: Option<B>,
    acquired: AtomicBool,
    age: AtomicU8,
    userdata: UserDataMap,
}

impl<B: Buffer> Slot<B> {
    pub fn userdata(&self) -> &UserDataMap {
        &self.0.userdata
    }

    pub fn age(&self) -> u8 {
        self.0.age.load(Ordering::SeqCst)
    }
}

impl<B: Buffer> Default for InternalSlot<B> {
    fn default() -> Self {
        InternalSlot {
            buffer: None,
            acquired: AtomicBool::new(false),
            age: AtomicU8::new(0),
            userdata: UserDataMap::new(),
        }
    }
}

impl<B: Buffer> Deref for Slot<B> {
    type Target = B;
    fn deref(&self) -> &B {
        Option::as_ref(&self.0.buffer).unwrap()
    }
}

impl<B: Buffer + AsDmabuf> AsDmabuf for Slot<B> {
    type Error = <B as AsDmabuf>::Error;

    fn export(&self) -> Result<super::dmabuf::Dmabuf, Self::Error> {
        let maybe_dmabuf = self.userdata().get::<Dmabuf>();
        if maybe_dmabuf.is_none() {
            let dmabuf = (**self).export()?;
            self.userdata().insert_if_missing(|| dmabuf);
        }

        Ok(self.userdata().get::<Dmabuf>().cloned().unwrap())
    }
}

impl<B: Buffer> Drop for Slot<B> {
    fn drop(&mut self) {
        self.0.acquired.store(false, Ordering::SeqCst);
    }
}

impl<A> Swapchain<A>
where
    A: Allocator,
{
    pub fn new(
        allocator: A,
        width: u32,
        height: u32,
        fourcc: Fourcc,
        modifiers: Vec<Modifier>,
    ) -> Swapchain<A> {
        Swapchain {
            allocator,
            width,
            height,
            fourcc,
            modifiers,
            slots: Default::default(),
        }
    }

    #[instrument(level = "trace", skip_all, err)]
    #[profiling::function]
    pub fn acquire(&mut self) -> Result<Option<Slot<A::Buffer>>, A::Error> {
        if let Some(free_slot) = self
            .slots
            .iter_mut()
            .find(|s| !s.acquired.swap(true, Ordering::SeqCst))
        {
            if free_slot.buffer.is_none() {
                let free_slot = Arc::get_mut(free_slot).expect("Acquired was false, but Arc is not unique?");
                match self
                    .allocator
                    .create_buffer(self.width, self.height, self.fourcc, &self.modifiers)
                {
                    Ok(buffer) => free_slot.buffer = Some(buffer),
                    Err(err) => {
                        free_slot.acquired.store(false, Ordering::SeqCst);
                        return Err(err);
                    }
                }
            }
            assert!(free_slot.buffer.is_some());
            return Ok(Some(Slot(free_slot.clone())));
        }

        Ok(None)
    }

    pub fn submitted(&mut self, slot: &Slot<A::Buffer>) {
        if !self.slots.iter().any(|other| Arc::ptr_eq(&slot.0, other)) {
            return;
        }

        slot.0.age.store(1, Ordering::SeqCst);
        for other_slot in &mut self.slots {
            if !Arc::ptr_eq(other_slot, &slot.0) && other_slot.buffer.is_some() {
                let res = other_slot
                    .age
                    .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |age| {
                        if age > 0 {
                            age.checked_add(1)
                        } else {
                            Some(0)
                        }
                    });
                if res.is_err() {
                    *other_slot = Default::default();
                }
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width == width && self.height == height {
            return;
        }

        self.width = width;
        self.height = height;
        self.slots = Default::default();
    }

    pub fn reset_buffers(&mut self) {
        for slot in &mut self.slots {
            *slot = Default::default();
        }
    }

    pub fn reset_buffer_ages(&mut self) {
        for slot in &mut self.slots {
            match Arc::get_mut(slot) {
                Some(slot) => slot.age = AtomicU8::new(0),
                None => *slot = Default::default(),
            }
        }
    }

    pub fn format(&self) -> Fourcc {
        self.fourcc
    }

    pub fn modifiers(&self) -> &[Modifier] {
        &self.modifiers
    }
}
