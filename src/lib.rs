
#[cfg(test)]
mod tests;

pub mod opcode;
mod vm;

pub use vm::{FALSE, Result, TRUE, UnknownOpHandler, VM, VmError, VmFn};

fn read_i16(bytes: &[u8]) -> i16 {
    unsafe {
        let u: &i16 = std::mem::transmute(&bytes[0]);
        *u
    }
}

fn read_i32(bytes: &[u8]) -> i32 {
    unsafe {
        let u: &i32 = std::mem::transmute(&bytes[0]);
        *u
    }
}

fn write_i16(bytes: &mut [u8], value: i16) {
    unsafe {
        let u: &mut i16 = std::mem::transmute(&mut bytes[0]);
        *u = value;
    }
}

fn write_i32(bytes: &mut [u8], value: i32) {
    unsafe {
        let u: &mut i32 = std::mem::transmute(&mut bytes[0]);
        *u = value;
    }
}

fn push_i32(bytes: &mut [u8], top: usize, value: i32) -> usize {
    unsafe {
        let u: &mut i32 = std::mem::transmute(&mut bytes[top]);
        *u = value;
    }
    top - 4
}

fn pop_i32(bytes: &[u8], top: usize) -> (usize, i32) {
    let value = unsafe {
        let u: &i32 = std::mem::transmute(&bytes[top + 4]);
        *u
    };
    (top + 4, value)
}