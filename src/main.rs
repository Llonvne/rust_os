#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use rust_os::println;

/// 这个函数将在 panic 时被调用
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

// 使用 no_mangle 标记这个函数，来对它禁用名称重整
// 我们还将函数标记为 extern "C"，告诉编译器这个函数应当使用 C 语言的调用约定
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to RustOS");

    rust_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");

    loop {}
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}



