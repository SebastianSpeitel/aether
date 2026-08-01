use core::error::Error;

#[cfg(feature = "std")]
extern crate std;

/// The fundamental driver interface implemented by hardware peripherals,
/// character/block devices, filesystems, and network drivers.
pub trait Driver {
    type Error: Error;

    /// The options/parameters required to open an instance of a resource from this driver.
    ///
    /// Examples:
    /// - `()` for singletons (System Console, System LED, Main Flash)
    /// - `u8` for hardware port channels (UART 0 vs 1)
    /// - `str` for path-based files (`"/dev/sda"`, `"config.json"`)
    type OpenOptions;

    /// The safe, owned handle type representing an active open device instance.
    ///
    /// Drivers that require resource cleanup (e.g. closing file descriptors or releasing sockets)
    /// implement [`Drop`] on `Handle`. For stateless/singleton devices, `Handle` can be `()`.
    type Handle;

    /// Opens a resource managed by this driver using `options`, returning a safe `Handle`.
    fn open(&self, options: Self::OpenOptions) -> Result<Self::Handle, Self::Error>;
}

/// Sub-capability trait for drivers supporting handle duplication (Linux `dup()` / `try_clone`).
pub trait CloneDriver: Driver {
    /// Attempts to duplicate/clone an open `handle`.
    ///
    /// The returned `Handle` references the same underlying resource, but has an independent
    /// lifetime and [`Drop`] cleanup.
    fn try_clone(&self, handle: &Self::Handle) -> Result<Self::Handle, Self::Error>;
}

/// Sub-capability trait for drivers supporting sequential streaming reads (UART, Sockets, Pipes).
pub trait ReadDriver: Driver {
    /// Reads sequential data from `handle` into `buf`.
    fn read(&self, handle: &Self::Handle, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

/// Sub-capability trait for drivers supporting sequential streaming writes (UART, Sockets, Logs).
pub trait WriteDriver: Driver {
    /// Writes sequential data from `buf` to `handle`.
    fn write(&self, handle: &Self::Handle, buf: &[u8]) -> Result<usize, Self::Error>;

    /// Flushes any pending cached writes for `handle`.
    #[inline]
    fn flush(&self, handle: &Self::Handle) -> Result<(), Self::Error> {
        let _ = handle;
        Ok(())
    }
}

/// Sub-capability trait for drivers supporting positional reads (Files, Flash memory).
pub trait PositionedReadDriver: Driver {
    /// Reads data from `handle` starting at `offset` into `buf`.
    fn read_at(
        &self,
        handle: &Self::Handle,
        buf: &mut [u8],
        offset: u64,
    ) -> Result<usize, Self::Error>;
}

/// Sub-capability trait for drivers supporting positional writes (Files, Flash memory).
pub trait PositionedWriteDriver: Driver {
    /// Writes data from `buf` to `handle` starting at `offset`.
    fn write_at(
        &self,
        handle: &Self::Handle,
        buf: &[u8],
        offset: u64,
    ) -> Result<usize, Self::Error>;
}

/// Sub-capability trait for block storage drivers (SD Card, NVMe, Sector Flash).
pub trait BlockDriver: Driver {
    /// Returns the sector size in bytes (e.g. 512 or 4096).
    fn sector_size(&self) -> usize;

    /// Reads a sector from block index `lba` into `buf`.
    fn read_sector(
        &self,
        handle: &Self::Handle,
        lba: u64,
        buf: &mut [u8],
    ) -> Result<(), Self::Error>;

    /// Writes a sector to block index `lba` from `buf`.
    fn write_sector(
        &self,
        handle: &Self::Handle,
        lba: u64,
        buf: &[u8],
    ) -> Result<(), Self::Error>;
}

/// Sub-capability trait for drivers supporting hardware control commands (Linux ioctl).
pub trait IoctlDriver: Driver {
    /// Executes a driver-specific control command on `handle`.
    fn ioctl(&self, handle: &Self::Handle, cmd: u32, arg: usize) -> Result<usize, Self::Error>;
}

/// Capability trait for kernels/contexts that provide access to a `Driver` type `D`.
pub trait HasDriver<D: Driver + ?Sized> {
    type DriverRef<'a>: core::ops::Deref<Target = D> + 'a
    where
        Self: 'a;

    /// Acquires a reference to the `Driver` instance `D`.
    fn get_driver<'a>(&'a self) -> Self::DriverRef<'a>;
}

/// Adapter converting a `ReadDriver` handle reference into a `std::io::Read` implementor.
#[cfg(feature = "std")]
pub struct StdReadAdapter<'a, 'h, D: ReadDriver + ?Sized> {
    pub driver: &'a D,
    pub handle: &'h D::Handle,
}

#[cfg(feature = "std")]
impl<'a, 'h, D: ReadDriver + ?Sized> std::io::Read for StdReadAdapter<'a, 'h, D> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.driver
            .read(self.handle, buf)
            .map_err(|e| std::io::Error::other(std::format!("{e:?}")))
    }
}

/// Adapter converting a `WriteDriver` handle reference into a `std::io::Write` implementor.
#[cfg(feature = "std")]
pub struct StdWriteAdapter<'a, 'h, D: WriteDriver + ?Sized> {
    pub driver: &'a D,
    pub handle: &'h D::Handle,
}

#[cfg(feature = "std")]
impl<'a, 'h, D: WriteDriver + ?Sized> std::io::Write for StdWriteAdapter<'a, 'h, D> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.driver
            .write(self.handle, buf)
            .map_err(|e| std::io::Error::other(std::format!("{e:?}")))
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.driver
            .flush(self.handle)
            .map_err(|e| std::io::Error::other(std::format!("{e:?}")))
    }
}
