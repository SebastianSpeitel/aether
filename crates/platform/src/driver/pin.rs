#[cfg(feature = "embedded")]
use embedded_hal::digital::{InputPin, OutputPin};

#[cfg(feature = "embedded")]
use aether_core::driver::{Driver, ReadDriver, WriteDriver};

/// A universal GPIO pin driver wrapper for `embedded-hal` pins.
///
/// Occupies 0 bytes of RAM when `P` is a Zero-Sized Type (ZST)!
#[cfg(feature = "embedded")]
pub struct PinDriver<P> {
    pin: core::cell::UnsafeCell<P>,
}

#[cfg(feature = "embedded")]
impl<P> PinDriver<P> {
    /// Creates a new `PinDriver` wrapping the provided `embedded-hal` pin.
    #[inline]
    pub const fn new(pin: P) -> Self {
        Self {
            pin: core::cell::UnsafeCell::new(pin),
        }
    }

    /// Consumes the `PinDriver` and returns the underlying `embedded-hal` pin.
    #[inline]
    pub fn into_inner(self) -> P {
        self.pin.into_inner()
    }
}

#[cfg(feature = "embedded")]
impl<P> Driver for PinDriver<P> {
    type Error = core::convert::Infallible;
    type OpenOptions = ();
    type Handle = ();

    #[inline]
    fn open(&self, _options: ()) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(feature = "embedded")]
impl<P: OutputPin> WriteDriver for PinDriver<P> {
    #[inline]
    fn write(&self, _handle: &(), buf: &[u8]) -> Result<usize, Self::Error> {
        if let Some(&state) = buf.first() {
            let pin = unsafe { &mut *self.pin.get() };
            if state != 0 {
                let _ = pin.set_high();
            } else {
                let _ = pin.set_low();
            }
        }
        Ok(1)
    }
}

#[cfg(feature = "embedded")]
impl<P: InputPin> ReadDriver for PinDriver<P> {
    #[inline]
    fn read(&self, _handle: &(), buf: &mut [u8]) -> Result<usize, Self::Error> {
        if !buf.is_empty() {
            let pin = unsafe { &mut *self.pin.get() };
            let is_high = pin.is_high().unwrap_or(false);
            buf[0] = if is_high { 1 } else { 0 };
        }
        Ok(1)
    }
}
