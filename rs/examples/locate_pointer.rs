use rs::identify_memory_section;

static GLOBAL_INIT: i32 = 42; // DATA
static mut GLOBAL_UNINIT: i32 = 0; // BSS

fn main() {
    let x = 10;
    let heap_num = Box::new(5);

    println!("TEXT : => {:?}", identify_memory_section(&main)); // always 1 on Linux
    println!("DATA : {:?}", identify_memory_section(&GLOBAL_INIT));
    println!(
        "BSS  : {:?}",
        identify_memory_section(unsafe { &GLOBAL_UNINIT })
    );
    println!("STACK: {:?}", identify_memory_section(&x));
    println!("HEAP : {:?}", identify_memory_section(&*heap_num));
}
