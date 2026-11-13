// Memory section of an address: TEXT, RODATA, DATA, BSS, STACK and HEAP
// (see `examples/locate_pointer.rs`).
// - Linux: the mapping of /proc/self/maps that holds the address, refined by
//   linker-defined symbols (`etext`, `__bss_start`, `end`).
// - macOS: Mach-O segments/sections of the loaded images, then `mach_vm_region`
//   (via `mach2`). In debug builds, each answer is cross-checked against a second,
//   independent detection (`macos_cross_check`), kept as a point of comparison.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemorySection {
    Text,   // Executable code segment
    Rodata, // Static read-only data
    Data,   // Static initialized (writable) data
    Bss,    // Static uninitialized data
    Heap,   // Heap
    Stack,  // Stack
    Mapped, // Other mapped segments
}

/// Identifies the memory section where `obj_ref` resides.
pub fn identify_memory_section<T>(obj_ref: &T) -> Option<MemorySection> {
    #[cfg(target_os = "linux")]
    {
        identify_memory_section_linux(obj_ref)
    }

    #[cfg(target_os = "macos")]
    {
        let section = identify_memory_section_macos(obj_ref);
        #[cfg(debug_assertions)]
        macos_cross_check::compare(obj_ref as *const T as usize, section);
        section
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        eprintln!("Not supported on this OS in this example.");
        None
    }
}

// ---------------------------------
// Linux Implementation
// ---------------------------------
#[cfg(target_os = "linux")]
fn identify_memory_section_linux<T>(obj_ref: &T) -> Option<MemorySection> {
    unsafe extern "C" {
        static etext: u8; // end of the code
        static __bss_start: u8; // start of .bss
        static end: u8; // end of .bss
    }
    let addr = obj_ref as *const T as usize;

    // .bss may spill over an anonymous mapping: rely on the linker bounds first
    if (&raw const __bss_start as usize..&raw const end as usize).contains(&addr) {
        return Some(MemorySection::Bss);
    }

    let maps = std::fs::read_to_string("/proc/self/maps").ok()?;
    let exe = std::fs::read_link("/proc/self/exe").ok()?;
    for line in maps.lines() {
        // e.g. "aaaae4000000-aaaae403f000 r-xp 00000000 00:2a 123 /path/to/exe"
        let fields: Vec<&str> = line.split_whitespace().collect();
        let (start, stop) = fields.first()?.split_once('-')?;
        let start = usize::from_str_radix(start, 16).ok()?;
        let stop = usize::from_str_radix(stop, 16).ok()?;
        if !(start..stop).contains(&addr) {
            continue;
        }
        let perms = *fields.get(1)?;
        let path = fields.get(5..).map(|p| p.join(" ")).unwrap_or_default();
        return Some(match path.as_str() {
            "[heap]" => MemorySection::Heap,
            p if p.starts_with("[stack") => MemorySection::Stack,
            p if exe.as_os_str() == p => {
                // .rodata may share the executable mapping of .text (GNU ld): use `etext`
                if perms.contains('x') && addr < &raw const etext as usize {
                    MemorySection::Text
                } else if !perms.contains('w') {
                    MemorySection::Rodata
                } else {
                    MemorySection::Data
                }
            }
            _ => MemorySection::Mapped, // shared libraries, anonymous mappings...
        });
    }
    None
}

