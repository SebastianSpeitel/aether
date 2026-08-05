//! Program Memory (Flash/ROM) utilities for microcontrollers.
//!
//! Provides static Flash string storage and byte access macros (`pstr!`, `pwrite!`)
//! with zero-RAM footprint on microcontrollers like AVR while supporting full
//! cross-platform host testing (`cargo test`).

/// An opaque pointer to memory residing in Program Memory (Flash/ROM).
///
/// This type intentionally DOES NOT implement `Deref` or raw pointer dereferencing,
/// preventing accidental RAM reads of Flash memory addresses.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProgPtr<T: ?Sized = u8> {
    addr: *const T,
}

unsafe impl<T: ?Sized> Send for ProgPtr<T> {}
unsafe impl<T: ?Sized> Sync for ProgPtr<T> {}

impl<T: ?Sized> ProgPtr<T> {
    /// Creates a new `ProgPtr` from a raw address pointer.
    #[must_use]
    #[inline(always)]
    pub const fn new(addr: *const T) -> Self {
        Self { addr }
    }

    /// Returns the underlying raw pointer address.
    #[must_use]
    #[inline(always)]
    pub const fn as_ptr(&self) -> *const T {
        self.addr
    }

    /// Offset the pointer by `count` elements.
    #[must_use]
    #[inline(always)]
    pub fn add(&self, count: usize) -> Self
    where
        T: Sized,
    {
        Self {
            addr: unsafe { self.addr.add(count) },
        }
    }
}

impl ProgPtr<u8> {
    /// Reads the byte at this Flash location safely.
    #[must_use]
    #[inline]
    pub fn read_byte(&self) -> u8 {
        unsafe { read_byte(self.addr) }
    }
}

/// Reads a single byte from Flash memory (ROM) at the specified pointer address.
///
/// # Safety
/// The caller must ensure that `addr` points to a valid byte in program flash memory (or RAM fallback).
#[inline]
pub(crate) unsafe fn read_byte(addr: *const u8) -> u8 {
    #[cfg(target_arch = "avr")]
    {
        let ptr_u16 = addr as u16;
        let low = ptr_u16 as u8;
        let high = (ptr_u16 >> 8) as u8;
        let byte: u8;
        unsafe {
            core::arch::asm!(
                "lpm {out}, Z",
                out = out(reg) byte,
                in("r30") low,
                in("r31") high,
            );
        }
        byte
    }
    #[cfg(not(target_arch = "avr"))]
    {
        unsafe { *addr }
    }
}

// -----------------------------------------------------------------------------
// Target-Dependent System Representation
// -----------------------------------------------------------------------------

#[cfg(target_arch = "avr")]
mod sys {
    use super::{PStr, ProgPtr};

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub(super) struct PStrInner {
        ptr: ProgPtr<u8>,
        len: usize,
    }

    unsafe impl Send for PStrInner {}
    unsafe impl Sync for PStrInner {}

    impl PStrInner {
        #[inline(always)]
        pub const fn len(&self) -> usize {
            self.len
        }
    }

    #[inline(always)]
    pub const unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> PStr {
        PStr {
            inner: PStrInner {
                ptr: ProgPtr::new(ptr),
                len,
            },
        }
    }

    impl core::fmt::Display for PStrInner {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            for i in 0..self.len {
                let b = self.ptr.add(i).read_byte();
                f.write_str(core::str::from_utf8(core::slice::from_ref(&b)).unwrap_or("?"))?;
            }
            Ok(())
        }
    }

    #[cfg(feature = "ufmt")]
    impl ufmt::uDisplay for PStrInner {
        fn fmt<W: ufmt::uWrite + ?Sized>(
            &self,
            f: &mut ufmt::Formatter<'_, W>,
        ) -> Result<(), W::Error> {
            for i in 0..self.len {
                let b = self.ptr.add(i).read_byte();
                f.write_char(b as char)?;
            }
            Ok(())
        }
    }

    impl From<&'static str> for PStrInner {
        #[inline(always)]
        fn from(s: &'static str) -> Self {
            Self {
                ptr: ProgPtr::new(s.as_ptr()),
                len: s.len(),
            }
        }
    }
}

#[cfg(not(target_arch = "avr"))]
mod sys {
    use super::PStr;

    pub(super) type PStrInner = &'static str;

    #[inline(always)]
    #[must_use]
    pub const unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> PStr {
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        let s = unsafe { core::str::from_utf8_unchecked(slice) };
        PStr { inner: s }
    }
}

use sys::PStrInner;
pub use sys::from_raw_parts;

// -----------------------------------------------------------------------------
// Concrete `PStr` Wrapper (Unified API Across All Platforms)
// -----------------------------------------------------------------------------

/// A reference to a string stored in Program Memory (Flash/ROM).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PStr {
    inner: PStrInner,
}

unsafe impl Send for PStr {}
unsafe impl Sync for PStr {}

impl From<&'static str> for PStr {
    #[inline(always)]
    fn from(s: &'static str) -> Self {
        Self {
            inner: PStrInner::from(s),
        }
    }
}

impl PStr {
    /// Returns the length of the Flash string in bytes.
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` if the Flash string is empty.
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl core::fmt::Display for PStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.inner, f)
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDisplay for PStr {
    fn fmt<W: ufmt::uWrite + ?Sized>(
        &self,
        f: &mut ufmt::Formatter<'_, W>,
    ) -> Result<(), W::Error> {
        #[cfg(target_arch = "avr")]
        {
            ufmt::uDisplay::fmt(&self.inner, f)
        }
        #[cfg(not(target_arch = "avr"))]
        {
            f.write_str(self.inner)
        }
    }
}

// -----------------------------------------------------------------------------
// Macros
// -----------------------------------------------------------------------------

/// Macro to define a static byte array in Flash (ROM) memory (`.progmem.data`)
/// and return a `PStr`.
#[macro_export]
macro_rules! pstr {
    ($str_bytes:expr) => {{
        #[unsafe(link_section = ".progmem.data")]
        static STR: [u8; $str_bytes.len()] = *$str_bytes;
        unsafe { $crate::progmem::from_raw_parts(STR.as_ptr(), STR.len()) }
    }};
}

/// Macro to stream a static Flash (ROM) byte string directly to a `ufmt` writer.
#[cfg(feature = "ufmt")]
#[macro_export]
macro_rules! pwrite {
    ($writer:expr, $str_bytes:expr) => {{
        #[unsafe(link_section = ".progmem.data")]
        static STR: [u8; $str_bytes.len()] = *$str_bytes;
        let _ = ufmt::uwrite!($writer, "{}", unsafe {
            $crate::progmem::from_raw_parts(STR.as_ptr(), STR.len())
        });
    }};
}
