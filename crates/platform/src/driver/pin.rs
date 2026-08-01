use aether_core::driver::{Driver, ReadDriver, WriteDriver};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

/// A universal GPIO pin driver wrapper for `embedded-hal` pins.
///
/// Occupies 0 bytes of RAM when `P` is a Zero-Sized Type (ZST)!
pub struct PinDriver<P> {
    pin: core::cell::UnsafeCell<P>,
}

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

impl<P: ErrorType> Driver for PinDriver<P>
where
    P::Error: core::error::Error,
{
    type Error = P::Error;
    type OpenOptions = ();
    type Handle = ();

    fn open(&self, _options: Self::OpenOptions) -> Result<Self::Handle, Self::Error> {
        Ok(())
    }
}

impl<P: OutputPin> WriteDriver for PinDriver<P>
where
    P::Error: core::error::Error,
{
    fn write(&self, _handle: &Self::Handle, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let pin = unsafe { &mut *self.pin.get() };
        if buf[0] == 0 {
            pin.set_low()?;
        } else {
            pin.set_high()?;
        }
        Ok(buf.len())
    }

    fn flush(&self, _handle: &Self::Handle) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P: InputPin> ReadDriver for PinDriver<P>
where
    P::Error: core::error::Error,
{
    fn read(&self, _handle: &Self::Handle, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let pin = unsafe { &mut *self.pin.get() };
        buf[0] = u8::from(pin.is_high()?);
        Ok(1)
    }
}
