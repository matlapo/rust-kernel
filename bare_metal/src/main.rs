#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(custom_test_frameworks)]
#![test_runner(bare_metal::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bare_metal::println;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] 
pub extern "C" fn _start() -> ! {

    println!("Hello world{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// called on panic
#[cfg(not(test))] 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	bare_metal::test_panic_handler(info)
}



