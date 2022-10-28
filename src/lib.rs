use crate::op_code::OpCode;
use std::io::stdin;
use std::ops::{BitAnd, BitOr, Not, Rem};

mod op_code;

enum Value {
    Literal(u16),
    Register(u8),
}

impl TryFrom<u16> for Value {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value <= 32767 {
            Ok(Value::Literal(value))
        } else if value <= 35775 {
            Ok(Value::Register((value - 32768) as u8))
        } else {
            Err("Given value is neither a value or register")
        }
    }
}

pub struct VirtualMachine {
    memory: [u16; 32768],
    ptr: usize,
    registers: [u16; 8],
    stack: Vec<u16>,
    input: String,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self {
            memory: [0; 32768],
            ptr: 0,
            registers: [0; 8],
            stack: vec![],
            input: String::new(),
        }
    }
}

impl VirtualMachine {
    pub fn execute(&mut self, binary: &[u8]) -> Result<(), &'static str> {
        self.copy_binary_to_mem(binary)?;

        'program: while self.ptr < self.memory.len() {
            let code = OpCode::get_and_increment(&self.memory[self.ptr..], &mut self.ptr)?;
            match code {
                OpCode::Halt => {
                    break 'program;
                }
                OpCode::Set(a, b) => {
                    let val = self.get_value_from_enum(Value::try_from(b)?);
                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;
                    *reg = val;
                }
                OpCode::Push(a) => {
                    let val = self.get_value_from_enum(Value::try_from(a)?);
                    self.stack.push(val);
                }
                OpCode::Pop(a) => {
                    if let Some(val) = self.stack.pop() {
                        let reg = self.get_register_from_enum(Value::try_from(a)?)?;
                        *reg = val;
                    } else {
                        return Err("Stack is empty");
                    }
                }
                OpCode::Eq(a, b, c) => {
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    let val_c = self.get_value_from_enum(Value::try_from(c)?);
                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;
                    *reg = u16::from(val_b == val_c);
                }
                OpCode::Gt(a, b, c) => {
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    let val_c = self.get_value_from_enum(Value::try_from(c)?);
                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;
                    *reg = u16::from(val_b > val_c);
                }
                OpCode::Jmp(a) => {
                    let val_a = self.get_value_from_enum(Value::try_from(a)?);
                    self.ptr = val_a as usize;
                }
                OpCode::Jt(a, b) => {
                    let val_a = self.get_value_from_enum(Value::try_from(a)?);
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);

                    if val_a != 0 {
                        self.ptr = val_b as usize;
                    }
                }
                OpCode::Jf(a, b) => {
                    let val_a = self.get_value_from_enum(Value::try_from(a)?);
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);

                    if val_a == 0 {
                        self.ptr = val_b as usize;
                    }
                }
                OpCode::Add(a, b, c) => {
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    let val_c = self.get_value_from_enum(Value::try_from(c)?);

                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;

                    *reg = VirtualMachine::add(val_b, val_c);
                }
                OpCode::Mult(a, b, c) => {
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    let val_c = self.get_value_from_enum(Value::try_from(c)?);

                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;

                    *reg = VirtualMachine::mult(val_b, val_c);
                }
                OpCode::Mod(a, b, c) => {
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    let val_c = self.get_value_from_enum(Value::try_from(c)?);

                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;

                    *reg = val_b.rem(val_c);
                }
                OpCode::And(a, b, c) => {
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    let val_c = self.get_value_from_enum(Value::try_from(c)?);

                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;

                    *reg = val_b.bitand(val_c);
                }
                OpCode::Or(a, b, c) => {
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    let val_c = self.get_value_from_enum(Value::try_from(c)?);

                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;

                    *reg = val_b.bitor(val_c);
                }
                OpCode::Not(a, b) => {
                    let val = self.get_value_from_enum(Value::try_from(b)?);
                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;
                    *reg = val.not().bitand(0x7FFF);
                }
                OpCode::RMem(a, b) => {
                    let val = self.memory[self.get_value_from_enum(Value::try_from(b)?) as usize];
                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;
                    *reg = val;
                }
                OpCode::WMem(a, b) => {
                    let val_a = self.get_value_from_enum(Value::try_from(a)?);
                    let val_b = self.get_value_from_enum(Value::try_from(b)?);
                    self.memory[val_a as usize] = val_b;
                }
                OpCode::Call(a) => {
                    self.stack.push(self.ptr as u16);
                    let val = self.get_value_from_enum(Value::try_from(a)?);
                    self.ptr = val as usize;
                }
                OpCode::Ret => {
                    if let Some(val) = self.stack.pop() {
                        self.ptr = val as usize;
                    } else {
                        return Err("Stack is empty");
                    }
                }
                OpCode::Out(a) => {
                    let val = self.get_value_from_enum(Value::try_from(a)?);
                    if let Some(c) = char::from_u32(val as u32) {
                        print!("{}", c);
                    }
                }
                OpCode::In(a) => {
                    if self.input.is_empty() {
                        stdin()
                            .read_line(&mut self.input)
                            .map_err(|_| "Failed to read input")?;
                        self.input = format!("{}{}", self.input.trim(), '\n');
                    }
                    let val = self.input.as_bytes()[0];
                    if val == b'\n' {
                        self.input = String::new();
                    } else {
                        self.input = self.input[1..].to_string();
                    }
                    let reg = self.get_register_from_enum(Value::try_from(a)?)?;
                    *reg = val as u16;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn copy_binary_to_mem(&mut self, binary: &[u8]) -> Result<(), &'static str> {
        for (addr, chunk) in binary.chunks(2).enumerate() {
            self.memory[addr] = u16::from_le_bytes(
                chunk
                    .try_into()
                    .map_err(|_| "Failed to read chunk to u16")?,
            );
        }

        Ok(())
    }

    #[inline]
    fn get_value_from_enum(&self, val: Value) -> u16 {
        match val {
            Value::Literal(val) => val,
            Value::Register(reg) => self.registers[reg as usize],
        }
    }

    #[inline]
    fn get_register_from_enum(&mut self, val: Value) -> Result<&mut u16, &'static str> {
        match val {
            Value::Register(reg) => Ok(&mut self.registers[reg as usize]),
            _ => Err("Value is not a register"),
        }
    }

    #[inline]
    fn add(a: u16, b: u16) -> u16 {
        ((a as u32 + b as u32) % 32768) as u16
    }

    #[inline]
    fn mult(a: u16, b: u16) -> u16 {
        ((a as u32 * b as u32) % 32768) as u16
    }
}
