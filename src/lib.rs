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

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
