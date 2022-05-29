#![no_std]

#[no_mangle]
pub fn hello_from_rust_wo_std() -> *const u8 {
    b"Hello world, from rust!\0".as_ptr()
}


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
