use rs::identify_memory_section;

static GLOBAL_INIT: i32 = 42; // DATA
static mut GLOBAL_UNINIT: i32 = 0; // BSS

fn main() {
    let x = 10;
    let heap_num = Box::new(5);

    println!("TEXT : => {:?}", identify_memory_section(&main)); // always 1 on Linux
    println!("DATA : {:?}", identify_memory_section(&GLOBAL_INIT));
    // `&GLOBAL_UNINIT` builds a shared &-reference to a `static mut`: rustc's
    // `static_mut_refs` lint flags this as unsound (another thread could mutate it
    // while the reference is alive). Kept on purpose (sound here: single-threaded,
    // never mutated). Lint-free alternative: `&raw const GLOBAL_UNINIT` (raw-borrow
    // operator, stable since 1.82) only takes the ADDRESS as a `*const i32`, with no
    // reference/aliasing guarantee, then `unsafe { &*ptr }` to get back a `&i32`.
    #[allow(static_mut_refs)]
    let bss_ref = unsafe { &GLOBAL_UNINIT };
    println!("BSS  : {:?}", identify_memory_section(bss_ref));
    println!("STACK: {:?}", identify_memory_section(&x));
    println!("HEAP : {:?}", identify_memory_section(&*heap_num));
}
