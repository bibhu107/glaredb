use std::marker::PhantomData;

/// Marker type that indicates covariance of `T` but does not inherit the bounds
/// of `T`.
///
/// Has all the same properties of `PhantomData` minus the inherited trait
/// bounds. This lets us make structs and other types covariant to `T` but
/// without the potential inheritence of `?Sized` (or other undesired traits) in
/// the outer type.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhantomCovariant<T>(PhantomData<fn() -> T>)
where
    T: ?Sized;

impl<T> PhantomCovariant<T>
where
    T: ?Sized,
{
    pub const fn new() -> Self {
        PhantomCovariant(PhantomData)
    }
}

impl<T> Clone for PhantomCovariant<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for PhantomCovariant<T> where T: ?Sized {}

impl<T> Default for PhantomCovariant<T>
where
    T: ?Sized,
{
    fn default() -> Self {
        Self::new()
    }
}
