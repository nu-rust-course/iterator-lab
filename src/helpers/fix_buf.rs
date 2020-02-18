use std::convert::{AsMut, AsRef};
use std::marker::PhantomData;
use std::mem;

use super::as_slice::{AsMutSlice, AsSlice};

#[derive(Debug, Default)]
pub struct FixBuf<A, E = char> {
    array: A,
    len: usize,
    _marker: PhantomData<E>,
}

impl<A, E> FixBuf<A, E> {
    pub fn len(&self) -> usize {
        self.len
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<A, E> FixBuf<A, E>
where
    A: Default + AsMutSlice<E>,
{
    pub fn new<I>(elems: I) -> Self
    where
        I: IntoIterator<Item = E>,
        I::IntoIter: DoubleEndedIterator,
    {
        let mut array = A::default();
        let mut len = 0;

        let dsts = array.as_mut_slice().iter_mut().rev();
        let srcs = elems.into_iter().rev();

        for (dst, src) in dsts.zip(srcs) {
            *dst = src;
            len += 1;
        }

        FixBuf {
            array,
            len,
            _marker: PhantomData,
        }
    }
}

impl<A, E> FixBuf<A, E>
where
    A: AsSlice<E>,
{
    fn front_index(&self) -> usize {
        self.array.as_slice().len() - self.len()
    }
}

impl<A, E> AsRef<[E]> for FixBuf<A, E>
where
    A: AsSlice<E>,
{
    fn as_ref(&self) -> &[E] {
        self.array.as_slice()
    }
}

impl<A, E> FixBuf<A, E>
where
    A: AsMutSlice<E>,
{
    fn front_ref_mut(&mut self) -> &mut E {
        let front_index = self.front_index();
        &mut self.array.as_mut_slice()[front_index]
    }
}

impl<A, E> AsMut<[E]> for FixBuf<A, E>
where
    A: AsMutSlice<E>,
{
    fn as_mut(&mut self) -> &mut [E] {
        let front_index = self.front_index();
        &mut self.array.as_mut()[front_index..]
    }
}

impl<A, E> Iterator for FixBuf<A, E>
where
    A: AsMutSlice<E>,
    E: Default,
{
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        self.len.checked_sub(1).map(|new_len| {
            let item = mem::replace(self.front_ref_mut(), E::default());
            self.len = new_len;
            item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FixBuf;

    #[test]
    fn room_for_four_chars() {
        let mut buf1: FixBuf<[char; 4]> = FixBuf::default();
        assert_eq!(buf1.next(), None);

        let mut buf2: FixBuf<[char; 4]> = FixBuf::new("foob".chars());
        assert_eq!(buf2.next(), Some('f'));
        assert_eq!(buf2.next(), Some('o'));
        assert_eq!(buf2.next(), Some('o'));
        assert_eq!(buf2.next(), Some('b'));
        assert_eq!(buf2.next(), None);

        let mut buf3: FixBuf<[char; 4]> = FixBuf::new("fo".chars());
        assert_eq!(buf3.next(), Some('f'));
        assert_eq!(buf3.next(), Some('o'));
        assert_eq!(buf3.next(), None);
    }
}
