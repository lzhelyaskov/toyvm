use crate::{opcode, pop_i32, push_i32, read_i16, read_i32, write_i16, write_i32};
use std::mem;

pub type VmFn = &'static dyn Fn(&'_ mut VM);
pub type UnknownOpHandler = &'static dyn Fn(&'_ mut VM, &mut usize, u8) -> bool;

#[derive(Debug)]
pub enum VmError {
    UnknownOp(u8, usize),
    Unreachable(usize),
    UnknownVmFn(usize),
}

pub type Result<T> = std::result::Result<T, VmError>;

pub const TRUE: i32 = 0x1;
pub const FALSE: i32 = 0x0;

pub struct VM {
    memory: Vec<u8>,
    functions: Vec<VmFn>,
    pstack_top: usize,
    rstack_top: usize,
    unknown_opcode_handler: Vec<UnknownOpHandler>,
}

impl VM {
    pub fn new(
        memory: Vec<u8>,
        functions: Vec<VmFn>,
        pstack_top: usize,
        rstack_top: usize,
    ) -> Self {
        VM {
            memory,
            functions,
            pstack_top,
            rstack_top,
            unknown_opcode_handler: Vec::new(),
        }
    }

    pub fn read_u8(&self, idx: usize) -> u8 {
        self.memory[idx]
    }

    pub fn read_i16(&self, idx: usize) -> i16 {
        read_i16(&self.memory[idx..])
    }

    pub fn read_i32(&self, idx: usize) -> i32 {
        read_i32(&self.memory[idx..])
    }

    pub fn read(&self, from: usize, dst: &mut [u8]) {
        let n = dst.len();
        // for i in 0..n {
        //     dst[i] = self.memory[from + i];
        // }
        dst[..n].copy_from_slice(&self.memory[from..(n + from)]);
    }

    pub fn write_u8(&mut self, value: u8, idx: usize) {
        self.memory[idx] = value;
    }

    pub fn write_i16(&mut self, value: i16, idx: usize) {
        write_i16(&mut self.memory[idx..], value);
    }

    pub fn write_i32(&mut self, value: i32, idx: usize) {
        write_i32(&mut self.memory[idx..], value);
    }

    pub fn write(&mut self, to: usize, src: &[u8]) {
        self.memory[to..to + src.len()].copy_from_slice(src);
    }

    pub fn memcopy(&mut self, from: usize, to: usize, n: usize) {
        self.memory.copy_within(from..from + n, to);
    }

    pub fn memcmp(&self, a: usize, b: usize, n: usize) -> bool {
        for i in 0..n {
            if self.memory[a + i] != self.memory[b + i] {
                return false;
            }
        }
        true
    }

    pub fn memcmp_with(&self, from: usize, other: &[u8]) -> bool {
        for (i, b) in other.iter().enumerate() {
            if self.memory[from + i] != *b {
                return false;
            }
        }
        true
    }

    pub fn memory_ref(&self) -> &[u8] {
        &self.memory
    }

    pub fn memory_ref_mut(&mut self) -> &mut [u8] {
        &mut self.memory
    }

    pub fn push_i32(&mut self, value: i32) {
        let stack_top = self.read_i32(self.pstack_top) as usize;
        let stack_top = push_i32(&mut self.memory, stack_top, value);
        self.write_i32(stack_top as i32, self.pstack_top);
    }

    pub fn pop_i32(&mut self) -> i32 {
        let stack_top = self.read_i32(self.pstack_top) as usize;
        let (stack_top, value) = pop_i32(&self.memory, stack_top);
        self.write_i32(stack_top as i32, self.pstack_top);
        value
    }

    fn rs_push(&mut self, value: i32) {
        let stack_top = self.read_i32(self.rstack_top) as usize;
        let stack_top = push_i32(&mut self.memory, stack_top, value);
        self.write_i32(stack_top as i32, self.rstack_top);
    }

    fn rs_pop(&mut self) -> i32 {
        let stack_top = self.read_i32(self.rstack_top) as usize;
        let (stack_top, value) = pop_i32(&self.memory, stack_top);
        self.write_i32(stack_top as i32, self.rstack_top);
        value
    }

    pub fn add_function(&mut self, f: VmFn) -> usize {
        self.functions.push(f);
        self.functions.len() - 1
    }

    pub fn add_unknown_op_handler(&mut self, f: UnknownOpHandler) {
        self.unknown_opcode_handler.push(f);
    }

    fn vm_fn(&mut self) -> Result<()> {
        let fn_idx = self.pop_i32() as usize;
        if fn_idx >= self.functions.len() {
            return Err(VmError::UnknownVmFn(fn_idx));
        }
        self.functions[fn_idx](self);
        Ok(())
    }

    pub fn run(&mut self, ip: &mut usize) -> Result<()> {
        while self.step(ip)? {}

        Ok(())
    }

