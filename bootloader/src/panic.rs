use core::panic::PanicInfo;
use uefi::println;

#[panic_handler]
fn panic_handle(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