// ---------------------------------
// macOS Implementation
// ---------------------------------
#[cfg(target_os = "macos")]
pub fn identify_memory_section_macos<T>(obj_ref: &T) -> Option<MemorySection> {
    use std::mem;
    // Imports from mach2
    use mach2::kern_return::KERN_SUCCESS;
    use mach2::mach_types::task_t;
    use mach2::message::mach_msg_type_number_t;
    use mach2::port::mach_port_t;
    use mach2::traps::mach_task_self;
    use mach2::vm::mach_vm_region;
    use mach2::vm_region::VM_REGION_EXTENDED_INFO;
    use mach2::vm_types::{mach_vm_address_t, mach_vm_size_t};

    // Get the address of the object
    let addr = obj_ref as *const T as usize;
    let task: task_t = unsafe { mach_task_self() };
    if task == 0 {
        return None;
    }

    // First, try to see if the address is within a Mach-O segment of a loaded image.
    if let Some(section) = check_macho_segments(addr) {
        return Some(section);
    }

    // Otherwise, use mach_vm_region to obtain info about the memory region.
    let mut query_addr: mach_vm_address_t = addr as mach_vm_address_t;
    let mut region_size: mach_vm_size_t = 0;
    // Create the info structure using the appropriate type.
    let mut info: mach2::vm_region::vm_region_extended_info_data_t = unsafe { mem::zeroed() };
    let mut info_count: mach_msg_type_number_t = (mem::size_of::<
        mach2::vm_region::vm_region_extended_info_data_t,
    >() / mem::size_of::<i32>())
        as mach_msg_type_number_t;
    let mut object_name: mach_port_t = 0;

    let kr = unsafe {
        mach_vm_region(
            task,
            &mut query_addr,
            &mut region_size,
            VM_REGION_EXTENDED_INFO,
            (&mut info as *mut _) as *mut _,
            &mut info_count,
            &mut object_name,
        )
    };

    // `mach_vm_region` returns the first region at or *after* the address:
    // if it starts beyond, the address is not mapped at all.
    if kr != KERN_SUCCESS || query_addr as usize > addr {
        return None;
    }

    // Deduce the region type based on the info fields.
    // Tags from <mach/vm_statistics.h>
    const VM_MEMORY_STACK: u32 = 30;
    if info.user_tag == VM_MEMORY_STACK {
        return Some(MemorySection::Stack);
    }
    // Define the shared mode constant.
    const SM_SHARED: u32 = 3;
    // If the region is writable and not marked as shared, assume it's heap.
    if (info.protection & 0x2 != 0) && (u32::from(info.share_mode) != SM_SHARED) {
        return Some(MemorySection::Heap);
    }
    Some(MemorySection::Mapped)
}

// ---------------------------------
// macOS cross-check (debug builds only, not part of the API)
// ---------------------------------
// A second detection, written by hand without `mach2`: section bounds defined by
// the linker (ld64 `section$start$SEG$SECT` symbols), stack of the current thread,
// malloc zones. Narrower (4 named sections, current thread only) but independent:
// whenever it gives an answer, the `mach2` detection must agree.
#[cfg(target_os = "macos")]
mod macos_cross_check {
    use super::MemorySection;

    unsafe extern "C" {
        #[link_name = "\x01section$start$__TEXT$__text"]
        static TEXT_START: u8;
        #[link_name = "\x01section$end$__TEXT$__text"]
        static TEXT_END: u8;
        #[link_name = "\x01section$start$__TEXT$__const"]
        static RODATA_START: u8;
        #[link_name = "\x01section$end$__TEXT$__const"]
        static RODATA_END: u8;
        #[link_name = "\x01section$start$__DATA$__data"]
        static DATA_START: u8;
        #[link_name = "\x01section$end$__DATA$__data"]
        static DATA_END: u8;
        #[link_name = "\x01section$start$__DATA$__bss"]
        static BSS_START: u8;
        #[link_name = "\x01section$end$__DATA$__bss"]
        static BSS_END: u8;
        fn pthread_self() -> *mut u8;
        fn pthread_get_stackaddr_np(thread: *mut u8) -> *const u8; // top of the stack
        fn pthread_get_stacksize_np(thread: *mut u8) -> usize;
        fn malloc_zone_from_ptr(ptr: *const u8) -> *const u8; // null if not from malloc
    }

    /// `None` when this narrower detection cannot tell.
    pub(super) fn by_linker_symbols(addr: usize) -> Option<MemorySection> {
        let within =
            |start: *const u8, end: *const u8| (start as usize..end as usize).contains(&addr);
        let (stack_top, stack_size) = unsafe {
            let thread = pthread_self();
            (
                pthread_get_stackaddr_np(thread),
                pthread_get_stacksize_np(thread),
            )
        };
        if within(&raw const TEXT_START, &raw const TEXT_END) {
            Some(MemorySection::Text)
        } else if within(&raw const RODATA_START, &raw const RODATA_END) {
            Some(MemorySection::Rodata)
        } else if within(&raw const DATA_START, &raw const DATA_END) {
            Some(MemorySection::Data)
        } else if within(&raw const BSS_START, &raw const BSS_END) {
            Some(MemorySection::Bss)
        } else if within(stack_top.wrapping_sub(stack_size), stack_top) {
            Some(MemorySection::Stack)
        } else if !unsafe { malloc_zone_from_ptr(addr as *const u8) }.is_null() {
            Some(MemorySection::Heap)
        } else {
            None
        }
    }

