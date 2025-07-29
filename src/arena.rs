const ARENA_REGION_DEFAULT_CAP: usize = 8 * 1024;

pub struct Arena<'prog> {
    begin: *mut Region,
    end: *mut Region,

    count: usize,

    _lifetime: core::marker::PhantomData<&'prog str>,
}

// TODO: make it so we can store `Drop` items
/// A region of an arena
///
/// # Important
/// This region can only hold `!Drop` items
struct Region {
    next: *mut Region,
    count: usize,
    capacity: usize,
    data: *mut u8,
}

impl<'prog> Arena<'prog> {
    pub fn new() -> Self {
        Self {
            begin: core::ptr::null_mut(),
            end: core::ptr::null_mut(),

            count: 0,
            _lifetime: core::marker::PhantomData,
        }
    }

    pub fn strdup(&self, str: &str) -> &'prog str {
        unsafe {
            let chunk = self.arena_alloc(str.len());
            core::ptr::copy_nonoverlapping(str.as_ptr(), chunk, str.len());
            let slice = core::slice::from_raw_parts(chunk, str.len());
            core::str::from_utf8_unchecked(slice)
        }
    }

    unsafe fn arena_alloc(&self, size_bytes: usize) -> *mut u8 {
        let self_mut = self as *const _ as *mut Arena;

        if self.end.is_null() {
            assert!(self.begin.is_null());
            let cap = if ARENA_REGION_DEFAULT_CAP < size_bytes {
                size_bytes
            } else {
                ARENA_REGION_DEFAULT_CAP
            };

            (*self_mut).end = Region::new(cap);
            (*self_mut).begin = (*self_mut).end;
            (*self_mut).count += 1;
        }

        while (*self.end).count + size_bytes > (*self.end).capacity && !(*self.end).next.is_null() {
            (*self_mut).end = (*(*self_mut).end).next;
        }

        if (*self.end).count + size_bytes > (*self.end).capacity {
            assert!((*self.end).next.is_null());
            let cap = if ARENA_REGION_DEFAULT_CAP < size_bytes {
                size_bytes
            } else {
                ARENA_REGION_DEFAULT_CAP
            };

            (*self.end).next = Region::new(cap);
            (*self_mut).end = (*(*self_mut).end).next;
            (*self_mut).count += 1;
        }

        let result = (*self.end).data.add((*self.end).count);
        (*self.end).count += size_bytes;
        result
    }
}

impl<'prog> Drop for Arena<'prog> {
    fn drop(&mut self) {
        unsafe {
            while !(*self.end).next.is_null() {
                let next = (*self.end).next;
                let _ = Box::from_raw(self.end);
                self.end = next;
            }
        }
    }
}

impl Region {
    pub unsafe fn new(capacity: usize) -> *mut Self {
        let size_bytes = core::mem::size_of::<*const ()>() * capacity;
        let layout =
            core::alloc::Layout::array::<*const ()>(size_bytes).expect("cannot allocate memory");

        assert!(layout.size() <= isize::MAX as usize, "allocation too large");
        let data = std::alloc::alloc(layout);

        let region = Self {
            next: core::ptr::null_mut(),
            count: 0,
            capacity,
            data,
        };

        // Trick to return a ptr to the region allocated on the stack
        Box::into_raw(Box::new(region))
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        let size_bytes = core::mem::size_of::<*const ()>() * self.capacity;
        let layout =
            core::alloc::Layout::array::<*const ()>(size_bytes).expect("unable to create layout");
        unsafe {
            std::alloc::dealloc(self.data, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strdup() {
        let arena = Arena::new();
        let str_1 = arena.strdup("Hello World!");
        let str_2 = arena.strdup("Hello World!");

        assert_eq!(str_1, "Hello World!");
        assert_eq!(str_2, "Hello World!");
        assert_eq!(arena.count, 1);
    }
}
