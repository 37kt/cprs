pub struct Arena {
    pool: Vec<u8>,
    offset: usize,
}

impl Arena {
    #[allow(clippy::uninit_vec)]
    pub fn new(capacity: usize) -> Self {
        let mut pool = Vec::with_capacity(capacity);
        unsafe { pool.set_len(capacity) };
        Self { pool, offset: 0 }
    }

    pub fn alloc<T>(&mut self, value: T) -> *mut T {
        let align = std::mem::align_of::<T>();
        let size = std::mem::size_of::<T>();

        let pool_addr = self.pool.as_ptr() as usize;
        let start = (pool_addr + self.offset + align - 1) & !(align - 1);
        let end = start + size;
        assert!(
            end <= pool_addr + self.pool.len(),
            "simple_arena: out of memory"
        );

        let ptr = start as *mut T;
        unsafe {
            std::ptr::write(ptr, value);
        }
        self.offset = end - pool_addr;
        ptr
    }
}
