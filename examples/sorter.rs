use static_borrow_vec::{BorrowVecGuard, EmptyBorrowVec};

struct Sorter<T: 'static> {
    /// The buffer that stores references to `T`.
    ///
    /// [`Vec`] is not used because it leaks the lifetime of `T` to the owner of this struct.
    buf: EmptyBorrowVec<T>,
}

impl<T: Ord + 'static> Sorter<T> {
    #[allow(clippy::default_constructed_unit_structs)]
    pub fn new() -> Self {
        Self {
            buf: EmptyBorrowVec::new(),
        }
    }

    /// Return a reusable buffer instead of an allocated [`Vec`].
    pub fn sort<'t>(&'t mut self, t: impl Iterator<Item = &'t T>) -> BorrowVecGuard<'t, 't, T> {
        let mut buf = self.buf.get_mut();
        buf.get_mut().extend(t);
        buf.get_mut().sort_unstable();
        buf
    }
}

fn main() {
    let mut sorter = Sorter::new();
    let numbers = [3, 2, 1];
    let sorted = sorter.sort(numbers.iter());

    let numbers = [1, 2, 3].iter().collect::<Vec<_>>();
    assert_eq!(&numbers, sorted.get());
}
