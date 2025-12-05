use uefi::println;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	println!("panic: {_info}");
	loop {}
}
