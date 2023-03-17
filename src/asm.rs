// Low level instructions

use core::arch::asm;

pub fn in32(port: u16) -> u32 {
    let ret: u32;
    unsafe {
        asm!("in eax, dx",
             in("dx") port,
             out("eax") ret,
             options(nostack, preserves_flags));
    }
    ret
}

pub fn out32(port: u16, val: u32) {
    unsafe {
        asm!("out dx, eax",
             in("dx") port,
             in("eax") val,
             options(nostack, preserves_flags));
    }
}