    pub fn step(&mut self, ip: &mut usize) -> Result<bool> {
        let op = self.memory[*ip];
        *ip += 1;

        match op {
            opcode::UNREACHABLE => {
                return Err(VmError::Unreachable(*ip));
            }
            opcode::NOP => {}
            opcode::END => {
                return Ok(false);
            }
            opcode::BR => {
                *ip = self.read_i32(*ip) as usize;
            }
            opcode::BRZ => {
                let is_zero = self.pop_i32() == 0;
                let addr = self.read_i32(*ip) as usize;
                if is_zero {
                    *ip = addr;
                } else {
                    *ip += 4;
                }
            }
            opcode::JMP => {
                *ip = self.pop_i32() as usize;
            }
            opcode::JZ => {
                let is_zero = self.pop_i32() == 0;
                let addr = self.pop_i32() as usize;
                if is_zero {
                    *ip = addr;
                }
            }
            opcode::RETURN => {
                *ip = self.rs_pop() as usize;
            }
            opcode::CALL_VM => {
                self.vm_fn()?;
            }
            opcode::CALL => {
                // store ip to return stack
                // ip = pop
                self.rs_push(*ip as i32);
                *ip = self.pop_i32() as usize;
            }
            opcode::CALLI => {
                self.rs_push(*ip as i32 + 4);
                *ip = self.read_i32(*ip) as usize;
            }
            opcode::DROP => {
                self.pop_i32();
            }
            opcode::DUP => {
                let a = self.pop_i32();
                self.push_i32(a);
                self.push_i32(a);
            }
            opcode::SWAP => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(a);
                self.push_i32(b);
            }
            opcode::SELECT => {
                unimplemented!()
            }
            opcode::I32_LOAD => {
                let addr = self.pop_i32() as usize;
                let value = self.read_i32(addr);
                self.push_i32(value);
            }
            opcode::I32_LOAD_8 => {
                let addr = self.pop_i32() as usize;
                let value = self.read_u8(addr);
                self.push_i32(value as i32);
            }
            opcode::I32_LOAD_16 => {
                let addr = self.pop_i32() as usize;
                let value = self.read_i16(addr);
                self.push_i32(value as i32);
            }

            // TODO: impl I64 LOADS and STORES
            opcode::I32_STORE => {
                // ( value addr -- )
                let addr = self.pop_i32() as usize;
                let value = self.pop_i32();
                // write_i32(&mut self.memory[addr..], value);
                self.write_i32(value, addr);
            }
            opcode::I32_STORE_8 => {
                let addr = self.pop_i32() as usize;
                let value = self.pop_i32();
                self.write_u8(value as u8, addr);
            }
            opcode::I32_STORE_16 => {
                let addr = self.pop_i32() as usize;
                let value = self.pop_i32();
                self.write_i16(value as i16, addr);
            }
            opcode::I32_CONST => {
                let value = self.read_i32(*ip);
                *ip += 4;
                self.push_i32(value);
            }
            opcode::I64_CONST => {
                unimplemented!()
            }
            opcode::EQ => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(if a == b { TRUE } else { FALSE });
            }
            opcode::EQZ => {
                let a = self.pop_i32();
                self.push_i32(if a == 0 { TRUE } else { FALSE });
            }
            opcode::NE => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(if a != b { TRUE } else { FALSE });
            }
            opcode::LT_S => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(if a < b { TRUE } else { FALSE });
            }
            opcode::LT_U => {
                let a = self.pop_i32() as u32;
                let b = self.pop_i32() as u32;
                self.push_i32(if a < b { TRUE } else { FALSE });
            }
            opcode::GT_S => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(if a > b { TRUE } else { FALSE });
            }
            opcode::GT_U => {
                let a = self.pop_i32() as u32;
                let b = self.pop_i32() as u32;
                self.push_i32(if a > b { TRUE } else { FALSE });
            }
            opcode::LE_S => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(if a <= b { TRUE } else { FALSE });
            }
            opcode::LE_U => {
                let a = self.pop_i32() as u32;
                let b = self.pop_i32() as u32;
                self.push_i32(if a <= b { TRUE } else { FALSE });
            }
            opcode::GE_S => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(if a >= b { TRUE } else { FALSE });
            }
            opcode::GE_U => {
                let a = self.pop_i32() as u32;
                let b = self.pop_i32() as u32;
                self.push_i32(if a >= b { TRUE } else { FALSE });
            }
            // TODO: I64 ops
            opcode::ADD => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a + b);
            }
            opcode::SUB => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a - b);
            }
            opcode::MUL => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a * b);
            }
            opcode::DIV_S => {
                // TODO: division by zero
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a / b);
            }
            opcode::DIV_U => {
                // TODO: division by zero
                let b = self.pop_i32() as u32;
                let a = self.pop_i32() as u32;
                self.push_i32((a / b) as i32);
            }
            opcode::MOD_S => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a % b);
            }
            opcode::MOD_U => {
                let b = self.pop_i32() as u32;
                let a = self.pop_i32() as u32;
                self.push_i32((a % b) as i32);
            }
            opcode::AND => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a & b);
            }
            opcode::OR => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a | b);
            }
            opcode::XOR => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a ^ b);
            }
            opcode::SHL => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a << b);
            }
            opcode::SHR_S => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a >> b);
            }
            opcode::SHR_U => {
                let b = self.pop_i32() as u32;
                let a = self.pop_i32() as u32;
                self.push_i32((a >> b) as i32);
            }
            opcode::ROTL => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a.rotate_left(b as u32));
            }
            opcode::ROTR => {
                let b = self.pop_i32();
                let a = self.pop_i32();
                self.push_i32(a.rotate_right(b as u32));
            }
            opcode::NOT => {
                let a = self.pop_i32();
                self.push_i32(!a);
            }
            opcode::MIN => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(a.min(b));
            }
            opcode::MAX => {
                let a = self.pop_i32();
                let b = self.pop_i32();
                self.push_i32(a.max(b));
            }

            _ => {
                let handler = mem::take(&mut self.unknown_opcode_handler);
                let mut handled = false;
                for f in &handler {
                    if f(self, ip, op) {
                        handled = true;
                        break;
                    }
                }
                self.unknown_opcode_handler = handler;
                if !handled {
                    return Err(VmError::UnknownOp(op, *ip));
                }
            }
        }
        Ok(true)
    }
}
