pub enum OpCode {
    Halt,
    Set(u16, u16),
    Push(u16),
    Pop(u16),
    Eq(u16, u16, u16),
    Gt(u16, u16, u16),
    Jmp(u16),
    Jt(u16, u16),
    Jf(u16, u16),
    Add(u16, u16, u16),
    Mult(u16, u16, u16),
    Mod(u16, u16, u16),
    And(u16, u16, u16),
    Or(u16, u16, u16),
    Not(u16, u16),
    RMem(u16, u16),
    WMem(u16, u16),
    Call(u16),
    Ret,
    Out(u16),
    In(u16),
    NoOp,
}

impl OpCode {
    pub fn get_and_increment(mem: &[u16], ptr: &mut usize) -> Result<OpCode, &'static str> {
        if let Some(&code) = mem.first() {
            match code {
                0 => {
                    *ptr += 1;
                    Ok(OpCode::Halt)
                }
                1 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    *ptr += 3;
                    Ok(OpCode::Set(a, b))
                }
                2 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    *ptr += 2;
                    Ok(OpCode::Push(a))
                }
                3 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    *ptr += 2;
                    Ok(OpCode::Pop(a))
                }
                4 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    let c = *mem.get(3).ok_or("Failed to get third argument")?;
                    *ptr += 4;
                    Ok(OpCode::Eq(a, b, c))
                }
                5 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    let c = *mem.get(3).ok_or("Failed to get third argument")?;
                    *ptr += 4;
                    Ok(OpCode::Gt(a, b, c))
                }
                6 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    *ptr += 2;
                    Ok(OpCode::Jmp(a))
                }
                7 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    *ptr += 3;
                    Ok(OpCode::Jt(a, b))
                }
                8 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    *ptr += 3;
                    Ok(OpCode::Jf(a, b))
                }
                9 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    let c = *mem.get(3).ok_or("Failed to get third argument")?;
                    *ptr += 4;
                    Ok(OpCode::Add(a, b, c))
                }
                10 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    let c = *mem.get(3).ok_or("Failed to get third argument")?;
                    *ptr += 4;
                    Ok(OpCode::Mult(a, b, c))
                }
                11 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    let c = *mem.get(3).ok_or("Failed to get third argument")?;
                    *ptr += 4;
                    Ok(OpCode::Mod(a, b, c))
                }
                12 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    let c = *mem.get(3).ok_or("Failed to get third argument")?;
                    *ptr += 4;
                    Ok(OpCode::And(a, b, c))
                }
                13 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    let c = *mem.get(3).ok_or("Failed to get third argument")?;
                    *ptr += 4;
                    Ok(OpCode::Or(a, b, c))
                }
                14 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    *ptr += 3;
                    Ok(OpCode::Not(a, b))
                }
                15 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    *ptr += 3;
                    Ok(OpCode::RMem(a, b))
                }
                16 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    let b = *mem.get(2).ok_or("Failed to get second argument")?;
                    *ptr += 3;
                    Ok(OpCode::WMem(a, b))
                }
                17 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    *ptr += 2;
                    Ok(OpCode::Call(a))
                }
                18 => {
                    *ptr += 1;
                    Ok(OpCode::Ret)
                }
                19 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    *ptr += 2;
                    Ok(OpCode::Out(a))
                }
                20 => {
                    let a = *mem.get(1).ok_or("Failed to get first argument")?;
                    *ptr += 2;
                    Ok(OpCode::In(a))
                }
                21 => {
                    *ptr += 1;
                    Ok(OpCode::NoOp)
                }
                _ => Err("Invalid op code"),
            }
        } else {
            Err("Failed to get op code")
        }
    }
}
