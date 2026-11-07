use std::sync::atomic::{AtomicU32, Ordering};

pub static SERIAL_COUNTER: SerialCounter = SerialCounter::new();

#[derive(Debug, Copy, Clone)]
pub struct Serial(pub(crate) u32);

impl PartialEq for Serial {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Serial {}

impl PartialOrd for Serial {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let distance = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        if distance < u32::MAX / 2 {
            self.0.partial_cmp(&other.0)
        } else {
            other.0.partial_cmp(&self.0)
        }
    }
}

impl From<u32> for Serial {
    #[inline]
    fn from(n: u32) -> Self {
        Serial(n)
    }
}

impl From<Serial> for u32 {
    #[inline]
    fn from(serial: Serial) -> u32 {
        serial.0
    }
}

impl Serial {
    #[inline]
    pub fn is_no_older_than(&self, other: &Serial) -> bool {
        other <= self
    }
}

#[derive(Debug)]
pub struct SerialCounter {
    serial: AtomicU32,
}

impl Default for SerialCounter {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SerialCounter {
    pub const fn new() -> Self {
        Self {
            serial: AtomicU32::new(1),
        }
    }

    pub fn next_serial(&self) -> Serial {
        let _ = self
            .serial
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::SeqCst);
        Serial(self.serial.fetch_add(1, Ordering::AcqRel))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_serial_counter(initial_value: u32) -> SerialCounter {
        SerialCounter {
            serial: AtomicU32::new(initial_value),
        }
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn serial_equals_self() {
        let counter = create_serial_counter(0);
        let serial = counter.next_serial();
        assert!(serial == serial);
    }

    #[test]
    fn consecutive_serials() {
        let counter = create_serial_counter(0);
        let serial1 = counter.next_serial();
        let serial2 = counter.next_serial();
        assert!(serial1 < serial2);
    }

    #[test]
    fn non_consecutive_serials() {
        let skip_serials = 147;

        let counter = create_serial_counter(0);
        let serial1 = counter.next_serial();
        for _ in 0..skip_serials {
            let _ = counter.next_serial();
        }
        let serial2 = counter.next_serial();
        assert!(serial1 < serial2);
    }

    #[test]
    fn serial_wrap_around() {
        let counter = create_serial_counter(u32::MAX);
        let serial1 = counter.next_serial();
        let serial2 = counter.next_serial();

        assert!(serial1 == u32::MAX.into());
        assert!(serial2 == 1.into());

        assert!(serial1 < serial2);
    }
}
