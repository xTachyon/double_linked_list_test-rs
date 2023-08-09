use std::{
    alloc::{GlobalAlloc, Layout},
    cell::Cell,
};

const GB: usize = 1024 * 1024 * 1024;
const TO_ALLOC: usize = GB + GB * 8 / 10;

pub struct MyAlloc {
    buffer: [u8; TO_ALLOC],
    offset: Cell<usize>,
}
unsafe impl Send for MyAlloc {}
unsafe impl Sync for MyAlloc {}

impl MyAlloc {
    pub const fn new() -> MyAlloc {
        MyAlloc {
            buffer: [0; TO_ALLOC],
            offset: Cell::new(0),
        }
    }
}

unsafe impl GlobalAlloc for MyAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        const ALIGNMENT: usize = 16;
        assert!(layout.align() <= ALIGNMENT);

        let size = (layout.size() + (ALIGNMENT - 1)) & (0usize.wrapping_sub(ALIGNMENT));
        let offset = self.offset.get();

        assert!(size + offset <= self.buffer.len());
        let ptr = self.buffer.as_ptr().add(offset) as *mut _;
        self.offset.set(offset + size);
        ptr
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}
