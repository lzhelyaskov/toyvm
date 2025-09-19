pub mod opcode;
mod vm;

pub use vm::{FALSE, Result, TRUE, UnknownOpHandler, VM, VmError, VmFn};