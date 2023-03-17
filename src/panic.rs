use crate::println;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO change color or something
    println!("\n{}", info);
    loop {}
}
