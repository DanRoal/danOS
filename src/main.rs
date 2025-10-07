//Let's create our own OS, for that we will need to 
//disable the link to the standard library


#![no_std] //this atribute lets us disable the link
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// This funcion is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // The ! return type means that the function is diverging, i.e. not allowed
    // to ever return.
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)] // with this we disable name mangling. This is needed since otherwise the compiler would generate some unique name (something like: _ZN3blog_os4_start7hb173fedf945531caE), but since we need to tell the name of the entry point functon to the linker in the next step
pub extern "C" fn _start() -> ! {
    // We mark the function as extern "C" to tell the compiler that it 
    // should use the C calling convention. We do it this way since the 
    // name "_start" is the default entry point for most systems

    use core::fmt::Write; // Necesitamos importar fmt::Write para ser capaces de usar sus funciones
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}

    // The never return (!) type is needed since the entry point is not called
    // by any function, but invoked directly by the OS or bootloader
}