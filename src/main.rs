//Let's create our own OS, for that we will need to 
//disable the link to the standard library


#![no_std] //this atribute lets us disable the link
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(danOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use danOS::{println, vga_buffer::*};

#[unsafe(no_mangle)] // with this we disable name mangling. This is needed since otherwise the compiler would generate some unique name (something like: _ZN3blog_os4_start7hb173fedf945531caE), but since we need to tell the name of the entry point functon to the linker in the next step
pub extern "C" fn _start() -> ! {
    // We mark the function as extern "C" to tell the compiler that it 
    // should use the C calling convention. We do it this way since the 
    // name "_start" is the default entry point for most systems

    println!("Hello World{}", "!");

    danOS::init();

    //invocamos una excepcion de breakpoint
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}

    // The never return (!) type is needed since the entry point is not called
    // by any function, but invoked directly by the OS or bootloader
}
// This funcion is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // The ! return type means that the function is diverging, i.e. not allowed
    // to ever return.

    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    danOS::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1,1);
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i,c) in s.chars().enumerate() {
        let screen_char = read_byte_at(BUFFER_HEIGHT -2, i);
        assert_eq!(char::from(screen_char), c)
    }
}

