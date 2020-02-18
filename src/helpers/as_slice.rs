pub trait AsSlice<T>: AsRef<[T]> {
    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

pub trait AsMutSlice<T>: AsMut<[T]> + AsSlice<T> {
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

impl<T, U> AsSlice<T> for U where U: AsRef<[T]> {}

impl<T, U> AsMutSlice<T> for U where U: AsRef<[T]> + AsMut<[T]> {}
