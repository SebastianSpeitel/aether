use core::any::Any;

use crate::allocator::Allocator;
use crate::capability::{HasAllocator, HasDriver};
use crate::driver::Driver;
use crate::kernel::Kernel;

/// Sentinel marker representing the end of a type-level driver chain.
#[derive(Debug, Clone, Copy, Default)]
pub struct NilDrivers;

/// Linked HList node holding driver `D` at the head and `Rest` as the tail chain.
#[derive(Debug, Clone, Copy)]
pub struct DriverNode<D, Rest> {
    pub driver: D,
    pub rest: Rest,
}

/// Dynamic/static recursive querying trait for type-level driver lookup.
pub trait DriverChain {
    fn query_driver<T: 'static>(&self) -> Option<&T>;
}

impl DriverChain for NilDrivers {
    #[inline]
    fn query_driver<T: 'static>(&self) -> Option<&T> {
        None
    }
}

impl<D: 'static, Rest: DriverChain> DriverChain for DriverNode<D, Rest> {
    #[inline]
    fn query_driver<T: 'static>(&self) -> Option<&T> {
        if let Some(val) = <dyn Any>::downcast_ref::<T>(&self.driver) {
            Some(val)
        } else {
            self.rest.query_driver::<T>()
        }
    }
}

/// A composite kernel assembled fluently via [`KernelBuilder`].
pub struct CompositeKernel<A, Drivers> {
    allocator: A,
    drivers: Drivers,
}

impl<A, Drivers> CompositeKernel<A, Drivers> {
    /// Creates a new `CompositeKernel` with the given `allocator` and `drivers` chain.
    #[inline]
    pub const fn new(allocator: A, drivers: Drivers) -> Self {
        Self { allocator, drivers }
    }

    /// Accesses the underlying allocator directly.
    #[inline]
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Accesses the drivers payload directly.
    #[inline]
    pub fn drivers(&self) -> &Drivers {
        &self.drivers
    }
}

impl<A, Drivers> Kernel for CompositeKernel<A, Drivers> {}

impl<A: Allocator + Sized, Drivers> HasAllocator for CompositeKernel<A, Drivers> {
    type Alloc<'a>
        = &'a A
    where
        Self: 'a;

    #[inline]
    fn get_allocator<'a>(&'a self) -> Self::Alloc<'a> {
        &self.allocator
    }
}

// -----------------------------------------------------------------------------
// Snek-style Pure Trait-Based HasDriver Implementation for ANY Driver Chain!
// -----------------------------------------------------------------------------

impl<A, Drivers, D> HasDriver<D> for CompositeKernel<A, Drivers>
where
    D: Driver + 'static,
    Drivers: DriverChain,
{
    type DriverRef<'a>
        = &'a D
    where
        Self: 'a,
        D: 'a;

    #[inline]
    fn get_driver<'a>(&'a self) -> Self::DriverRef<'a> {
        self.drivers
            .query_driver::<D>()
            .expect("Driver capability requested by Task was not attached to Kernel")
    }
}

/// Fluent builder for constructing a [`CompositeKernel`] without manual trait boilerplate or macros.
#[derive(Debug, Clone, Copy, Default)]
pub struct KernelBuilder<A = (), Drivers = NilDrivers> {
    allocator: A,
    drivers: Drivers,
}

impl KernelBuilder<(), NilDrivers> {
    /// Creates a new, empty `KernelBuilder`.
    #[inline]
    pub const fn new() -> Self {
        Self {
            allocator: (),
            drivers: NilDrivers,
        }
    }
}

impl<A, Drivers> KernelBuilder<A, Drivers> {
    /// Sets or replaces the allocator capability for the kernel.
    #[inline]
    pub fn with_allocator<NewA: Allocator>(self, allocator: NewA) -> KernelBuilder<NewA, Drivers> {
        KernelBuilder {
            allocator,
            drivers: self.drivers,
        }
    }

    /// Appends a new driver capability to the kernel chain.
    #[inline]
    pub fn with_driver<D: Driver>(self, driver: D) -> KernelBuilder<A, DriverNode<D, Drivers>> {
        KernelBuilder {
            allocator: self.allocator,
            drivers: DriverNode {
                driver,
                rest: self.drivers,
            },
        }
    }

    /// Builds and returns the final [`CompositeKernel`].
    #[inline]
    pub fn build(self) -> CompositeKernel<A, Drivers> {
        CompositeKernel::new(self.allocator, self.drivers)
    }
}
