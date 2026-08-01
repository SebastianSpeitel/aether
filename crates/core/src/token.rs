use crate::allocator::Allocator;

/// A unified interface over both owned and raw allocator tokens.
///
/// This trait is the bridge that lets allocator methods such as [`Allocator::read`]
/// and [`Allocator::write`] accept **either** an owned token (`A::Token<T>`) or
/// a raw token (`A::RawToken<T>`) without requiring a separate generic parameter
/// at every call site.
///
/// # `RAW` const parameter
///
/// The `RAW` const bool selects which blanket impl applies:
///
/// | Concrete type    | `RAW`   | `as_raw` behaviour                         |
/// |------------------|---------|--------------------------------------------|
/// | `A::Token<T>`    | `false` | downgrades via [`Allocator::downgrade`]    |
/// | `A::RawToken<T>` | `true`  | identity — returns `*self`                 |
///
/// `RAW` defaults to `false` because owned tokens are the preferred handle; raw
/// tokens are the lower-level escape hatch.
///
/// At call sites `RAW` is always inferred, so callers never need to name it:
///
/// ```ignore
/// // Both compile — RAW is inferred as false / true respectively.
/// alloc.read(&owned_token)?;
/// alloc.read(&raw_token)?;
/// ```
///
/// # Implementing
///
/// You should **not** implement this trait manually. The two blanket impls
/// provided by this crate cover all intended use cases.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be used as a token for allocator `{A}`",
    label = "expected `{A}::Token<{T}>` (RAW=false) or `{A}::RawToken<{T}>` (RAW=true)",
    note = "only the allocator's own `Token<T>` and `RawToken<T>` types implement this trait"
)]
pub trait Token<T: ?Sized, A: Allocator + ?Sized, const RAW: bool = false> {
    /// Converts this token into the allocator's raw token representation.
    ///
    /// For `RAW = true` this is a no-op copy; for `RAW = false` this calls
    /// [`Allocator::downgrade`] without consuming or invalidating the owned token.
    fn as_raw(&self, alloc: &A) -> A::RawToken<T>;
}

/// `RAW = true` impl — raw tokens are already in raw form, so `as_raw` is the identity.
impl<T: ?Sized, A: Allocator> Token<T, A, true> for A::RawToken<T> {
    #[inline]
    fn as_raw(&self, _alloc: &A) -> A::RawToken<T> {
        *self
    }
}

/// `RAW = false` impl — owned tokens downgrade to raw via [`Allocator::downgrade`].
///
/// The `A: Allocator<Token<T> = O>` bound ensures this impl only activates
/// when `O` is exactly the allocator's owned token type for `T`, preventing
/// accidental implementations on unrelated types.
impl<T: ?Sized, A: Allocator, O> Token<T, A, false> for O
where
    A: Allocator<Token<T> = O>,
{
    #[inline]
    fn as_raw(&self, alloc: &A) -> A::RawToken<T> {
        alloc.downgrade(self)
    }
}

/// Extension for promoting a `MaybeUninit` raw token to its initialized form.
///
/// # Safety contract
///
/// The caller is responsible for ensuring the memory behind the token has been
/// fully initialized before calling [`assume_init`](TokenAssumeInit::assume_init).
/// Calling it on uninitialized memory is **undefined behaviour**.
pub trait TokenAssumeInit<U, A: Allocator + ?Sized> {
    /// Reinterprets a `RawToken<MaybeUninit<U>>` as a `RawToken<U>`.
    ///
    /// # Safety
    /// The memory referred to by `self` must be fully initialized as a valid `U`.
    unsafe fn assume_init(self, alloc: &A) -> A::RawToken<U>;
}

impl<U, A: Allocator + ?Sized> TokenAssumeInit<U, A> for A::RawToken<core::mem::MaybeUninit<U>> {
    #[inline]
    unsafe fn assume_init(self, alloc: &A) -> A::RawToken<U> {
        // SAFETY: delegated to the caller via the trait's safety contract.
        unsafe { alloc.cast(self) }
    }
}
