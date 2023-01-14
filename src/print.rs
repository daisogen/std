#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: core::fmt::Arguments) {
    let mut output = alloc::string::String::new();
    core::fmt::write(&mut output, args).expect("formatting error");
    let strptr = output.as_bytes().as_ptr() as u64;
    let sz = output.as_bytes().len() as u64;
    crate::daisogen::pd_call2("print", strptr, sz);
}
