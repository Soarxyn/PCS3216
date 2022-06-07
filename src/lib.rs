use std::{collections::HashMap, fs, str::FromStr};
extern crate strum;
#[macro_use]
extern crate strum_macros;
#[repr(u8)]
#[derive(EnumString)]
pub enum OpCodes {
    IRQ,
    LDA,
    STA,
    ADD,
    SUB,
    MUL,
    DIV,
    CMP,
    NEG,
    BEQ,
    BGT,
    BLT,
    BHS,
    BMI,
    BVS,
    BHI,
    PSH,
    POP,
    JAL,
    JMP,
    AND,
    ORR,
    NOT,
    XOR,
    LSL,
    LSR,
    ASL,
    ASR,
    ROL,
    ROR,
    RCL,
    RCR,
}

#[derive(EnumString, PartialEq)]
pub enum PseudoOps {
    HALT,
    PRINT,
    READ,
    BEGIN,
    END,
    SET,
    CLEAR,
    EXTERN,
}

pub fn assemble(in_asm: &str, out: &str) -> Result<(), String> {
    let mut symbol_table = HashMap::new();

    let s = match fs::read_to_string(in_asm) {
        Ok(s) => s,
        Err(why) => return Err(why.to_string()),
    };

    let mut addr = 1 << 25;
    let mut began = false;
    let mut ended = false;
    s.lines().enumerate().try_for_each(|(i, line)| {
        let mut tokens = line.split_whitespace();
        if let Some(token) = tokens.next() {
            match began {
                true => {
                    if ended {
                        return Err("File continues after END".to_owned());
                    }
                    match token.ends_with(':') {
                        true => {
                            if let Some(token) = tokens.next() {
                                if OpCodes::from_str(token).is_err() {
                                    match PseudoOps::from_str(token) {
                                        Ok(psop) => match psop {
                                            PseudoOps::BEGIN => {
                                                return Err(format!(
                                                    "Repeated BEGIN statement at line {}",
                                                    i + 1
                                                ))
                                            }
                                            PseudoOps::END => ended = true,
                                            PseudoOps::EXTERN => {
                                                return Err(format!(
                                                    "Found EXTERN statement after BEGIN at line {}",
                                                    i + 1
                                                ))
                                            }
                                            _ => (),
                                        },
                                        Err(_) => {
                                            return Err(format!(
                                            "Expected instruction at line {}\t\nfound {} instead",
                                            i + 1, token
                                        ))
                                        }
                                    }
                                }
                            }
                            symbol_table.insert(token.trim_end_matches(':'), addr);
                            addr += 1;
                        }
                        false => {
                            if OpCodes::from_str(token).is_err() {
                                match PseudoOps::from_str(token) {
                                    Ok(psop) => match psop {
                                        PseudoOps::BEGIN => {
                                            return Err("BEGIN statement already used".to_owned())
                                        }
                                        PseudoOps::END => ended = true,
                                        PseudoOps::EXTERN => {
                                            return Err(format!(
                                                "Found EXTERN statement after BEGIN at line {}",
                                                i + 1
                                            ))
                                        }
                                        _ => (),
                                    },
                                    Err(_) => {
                                        return Err(format!(
                                            "Expected instruction at line {}\t\nfound {} instead",
                                            i + 1,
                                            token
                                        ))
                                    }
                                }
                            }
                        }
                    }
                }
                false => match token.ends_with(':') {
                    true => {
                        match tokens.next() {
                            Some(direc) => match direc {
                                ".word" => {
                                    symbol_table.insert(token.trim_end_matches(':'), addr);
                                    while let Some(token) = tokens.next() {
                                        // todo!("insert into memory");
                                        addr += 1;
                                        if !token.ends_with(',') {
                                            break;
                                        }
                                    }
                                    if tokens.next().is_some() {
                                        return Err(format!("Expected comma at line {}", i + 1));
                                    }
                                }
                                ".text" => {
                                    symbol_table.insert(token.trim_end_matches(':'), addr);
                                    while let Some(token) = tokens.next() {
                                        // todo!("insert into memory");
                                        addr += 1;
                                        if !token.ends_with(',') {
                                            break;
                                        }
                                    }
                                    if tokens.next().is_some() {
                                        return Err(format!("Expected comma at line {}", i + 1));
                                    }
                                }
                                _ => {
                                    return Err(format!(
                                    "Expected directive after label at line {}\n\tfound {} instead",
                                    i + 1, token
                                ))
                                }
                            },
                            None => {
                                return Err(format!(
                                    "Expected directive after label at line {}",
                                    i + 1
                                ))
                            }
                        }
                    }
                    false => match PseudoOps::from_str(token) {
                        Ok(psop) => match psop {
                            PseudoOps::BEGIN => {
                                began = true;
                                addr = 0;
                            }
                            PseudoOps::EXTERN => todo!(),
                            _ => {
                                return Err(format!(
                                "Expected label or BEGIN statement at line {}\n\tfound {} instead",
                                i + 1, token
                            ))
                            }
                        },
                        Err(_) => {
                            return Err(format!(
                                "Expected label or BEGIN statement at line {}\n\tfound {} instead",
                                i + 1,
                                token
                            ))
                        }
                    },
                },
            }
        }
        Ok(())
    })?;

    let mut buf = Vec::new();

    began = false;
    s.lines()
        .enumerate()
        .try_for_each(|(i, line)| {
            let mut tokens = line.split_whitespace();
            if let Some(token) = tokens.next() {
                match began {
                    true => match token.ends_with(':') {
                        true => match tokens.next() {
                            Some(token) => match OpCodes::from_str(token) {
                                Ok(op) => match tokens.next() {
                                    Some(token) => {
                                        let field = match symbol_table.contains_key(token) {
                                            true => symbol_table[token],
                                            false => match u32::from_str_radix(token, 16) {
                                                Ok(field) => field,
                                                Err(_) => return Err(format!("Expected hex number as argument at line {}\n\tfound {} instead", i + 1, token))
                                            }
                                        };
                                        buf.push((op as u32) << 27 | field);
                                    }
                                    None => return Err(format!("Expected argument at line {}", i + 1)),
                                },
                                Err(_) => match PseudoOps::from_str(token) {
                                    Ok(psop) => match psop {
                                        PseudoOps::HALT => buf.push(0),
                                        PseudoOps::PRINT => buf.push(1 << 24),
                                        PseudoOps::READ => buf.push(2 << 24),
                                        PseudoOps::SET => {
                                            let field = match tokens.next() {
                                                Some(token) => match u32::from_str_radix(token, 2) {
                                                    Ok(field) => field,
                                                    Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token))
                                                }
                                                None => {
                                                    return Err(format!(
                                                        "Expected argument at line {}",
                                                        i + 1
                                                    ))
                                                }
                                            };
                                            buf.push(3 << 24 | field);
                                        }
                                        PseudoOps::CLEAR => buf.push(4 << 24),
                                        PseudoOps::EXTERN => todo!(),
                                        _ => (),
                                    },
                                    Err(_) => {
                                        return Err(format!(
                                            "Expected instruction at line {}\t\nfound {} instead",
                                            i + 1, token
                                        ))
                                    }
                                },
                            },
                            None => ()
                        },
                        false => match OpCodes::from_str(token) {
                            Ok(op) => match tokens.next() {
                                Some(token) => {
                                    let field = match symbol_table.contains_key(token) {
                                        true => symbol_table[token],
                                        false => match u32::from_str_radix(token, 16) {
                                            Ok(field) => field,
                                            Err(_) => return Err(format!("Expected hex number as argument at line {}\n\tfound {} instead", i + 1, token))
                                        }
                                    };
                                    buf.push((op as u32) << 27 | field);
                                }
                                None => return Err(format!("Expected argument at line {}", i + 1)),
                            },
                            Err(_) => match PseudoOps::from_str(token) {
                                Ok(psop) => match psop {
                                    PseudoOps::HALT => buf.push(0),
                                    PseudoOps::PRINT => buf.push(1 << 24),
                                    PseudoOps::READ => buf.push(2 << 24),
                                    PseudoOps::SET => {
                                        let field = match tokens.next() {
                                            Some(token) => match u32::from_str_radix(token, 2) {
                                                Ok(field) => field,
                                                Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token))
                                            }
                                            None => {
                                                return Err(format!(
                                                    "Expected argument at line {}",
                                                    i + 1
                                                ))
                                            }
                                        };
                                        buf.push(3 << 24 | field);
                                    }
                                    PseudoOps::CLEAR => buf.push(4 << 24),
                                    PseudoOps::EXTERN => todo!(),
                                    _ => (),
                                },
                                Err(_) => {
                                    return Err(format!(
                                        "Expected instruction at line {}\t\nfound {} instead",
                                        i + 1, token
                                    ))
                                }
                            },
                        },
                    },
                    false => {
                        if let Ok(psop) = PseudoOps::from_str(token) {
                            match psop {
                                PseudoOps::BEGIN => began = true,
                                PseudoOps::EXTERN => todo!(),
                                _ => (),
                            }
                        }
                        // todo!("header");
                    }
                }
            }
            Ok(())
        })?;
    match fs::write(
        out,
        buf.into_iter()
            .map(|seq| seq.to_le_bytes())
            .flatten()
            .collect::<Vec<_>>(),
    ) {
        Ok(_) => (),
        Err(why) => return Err(why.to_string()),
    };
    Ok(())
}