    pub(super) fn compare(addr: usize, found: Option<MemorySection>) {
        if let Some(expected) = by_linker_symbols(addr) {
            debug_assert_eq!(
                found,
                Some(expected),
                "memory section of {addr:#x}: mach2 and linker-symbol detections disagree"
            );
        }
    }
}

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn _dyld_get_image_header(image_index: u32) -> *const mach_o::header::mach_header_64;
    fn _dyld_get_image_vmaddr_slide(image_index: u32) -> isize;
    fn _dyld_image_count() -> u32;
}

#[cfg(target_os = "macos")]
fn check_macho_segments(addr: usize) -> Option<MemorySection> {
    // Use dyld APIs to iterate over loaded images.
    let image_count = unsafe { _dyld_image_count() };
    for i in 0..image_count {
        let header_ptr = unsafe { _dyld_get_image_header(i) };
        if header_ptr.is_null() {
            continue;
        }
        let slide = unsafe { _dyld_get_image_vmaddr_slide(i) } as usize;
        // Interpret the header as a mach_header_64
        #[allow(clippy::unnecessary_cast)]
        let header = unsafe { &*(header_ptr as *const mach_o::header::mach_header_64) };
        if let Some(section) = match_address_in_mach_header(header, slide, addr) {
            return Some(section);
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn match_address_in_mach_header(
    header: &mach_o::header::mach_header_64,
    slide: usize,
    addr: usize,
) -> Option<MemorySection> {
    use mach_o::load_command::*;
    use mach_o::segments_command::*;

    let ncmds = header.ncmds;
    // The first load_command immediately follows the header.
    let first_lc =
        unsafe { (header as *const mach_o::header::mach_header_64).add(1) as *const load_command };

    let mut cmd = first_lc;
    for _ in 0..ncmds {
        let lc = unsafe { &*cmd };
        if lc.cmd == LC_SEGMENT_64 {
            let seg_cmd = unsafe { &*(cmd as *const segment_command_64) };
            let seg_start = seg_cmd.vmaddr as usize + slide;
            let seg_end = seg_start + seg_cmd.vmsize as usize;
            if addr >= seg_start && addr < seg_end {
                // Read-only segment: `__TEXT` (r-x) or `__DATA_CONST` (made read-only
                // by dyld after relocation, flagged SG_READ_ONLY).
                let seg_read_only =
                    seg_cmd.initprot & VM_PROT_WRITE == 0 || seg_cmd.flags & SG_READ_ONLY != 0;
                // Refine using the sections (`__text`, `__const`, `__data`, `__bss`...):
                // they immediately follow their segment_command_64.
                let sections = unsafe {
                    std::slice::from_raw_parts(
                        (seg_cmd as *const segment_command_64).add(1) as *const section_64,
                        seg_cmd.nsects as usize,
                    )
                };
                for sect in sections {
                    let sect_start = sect.addr as usize + slide;
                    let sect_end = sect_start + sect.size as usize;
                    if addr >= sect_start && addr < sect_end {
                        return Some(if sect.flags & S_ATTR_INSTRUCTIONS != 0 {
                            MemorySection::Text
                        } else if matches!(
                            sect.flags & SECTION_TYPE,
                            S_ZEROFILL | S_GB_ZEROFILL | S_THREAD_LOCAL_ZEROFILL
                        ) {
                            MemorySection::Bss
                        } else if seg_read_only {
                            MemorySection::Rodata // e.g. `__TEXT,__const`
                        } else {
                            MemorySection::Data // e.g. `__DATA,__data`
                        });
                    }
                }
                // Not inside a section (padding): fall back to the segment name
                let segname_str = segment_name_to_str(&seg_cmd.segname);
                if segname_str.starts_with("__TEXT") {
                    return Some(MemorySection::Text);
                } else if segname_str.starts_with("__DATA_CONST") {
                    return Some(MemorySection::Rodata);
                } else if segname_str.starts_with("__DATA") {
                    return Some(MemorySection::Data);
                }
            }
        }
        // Advance to the next load_command using cmdsize.
        let cmd_size = lc.cmdsize as usize;
        unsafe {
            let raw_ptr = cmd as *const u8;
            cmd = raw_ptr.add(cmd_size) as *const load_command;
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn segment_name_to_str(segname: &[i8; 16]) -> String {
    segname
        .iter()
        .map(|&c| c as u8)
        .take_while(|&c| c != 0)
        .map(|c| c as char)
        .collect()
}

#[cfg(target_os = "macos")]
mod mach_o {
    // Minimal definitions to parse the Mach-O header and load commands.
    pub mod header {
        #[repr(C)]
        pub struct mach_header_64 {
            pub magic: u32,
            pub cputype: i32,
            pub cpusubtype: i32,
            pub filetype: u32,
            pub ncmds: u32,
            pub sizeofcmds: u32,
            pub flags: u32,
            pub reserved: u32,
        }
    }

    pub mod load_command {
        #[repr(C)]
        pub struct load_command {
            pub cmd: u32,
            pub cmdsize: u32,
        }
    }

    pub mod segments_command {
        #[repr(C)]
        pub struct segment_command_64 {
            pub cmd: u32,
            pub cmdsize: u32,
            pub segname: [i8; 16],
            pub vmaddr: u64,
            pub vmsize: u64,
            pub fileoff: u64,
            pub filesize: u64,
            pub maxprot: i32,
            pub initprot: i32,
            pub nsects: u32,
            pub flags: u32,
        }
        pub const LC_SEGMENT_64: u32 = 0x19;
        pub const VM_PROT_WRITE: i32 = 0x2;
        pub const SG_READ_ONLY: u32 = 0x10;

        #[repr(C)]
        pub struct section_64 {
            pub sectname: [i8; 16],
            pub segname: [i8; 16],
            pub addr: u64,
            pub size: u64,
            pub offset: u32,
            pub align: u32,
            pub reloff: u32,
            pub nreloc: u32,
            pub flags: u32,
            pub reserved1: u32,
            pub reserved2: u32,
            pub reserved3: u32,
        }
        // Section flags, from <mach-o/loader.h>
        pub const SECTION_TYPE: u32 = 0x0000_00ff;
        pub const S_ZEROFILL: u32 = 0x1;
        pub const S_GB_ZEROFILL: u32 = 0xc;
        pub const S_THREAD_LOCAL_ZEROFILL: u32 = 0x12;
        // S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS
        pub const S_ATTR_INSTRUCTIONS: u32 = 0x8000_0000 | 0x0000_0400;
    }
}

#[cfg(test)]
mod tests {
    use super::{MemorySection, identify_memory_section};
    use std::sync::atomic::AtomicI32;

    static PROBE_RODATA: i32 = 42;
    static PROBE_DATA: AtomicI32 = AtomicI32::new(42);
    static PROBE_BSS: AtomicI32 = AtomicI32::new(0); // zero-initialized

    // On macOS (debug), each call also runs the hidden cross-check
    #[test]
    fn memory_sections() {
        let local = 10;
        let boxed = Box::new(5);
        let main_ptr = memory_sections as fn() as *const u8;
        assert_eq!(
            identify_memory_section(unsafe { &*main_ptr }),
            Some(MemorySection::Text)
        );
        assert_eq!(
            identify_memory_section(&PROBE_RODATA),
            Some(MemorySection::Rodata)
        );
        assert_eq!(
            identify_memory_section(&PROBE_DATA),
            Some(MemorySection::Data)
        );
        assert_eq!(
            identify_memory_section(&PROBE_BSS),
            Some(MemorySection::Bss)
        );
        // The test harness runs this on a secondary thread. On Linux, only the main
        // thread has `[stack]` / `[heap]` in /proc/self/maps: the stack of this thread,
        // and often its malloc arena, are anonymous mappings.
        let expected = |main_thread| {
            if cfg!(target_os = "linux") {
                vec![Some(main_thread), Some(MemorySection::Mapped)]
            } else {
                vec![Some(main_thread)]
            }
        };
        assert!(expected(MemorySection::Stack).contains(&identify_memory_section(&local)));
        assert!(expected(MemorySection::Heap).contains(&identify_memory_section(&*boxed)));
    }

    // The hidden macOS detection must recognise the probes, else the cross-check is vacuous
    #[cfg(target_os = "macos")]
    #[test]
    fn macos_cross_check_is_active() {
        use super::macos_cross_check::by_linker_symbols;
        let local = 10;
        let boxed = Box::new(5);
        let addr = |r: &dyn std::any::Any| r as *const dyn std::any::Any as *const u8 as usize;
        assert_eq!(
            by_linker_symbols(addr(&PROBE_RODATA)),
            Some(MemorySection::Rodata)
        );
        assert_eq!(
            by_linker_symbols(addr(&PROBE_DATA)),
            Some(MemorySection::Data)
        );
        assert_eq!(
            by_linker_symbols(addr(&PROBE_BSS)),
            Some(MemorySection::Bss)
        );
        assert_eq!(by_linker_symbols(addr(&local)), Some(MemorySection::Stack));
        assert_eq!(by_linker_symbols(addr(&*boxed)), Some(MemorySection::Heap));
    }
}
