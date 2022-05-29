use std::intrinsics::const_allocate;
use std::marker::PhantomData;
use std::ops;

pub(crate) struct ConstVec<T> {
    data: *mut u8,
    len: usize,
    capacity: usize,
    _phantom: PhantomData<T>,
}

impl<T> ConstVec<T> {
    pub(crate) const fn new() -> Self {
        ConstVec {
            data: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
            _phantom: PhantomData,
        }
    }

    pub(crate) const fn len(&self) -> usize {
        self.len
    }

    const unsafe fn ensure_has_capacity(&mut self) {
        if self.len == self.capacity {
            if self.capacity == 0 {
                self.capacity = 1;
                self.data = const_allocate(
                    self.capacity * std::mem::size_of::<T>(),
                    std::mem::align_of::<T>(),
                );
            } else {
                self.capacity *= 2;
                let new_alloc = const_allocate(
                    self.capacity * std::mem::size_of::<T>(),
                    std::mem::align_of::<T>(),
                );

                // Copy items from the old location
                let mut offset = 0;
                while offset < self.len {
                    new_alloc
                        .cast::<T>()
                        .offset(offset as isize)
                        .write(self.data
                            .cast::<T>()
                            .offset(offset as isize)
                            .read());
                    offset += 1;
                }
                self.data = new_alloc;
            }
        }
    }

    pub(crate) const fn push(&mut self, value: T) {
        unsafe {
            self.ensure_has_capacity();
            self.data.cast::<T>().offset(self.len as isize).write(value);
            self.len += 1;
        }
    }

    pub(crate) const fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            Some(unsafe { self.data.cast::<T>().offset(self.len as isize).read() })
        }
    }

    pub(crate) const fn as_slice(&self) -> &'static [T] {
        if self.len == 0 {
            &[]
        } else {
            unsafe {
                &*std::ptr::slice_from_raw_parts::<T>(self.data.cast::<T>(), self.len)
            }
        }
    }
}

impl<T> const ops::Index<usize> for ConstVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        unsafe { &*self.data.cast::<T>().offset(index as isize) }
    }
}

impl<T> const ops::IndexMut<usize> for ConstVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len);
        unsafe { &mut *self.data.cast::<T>().offset(index as isize) }
    }
}
