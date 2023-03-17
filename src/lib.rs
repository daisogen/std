#![no_std]
#![feature(rustc_private)]

extern crate alloc;
extern crate compiler_builtins;

mod allocator;
pub mod asm;
pub mod daisogen;
pub mod panic;
pub mod print;

pub use alloc::boxed;
pub use alloc::string;
pub use alloc::vec;
pub use core::mem;

pub mod collections {
    pub use hashbrown::*;
}

extern "Rust" {
    fn main() -> u64;
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        main();
    }

    todo!("exit()");
}
