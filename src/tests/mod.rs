use crate::{opcode::*, VM};

const MEMSIZE: usize = 0x4000; 
const PSTACK: usize = 0x2000;
const RSTACK: usize = 0x3FFC; // 0x4000 - 4;

fn create_vm() -> VM {

    let memory = vec![0; MEMSIZE];
    let functions = Vec::new();
    let mut vm = VM::new(memory, functions, 0, 4);
    vm.write_i32(PSTACK as i32, 0);
    vm.write_i32(RSTACK as i32, 4);

    vm
}

#[test]
fn test_stack() {
    let mut vm = create_vm();

    vm.push_i32(42);

    assert_eq!(PSTACK as i32 - 4, vm.read_i32(0));
    assert_eq!(42, vm.pop_i32());
    assert_eq!(PSTACK as i32, vm.read_i32(0));
}

#[test]
fn test_memory() {
    let mut vm = create_vm();

    let bytes = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    vm.write(16, &bytes);

    for (i, byte) in bytes.iter().enumerate() {
        assert_eq!(*byte, vm.memory_ref()[16 + i]);
    }

    let mut dst = vec![0; 12];

    vm.read(16, &mut dst);


    for (i, byte) in bytes.iter().enumerate() {
        assert_eq!(*byte, dst[i]);
    }

    let r = vm.memcmp_with(16, &bytes);
    assert!(r);

    let r = vm.memcmp_with(17, &bytes);
    assert!(!r);

    vm.memcopy(16, 48, 12);

    let r = vm.memcmp(16, 48, 12);
    assert!(r);
}

#[test]
fn test_calls() {

    let mut vm = create_vm();

    let program = [
        I32_CONST,      // 16
        26, 0, 0, 0,    // 17 - 20
        CALL,           // 21
        END,            // 22
// fn square
        DUP,            // 23
        MUL,            // 24
        RETURN,         // 25
// fn quad
        CALLI,          // 26
        23, 0, 0, 0,    // 27 - 30
        CALLI,          // 31
        23, 0, 0, 0,    // 32 - 35
        RETURN          // 36
    ];

    vm.write(16, &program);
    vm.push_i32(2);
    let mut ip = 16;
    // vm.run(&mut ip).unwrap();
    
    assert!(vm.run(&mut ip).is_ok());

    let result = vm.pop_i32();
    assert_eq!(16, result);
}