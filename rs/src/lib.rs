// WARNING: this code is Work in progress
// Even if STACK and HEAP sections seem well detected,
// TEXT, DATA, BSS are not yet well detected. Should be improved/fixed

#[derive(Debug)]
pub enum MemorySection {
    Text,   // Executable code segment
    Data,   // Static initialized data (or read-only data)
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
        identify_memory_section_macos(obj_ref)
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
    use std::fs;
    use std::path::PathBuf;

    let addr = obj_ref as *const T as usize;

    // println!("Looking for pointer: {:x}", addr);

    let maps_content = fs::read_to_string("/proc/self/maps").ok()?;
    let exe_path: Option<PathBuf> = fs::read_link("/proc/self/exe").ok();
    let exe_path_str = exe_path.as_ref().map(|p| p.to_string_lossy().into_owned());

    for line in maps_content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // println!("[Map line]: {}", line);

        let mut parts = line.split_whitespace();
        let range = parts.next()?;
        let perms = parts.next()?;
        let offset_hex = parts.next()?;
        let _dev = parts.next()?;
        let _inode = parts.next()?;
        let path = parts.collect::<Vec<_>>().join(" ");

        let mut addr_range = range.split('-');
        let start_str = addr_range.next()?;
        let end_str = addr_range.next()?;
        let start_addr = usize::from_str_radix(start_str, 16).ok()?;
        let end_addr = usize::from_str_radix(end_str, 16).ok()?;

        if addr < start_addr || addr >= end_addr {
            continue;
        }

        // We found the segment that contains the address
        if path == "[heap]" {
            return Some(MemorySection::Heap);
        } else if path.starts_with("[stack") {
            return Some(MemorySection::Stack);
        } else if path.is_empty() {
            return Some(MemorySection::Mapped);
        } else if let Some(ref exe) = exe_path_str {
            if path == *exe {
                // This segment belongs to the main executable
                if perms.contains('x') {
                    return Some(MemorySection::Text);
                } else if perms.contains('w') {
                    // This might be .data or .bss
                    if let Ok(offset) = u64::from_str_radix(offset_hex, 16) {
                        if let Ok(metadata) = fs::metadata(exe) {
                            let file_size = metadata.len();
                            let addr_u64 = addr as u64;
                            let start_u64 = start_addr as u64;
                            assert!(addr_u64 >= start_u64);
                            let offset_in_file = offset + (addr_u64 - start_u64);
                            if offset_in_file < file_size {
                                return Some(MemorySection::Data);
                            } else {
                                return Some(MemorySection::Bss);
                            }
                        }
                    }
                    return Some(MemorySection::Data);
                } else {
                    // Read-only => .rodata (treat as Data)
                    return Some(MemorySection::Data);
                }
            } else {
                unimplemented!()
            }
        }
        // Otherwise, it's likely a shared library or file mapping
        return Some(MemorySection::Mapped);
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

    if kr != KERN_SUCCESS {
        return None;
    }

    // Deduce the region type based on the info fields.
    const VM_MEMORY_STACK: u32 = 4;
    if info.user_tag == VM_MEMORY_STACK || info.user_tag == 255 {
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

#[cfg(target_os = "macos")]
extern "C" {
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
                let segname_str = segment_name_to_str(&seg_cmd.segname);
                if segname_str.starts_with("__TEXT") {
                    return Some(MemorySection::Text);
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
    }
}
