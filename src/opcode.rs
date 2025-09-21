macro_rules! op_code {
    ($m:ident, $c:expr) => {
        pub const $m: u8 = $c;
    };
}
op_code!(UNREACHABLE, 0x00);
op_code!(NOP, 0x01);
// 0x02 - 0x0A reserved
op_code!(END, 0x0b);
op_code!(BR, 0x0c);
op_code!(BRZ, 0x0d);
op_code!(JMP, 0x0e);
op_code!(JZ, 0x0f);
op_code!(RETURN, 0x10);
op_code!(CALL_VM, 0x11);
op_code!(CALL, 0x12);
op_code!(CALLI, 0x13);
// 0x14 - 0x19 reserved
op_code!(DROP, 0x1a);
op_code!(DUP, 0x1b);
op_code!(SWAP, 0x1c);
op_code!(SELECT, 0x1d);

op_code!(I32_LOAD, 0x28);
op_code!(I32_LOAD_8, 0x29);
op_code!(I32_LOAD_16, 0x2a);
op_code!(I64_LOAD, 0x2b);
op_code!(I64_LOAD_8, 0x2c);
op_code!(I64_LOAD_16, 0x2d);
op_code!(I64_LOAD_32, 0x2e);
op_code!(I32_STORE, 0x2f);
op_code!(I32_STORE_8, 0x30);
op_code!(I32_STORE_16, 0x31);
op_code!(I64_STORE, 0x32);
op_code!(I64_STORE_8, 0x33);
op_code!(I64_STORE_16, 0x34);
op_code!(I64_STORE_32, 0x35);

op_code!(I32_CONST, 0x36);
op_code!(I64_CONST, 0x37);

// i32
op_code!(EQ, 0x38);
op_code!(EQZ, 0x39);
op_code!(NE, 0x3a);
op_code!(LT_S, 0x3b);
op_code!(LT_U, 0x3c);
op_code!(GT_S, 0x3d);
op_code!(GT_U, 0x3e);
op_code!(LE_S, 0x3f);
op_code!(LE_U, 0x40);
op_code!(GE_S, 0x41);
op_code!(GE_U, 0x42);
// i64
op_code!(I64_EQ, 0x43);
op_code!(I64_EQZ, 0x44);
op_code!(I64_NE, 0x45);
op_code!(I64_LT_S, 0x46);
op_code!(I64_LT_U, 0x47);
op_code!(I64_GT_S, 0x48);
op_code!(I64_GT_U, 0x49);
op_code!(I64_LE_S, 0x4a);
op_code!(I64_LE_U, 0x4b);
op_code!(I64_GE_S, 0x4c);
op_code!(I64_GE_U, 0x4d);

// 0x4c - 50 reserved for f32 - f64
// i32
op_code!(ADD, 0x51);
op_code!(SUB, 0x52);
op_code!(MUL, 0x53);
op_code!(DIV_S, 0x54);
op_code!(DIV_U, 0x55);
op_code!(MOD_S, 0x56);
op_code!(MOD_U, 0x57);

op_code!(AND, 0x58);
op_code!(OR, 0x59);
op_code!(XOR, 0x5a);
op_code!(SHL, 0x5b);
op_code!(SHR_S, 0x5c);
op_code!(SHR_U, 0x5d);
op_code!(ROTL, 0x5e);
op_code!(ROTR, 0x5f);
op_code!(NOT, 0x60);
op_code!(MIN, 0x61);
op_code!(MAX, 0x62);

op_code!(INC, 0x63);
op_code!(DEC, 0x64);
op_code!(ZERO, 0x65);

// TODO: i64, f32, f64

op_code!(NEXT, 0xff); // remove this

pub fn opcode(op: u8) -> &'static str {
    match op {
        UNREACHABLE => "unreachable",
        NOP => "nop",
        END => "end",
        BR => "br",
        BRZ => "brz",
        JMP => "jmp",
        JZ => "jz",
        RETURN => "return",
        CALL_VM => "call_vm",
        CALL => "call",
        CALLI => "calli",
        DROP => "drop",
        DUP => "dup",
        SWAP => "swap",
        SELECT => "select",

        I32_LOAD => "i32.load",
        I32_LOAD_8 => "i32.load_8",
        I32_LOAD_16 => "i32.load_16",
        I64_LOAD => "i64.load",
        I64_LOAD_8 => "i64.load_8",
        I64_LOAD_16 => "i64.load_16",
        I64_LOAD_32 => "i64.load_32",

        I32_STORE => "i32.store",
        I32_STORE_8 => "i32.store_8",
        I32_STORE_16 => "i32.store_16",
        I64_STORE => "i64.store",
        I64_STORE_8 => "i64.store_8",
        I64_STORE_16 => "i64.store_16",
        I64_STORE_32 => "i64.store_32",

        I32_CONST => "i32.const",
        I64_CONST => "i64.const",

        EQ => "i32.eq",
        EQZ => "i32.eqz",
        NE => "i32.neq",

        LT_S => "i32.lt_s",
        LT_U => "i32.lt_u",
        LE_S => "i32.le_s",
        LE_U => "i32.le_u",
        GT_S => "i32.gt_s",
        GT_U => "i32.gt_u",
        GE_S => "i32.ge_s",
        GE_U => "i32.ge_u",

        I64_EQ => "i64.eq",
        I64_EQZ => "i64.eqz",
        I64_NE => "i64.neq",

        I64_LT_S => "i64.lt_s",
        I64_LT_U => "i64.lt_u",
        I64_LE_S => "i64.le_s",
        I64_LE_U => "i64.le_u",
        I64_GT_S => "i64.gt_s",
        I64_GT_U => "i64.gt_u",
        I64_GE_S => "i64.ge_s",
        I64_GE_U => "i64.ge_u",

        ADD => "i32.add",
        SUB => "i32.sub",
        MUL => "i32.mul",
        DIV_S => "i32.div_s",
        DIV_U => "i32.div_u",
        MOD_S => "i32.mod_s",
        MOD_U => "i32.mod_u",

        AND => "i32.and",
        OR => "i32.or",
        XOR => "i32.xor",
        SHL => "i32.shl",
        SHR_S => "i32.shr_s",
        SHR_U => "i32.shr_u",
        ROTL => "i32.rotl",
        ROTR => "i32.rotr",
        NOT => "i32.not",
        MIN => "i32.min",
        MAX => "i32.max",

        INC => "i32.inc",
        DEC => "i32.dec",
        ZERO => "i32.zero",

        NEXT => ".next",

        _ => "???",
    }
}
