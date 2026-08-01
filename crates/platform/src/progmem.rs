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
    #[inline(always)]
    pub fn read_byte(&self) -> u8 {
        read_byte(self.addr)
    }
}

/// Reads a single byte from Flash memory (ROM) at the specified pointer address.
#[inline(always)]
#[allow(clippy::inline_always, clippy::not_unsafe_ptr_arg_deref)]
pub fn read_byte(addr: *const u8) -> u8 {
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
// Target-Dependent Inner Storage Representation
// -----------------------------------------------------------------------------

#[cfg(target_arch = "avr")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PStrInner {
    ptr: ProgPtr<u8>,
    len: usize,
}

#[cfg(target_arch = "avr")]
impl PStrInner {
    #[inline(always)]
    const fn from_ptr(ptr: *const u8, len: usize) -> Self {
        Self {
            ptr: ProgPtr::new(ptr),
            len,
        }
    }

    #[inline(always)]
    const fn from_static_str(s: &'static str) -> Self {
        Self {
            ptr: ProgPtr::new(s.as_ptr()),
            len: s.len(),
        }
    }

    #[inline(always)]
    const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    fn read_byte(&self, index: usize) -> u8 {
        self.ptr.add(index).read()
    }
}

#[cfg(not(target_arch = "avr"))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PStrInner(&'static str);

#[cfg(not(target_arch = "avr"))]
impl PStrInner {
    #[inline(always)]
    const fn from_ptr(ptr: *const u8, len: usize) -> Self {
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        let s = match core::str::from_utf8(slice) {
            Ok(valid) => valid,
            Err(_) => "",
        };
        Self(s)
    }

    #[inline(always)]
    const fn from_static_str(s: &'static str) -> Self {
        Self(s)
    }

    #[inline(always)]
    const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn read_byte(&self, index: usize) -> u8 {
        self.0.as_bytes()[index]
    }
}

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

impl PStr {
    /// Creates a `PStr` from a raw Flash pointer and length.
    #[must_use]
    #[inline(always)]
    pub const fn from_ptr(ptr: *const u8, len: usize) -> Self {
        Self {
            inner: PStrInner::from_ptr(ptr, len),
        }
    }

    /// Creates a `PStr` from a static string slice.
    #[must_use]
    #[inline(always)]
    pub const fn from_static_str(s: &'static str) -> Self {
        Self {
            inner: PStrInner::from_static_str(s),
        }
    }

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

    /// Reads a single byte from the Flash string at `index`.
    #[must_use]
    #[inline(always)]
    pub fn read_byte(&self, index: usize) -> u8 {
        self.inner.read_byte(index)
    }
}

impl core::fmt::Display for PStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for i in 0..self.len() {
            let b = self.read_byte(i);
            f.write_str(core::str::from_utf8(core::slice::from_ref(&b)).unwrap_or("?"))?;
        }
        Ok(())
    }
}

#[cfg(feature = "ufmt")]
impl ufmt::uDisplay for PStr {
    fn fmt<W: ufmt::uWrite + ?Sized>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error> {
        for i in 0..self.len() {
            let b = self.read_byte(i);
            f.write_char(b as char)?;
        }
        Ok(())
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
        $crate::progmem::PStr::from_ptr(STR.as_ptr(), STR.len())
    }};
}

/// Macro to stream a static Flash (ROM) byte string directly to a `ufmt` writer.
#[cfg(feature = "ufmt")]
#[macro_export]
macro_rules! pwrite {
    ($writer:expr, $str_bytes:expr) => {{
        #[unsafe(link_section = ".progmem.data")]
        static STR: [u8; $str_bytes.len()] = *$str_bytes;
        let _ = ufmt::uwrite!($writer, "{}", $crate::progmem::PStr::from_ptr(STR.as_ptr(), STR.len()));
    }};
}
