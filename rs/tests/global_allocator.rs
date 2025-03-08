mod details {
    // Example usage in a global allocator
    use super::*;
    use core::fmt::{self, Write as FmtWrite};
    use libc::{c_void, write};

    /// Small wrapper around a u8 slice that implements `core::fmt::Write`
    /// to write formatted text without heap allocations.
    struct BufferWriter<'a> {
        buf: &'a mut [u8],
        pos: usize,
    }

    impl<'a> BufferWriter<'a> {
        fn new(buf: &'a mut [u8]) -> Self {
            Self { buf, pos: 0 }
        }

        fn as_bytes(&self) -> &[u8] {
            &self.buf[..self.pos]
        }
    }

    impl fmt::Write for BufferWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            // Ensure we don't exceed the buffer's capacity
            if self.pos + s.len() > self.buf.len() {
                return Err(fmt::Error);
            }
            // Copy the bytes
            self.buf[self.pos..self.pos + s.len()].copy_from_slice(s.as_bytes());
            self.pos += s.len();
            Ok(())
        }
    }

    pub fn trace_alloc(context: &'static str, ptr: *const u8, layout: &Layout) {
        // We define a stack buffer of fixed size
        let mut stack_buf = [0u8; 64];
        let mut writer = BufferWriter::new(&mut stack_buf);

        // We format the message into this buffer (without allocating on the heap)
        let _ = core::write!(
            writer,
            "{context} {} bytes with {}-alignment at {ptr:p}\n",
            layout.size(),
            layout.align()
        );

        // Retrieve the used portion
        let bytes = writer.as_bytes();
        // Then write directly to file descriptor 2 (stderr) using libc
        unsafe {
            write(2, bytes.as_ptr() as *const c_void, bytes.len());
        }
    }
}

use std::alloc::{GlobalAlloc, Layout, System};

struct CustomGlobalAlloc;

unsafe impl GlobalAlloc for CustomGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = System.alloc(layout);
        details::trace_alloc("Alloc", ptr, &layout);
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        details::trace_alloc("Free", ptr, &layout);
        System.dealloc(ptr, layout)
    }
}

// To use a local allocator, we currently need nightly + #![feature(allocator_api)]
#[global_allocator]
static GLOBAL_ALLOC: CustomGlobalAlloc = CustomGlobalAlloc;

#[test]
fn main() {
    println!("Start DEMO");
    let v = vec![42u8; 217];
    println!("v data address = {:p}", v.as_ptr());
    println!("Stop DEMO");
}
