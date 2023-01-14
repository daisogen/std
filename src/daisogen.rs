use crate::string::String;
use core::arch::global_asm;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref PD_CACHE: Mutex<HashMap<String, u64>> = Mutex::new(HashMap::new());
}

pub fn pd_get(name: &str) -> u64 {
    if PD_CACHE.lock().contains_key(name) {
        return *PD_CACHE.lock().get(name).unwrap();
    }

    let strptr = name.as_bytes().as_ptr() as u64;
    let sz = name.as_bytes().len();
    let ret = unsafe { jmp_pd_get(strptr, sz) };
    PD_CACHE.lock().insert(String::from(name), ret);
    ret
}

pub fn pd_set(name: &str, val: u64) {
    let strptr = name.as_bytes().as_ptr() as u64;
    let sz = name.as_bytes().len();
    unsafe {
        jmp_pd_set(strptr, sz, val);
    }
    PD_CACHE.lock().insert(String::from(name), val);
}

pub fn pd_call2(name: &str, arg1: u64, arg2: u64) -> u64 {
    unsafe { jmp2(arg1, arg2, pd_get(name)) }
}

extern "C" {
    fn jmp_pd_get(strptr: u64, sz: usize) -> u64;
    fn jmp_pd_set(stprtr: u64, sz: usize, val: u64) -> u64;
    fn jmp2(arg1: u64, arg2: u64, ptr: u64) -> u64;
}

global_asm!(
    "
jmp_pd_get:
    mov rax, 0xFFFF800000000000
    mov rax, qword ptr [rax]
    jmp rax

jmp_pd_set:
    mov rax, 0xFFFF800000000008
    mov rax, qword ptr [rax]
    jmp rax

jmp2:
    jmp rdx
"
);
