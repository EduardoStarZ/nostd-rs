#![no_std]
#![no_main]

use core::arch::asm;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let msg : &str = "Hello world!\n";

    println(msg);

    exit();
}

pub fn exit() -> ! {
    unsafe{ 
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") 0,
            options(noreturn)
        );
    }
}

pub fn println(st: &str) {
   let buffer_ptr = st.as_ptr();
   let len : usize = st.len();

    unsafe {
        asm!(
            "syscall",
            in("rax") 1,
            in("rdi") 1,
            in("rsi") buffer_ptr,
            in("rdx") len,
            out("rcx") _,
            out("r11") _,
        );
   } 
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { 
    loop {}
}
