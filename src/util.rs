use std::fmt::{Debug, Formatter};

/// Encapsulating type that provides ownership to its contained value without needing ownership to
/// the `ReplaceCell`, provided a replacement value is given.
///
/// see: [replace](ReplaceCell::replace)
pub struct ReplaceCell<T>(Option<T>);

impl<T> ReplaceCell<T> {
    pub fn new(value: T) -> ReplaceCell<T> {
        ReplaceCell(Some(value))
    }

    /// Provide access to the contained value through moving out the old value and replacing it with
    /// a new value
    pub fn replace(&mut self, f: impl FnOnce(T) -> T) {
        self.0 = Some(f(self.0.take().unwrap()))
    }

    /// Consume the object, returning the contained value
    pub fn into_inner(self) -> T {
        self.0.unwrap()
    }

    pub fn get_ref(&self) -> &T {
        self.0.as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.0.as_mut().unwrap()
    }
}

impl<T: Copy> Copy for ReplaceCell<T> {}

impl<T: Clone> Clone for ReplaceCell<T> {
    fn clone(&self) -> Self { ReplaceCell(self.0.clone()) }
}

impl<T: Eq> Eq for ReplaceCell<T> {}

impl<T: PartialEq> PartialEq for ReplaceCell<T> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl<T: Debug> Debug for ReplaceCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}
