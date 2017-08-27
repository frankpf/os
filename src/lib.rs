#![feature(lang_items)]
#![no_std]
extern crate rlibc;

fn fill_row(start: u32, row_num: u32) -> () {
    let buffer_ptr = (start + row_num*80) as *mut _;
    let color_byte = 0x1f201f20;
    let line = [color_byte; 40];
    unsafe { *buffer_ptr = line };
}

#[no_mangle]
pub extern fn rust_main() {
    /*
    let hello = b"Hello world!";
    let color_byte = 0x1f;

    let rows = 25;
    let cols = 80;

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // Write "Hello world" to the center
    // of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };
    */
    let start = 0xb8000;

    let mut column = 0;

    while column < 80 {
        fill_row(start, column);
        column += 1;
    }

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
