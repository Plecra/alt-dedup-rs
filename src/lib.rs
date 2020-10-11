use core::ptr;

pub trait NewDedup<T> {
    fn new_dedup(&mut self)
    where
        T: PartialEq,
    {
        self.new_dedup_by(|a, b| a == b);
    }
    fn new_dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool;
    fn new_dedup_by_key<F, K>(&mut self, mut key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.new_dedup_by(|x, y| key(x) == key(y))
    }
}
impl<T> NewDedup<T> for Vec<T> {
    fn new_dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        // TODO: Verify and explain safety
        struct Guard<'a, T> {
            vec: &'a mut Vec<T>,
            last_unique: *mut T,
            i: *mut T,
            end: *mut T,
        }
        impl<T> Drop for Guard<'_, T> {
            fn drop(&mut self) {
                unsafe {
                    let tail = (self.end as usize - self.i as usize) / core::mem::size_of::<T>();
                    let start_of_tail = self.last_unique.add(1);
                    let new_len = tail
                        + ((start_of_tail as usize - self.vec.as_mut_ptr() as usize)
                            / core::mem::size_of::<T>());
                    self.vec.set_len(new_len);
                    if start_of_tail != self.i {
                        ptr::copy(self.i, self.last_unique, tail);
                    }
                }
            }
        }
        if self.len() == 0 {
            return;
        }
        let mut i = unsafe { self.as_mut_ptr().add(1) };
        let end = unsafe { self.as_mut_ptr().add(self.len()) };
        while i < end {
            let last = unsafe { i.sub(1) };
            if same_bucket(unsafe { &mut *i }, unsafe { &mut *last }) {
                let mut guard = Guard {
                    last_unique: last,
                    i,
                    end,
                    vec: self,
                };
                loop {
                    let dupe = guard.i;
                    guard.i = unsafe { guard.i.add(1) };
                    unsafe {
                        ptr::drop_in_place(dupe);
                    }
                    loop {
                        if guard.i >= end {
                            unsafe {
                                guard.vec.set_len(
                                    (guard.last_unique.add(1) as usize
                                        - guard.vec.as_ptr() as usize)
                                        / core::mem::size_of::<T>(),
                                );
                            }
                            core::mem::forget(guard);
                            return;
                        }
                        if same_bucket(unsafe { &mut *guard.i }, unsafe { &mut *guard.last_unique })
                        {
                            break;
                        } else {
                            guard.last_unique = unsafe { guard.last_unique.add(1) };
                            unsafe {
                                ptr::copy_nonoverlapping(guard.i, guard.last_unique, 1);
                            }
                            guard.i = unsafe { guard.i.add(1) };
                        }
                    }
                }
            }
            i = unsafe { i.add(1) };
        }
    }
}
