use core::cell::UnsafeCell;
use core::marker::PhantomData;

pub type Priority = usize;

const fn needs_lock<const CURRENT: Priority, const MAX: Priority>() -> bool {
    CURRENT < MAX
}

const fn has_atomic_read<T>() -> bool {
    #[cfg(target_arch = "avr")]
    {
        core::mem::size_of::<T>() <= 1
    }
    #[cfg(not(target_arch = "avr"))]
    {
        core::mem::size_of::<T>() <= core::mem::size_of::<usize>()
    }
}

pub struct Token<'a, const P: Priority> {
    _marker: PhantomData<&'a mut ()>,
}

impl Token<'static, 0> {
    pub const ROOT: Self = Self {
        _marker: PhantomData,
    };
}

impl<const P: Priority> Token<'_, P> {
    /// Creates a new token without checking priority invariants.
    ///
    /// # Safety
    /// The caller must guarantee that priority invariants are correctly maintained.
    #[must_use]
    pub const unsafe fn new_unchecked() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

pub struct Lock<T, const MAX: Priority> {
    data: UnsafeCell<T>,
}

unsafe impl<T: Send, const MAX: Priority> Sync for Lock<T, MAX> {}

pub struct Guard<'a, T, const CURRENT: Priority, const MAX: Priority> {
    data: &'a mut T,
    sreg: u8,
}

impl<T, const CURRENT: Priority, const MAX: Priority> core::ops::Deref
    for Guard<'_, T, CURRENT, MAX>
{
    type Target = T;
    fn deref(&self) -> &T {
        self.data
    }
}

impl<T, const CURRENT: Priority, const MAX: Priority> core::ops::DerefMut
    for Guard<'_, T, CURRENT, MAX>
{
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}

impl<T, const CURRENT: Priority, const MAX: Priority> Drop for Guard<'_, T, CURRENT, MAX> {
    fn drop(&mut self) {
        if const { !needs_lock::<CURRENT, MAX>() } {
            return;
        }

        #[cfg(target_arch = "avr")]
        unsafe {
            core::arch::asm!(
                "out 0x3F, {}",
                in(reg) self.sreg,
                options(nostack)
            );
        }
        #[cfg(not(target_arch = "avr"))]
        {
            let _ = self.sreg;
        }
    }
}

impl<T, const MAX: Priority> Lock<T, MAX> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_mut<const CURRENT: Priority>(&self, _ctx: &mut Token<'_, CURRENT>) -> &mut T {
        const {
            assert!(
                CURRENT == MAX,
                "Priority ceiling mismatch: get_mut requires current priority to equal the ceiling"
            );
        }
        unsafe { &mut *self.data.get() }
    }

    #[must_use]
    pub fn lock<'a, const CURRENT: Priority>(
        &'a self,
        _ctx: &'a mut Token<'_, CURRENT>,
    ) -> Guard<'a, T, CURRENT, MAX> {
        const {
            assert!(
                CURRENT <= MAX,
                "Priority violation: current context priority exceeds the resource maximum ceiling"
            );
        }

        if const { !needs_lock::<CURRENT, MAX>() } {
            return Guard {
                data: unsafe { &mut *self.data.get() },
                sreg: 0,
            };
        }

        #[allow(unused_mut)]
        let mut sreg = 0u8;
        #[cfg(target_arch = "avr")]
        unsafe {
            core::arch::asm!(
                "in {}, 0x3F",
                "cli",
                out(reg) sreg,
                options(nostack)
            );
        }
        #[cfg(not(target_arch = "avr"))]
        {
            let _ = sreg;
        }

        Guard {
            data: unsafe { &mut *self.data.get() },
            sreg,
        }
    }
}

impl<T: Copy + Eq, const MAX: Priority> Lock<T, MAX> {
    pub fn read(&self) -> T {
        unsafe {
            if const { has_atomic_read::<T>() } {
                return core::ptr::read_volatile(self.data.get());
            }

            loop {
                let first = core::ptr::read_volatile(self.data.get());
                let second = core::ptr::read_volatile(self.data.get());
                if first == second {
                    return first;
                }
            }
        }
    }
}
