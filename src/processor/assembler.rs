use std::{fs, str::FromStr};

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

pub fn assemble(in_asm: &str, breadcrumb: Option<&str>) -> Result<(), String> {
    let s = match fs::read_to_string(in_asm) {
        Ok(s) => s,
        Err(why) => return Err(why.to_string()),
    };

    let mut words = String::new();
    let mut texts = String::new();
    let mut ext = String::new();
    let mut labels = String::new();
    let mut code = String::new();

    let mut began = false;
    let mut ended = false;
    let mut offset = 0;

    s.lines().enumerate().try_for_each(|(i, line)| {
        let mut tokens = line.split_whitespace();
        if let Some(token) = tokens.next() {
            if !token.starts_with("//") {
                match began {
                    true => {
                        if ended {
                            return Err("File continues after END".to_owned());
                        }
                        match token.ends_with(':') {
                            true => {
                                if let Some(token) = tokens.next() {
                                    if !token.starts_with("//") {
                                        return Err(format!(
                                            "Found instruction after label at line {}",
                                            i + 1
                                        ));
                                    }
                                }
                                if ext.contains(token) {
                                    return Err(format!(
                                        "EXTERN label already defined in file at line {}",
                                        i + 1
                                    ));
                                }
                                labels.push_str(token);
                                labels.push_str((i - offset).to_string().as_str());
                                labels.push('\n');
                                offset += 1;
                            }
                            false => {
                                match OpCodes::from_str(token) {
                                    Ok(op) => match op {
                                        OpCodes::IRQ => match tokens.next() {
                                            Some(irq_type) => match irq_type.parse::<u8>() {
                                                Ok(v) => match v {
                                                    0 => {
                                                        code.push_str("HALT");
                                                        code.push('\n');
                                                    }
                                                    1 => match tokens.next() {
                                                        Some(arg) => match arg.starts_with("//") {
                                                            true => return Err(format!("Expected label at line {}", i + 1)),
                                                            false => {
                                                                code.push_str("PRINT");
                                                                code.push(' ');
                                                                code.push_str(arg);
                                                                code.push('\n');
                                                            }
                                                        }
                                                        None => return Err(format!("Expected label at line {}", i + 1))
                                                    }
                                                    2 => match tokens.next() {
                                                        Some(arg) => match arg.starts_with("//") {
                                                            true => return Err(format!("Expected label at line {}", i + 1)),
                                                            false => {
                                                                code.push_str("READ");
                                                                code.push(' ');
                                                                code.push_str(arg);
                                                                code.push('\n');
                                                            }
                                                        }
                                                        None => return Err(format!("Expected label at line {}", i + 1))
                                                    }
                                                    3 => match tokens.next() {
                                                        Some(arg) => match arg.starts_with("//") {
                                                            true => return Err(format!("Expected label at line {}", i + 1)),
                                                            false => {
                                                                code.push_str("SET");
                                                                code.push(' ');
                                                                code.push_str(arg);
                                                                code.push('\n');
                                                            }
                                                        }
                                                        None => return Err(format!("Expected label at line {}", i + 1))
                                                    }
                                                    4 => {
                                                        code.push_str("CLEAR");
                                                        code.push('\n');
                                                    }
                                                    _ => return Err(format!("Unknown IRQ type at line {}: {}", i + 1, v)),
                                                },
                                                Err(_) => return Err(format!("Expected integer at line {}\n\tfound {} instead", i + 1, irq_type))
                                            },
                                            None => return Err(format!("Expected integer at line {}", i + 1))
                                        },
                                        _ => match tokens.next() {
                                            Some(arg) => match arg.starts_with("//") {
                                                true => return Err(format!("Expected label at line {}", i + 1)),
                                                false => {
                                                    code.push_str(token);
                                                    code.push(' ');
                                                    code.push_str(arg);
                                                    code.push('\n');
                                                }
                                            }
                                            None => return Err(format!("Expected label at line {}", i + 1)),
                                        }
                                    },
                                    Err(_) => match PseudoOps::from_str(token) {
                                        Ok(psop) => match psop {
                                            PseudoOps::EXTERN => {
                                                return Err(format!(
                                                    "Found EXTERN statement after BEGIN at line {}",
                                                    i + 1
                                                ))
                                            },
                                            PseudoOps::BEGIN => {
                                                return Err("BEGIN statement already used".to_owned())
                                            }
                                            PseudoOps::END => ended = true,
                                            PseudoOps::SET => {
                                                match tokens.next() {
                                                    Some(arg) => match u32::from_str_radix(token, 2) {
                                                        Ok(_) => {
                                                            code.push_str(token);
                                                            code.push(' ');
                                                            code.push_str(arg);
                                                            code.push('\n');
                                                        }
                                                        Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token))
                                                    }
                                                    None => {
                                                        return Err(format!(
                                                            "Expected label at line {}",
                                                            i + 1
                                                        ))
                                                    }
                                                }
                                            }
                                            PseudoOps::PRINT => match tokens.next() {
                                                Some(arg) => match arg.starts_with("//") {
                                                    true => return Err(format!("Expected label at line {}", i + 1)),
                                                    false => {
                                                        code.push_str(token);
                                                        code.push(' ');
                                                        code.push_str(arg);
                                                        code.push('\n');
                                                    }
                                                }
                                                None => return Err(format!("Expected label at line {}", i + 1)),
                                            }
                                            PseudoOps::READ => match tokens.next() {
                                                Some(arg) => match arg.starts_with("//") {
                                                    true => return Err(format!("Expected label at line {}", i + 1)),
                                                    false => {
                                                        code.push_str(token);
                                                        code.push(' ');
                                                        code.push_str(arg);
                                                        code.push('\n');
                                                    }
                                                }
                                                None => return Err(format!("Expected label at line {}", i + 1)),
                                            }
                                            _ => {
                                                code.push_str(token);
                                                code.push('\n');
                                            }
                                        },
                                        Err(_) => {
                                            return Err(format!(
                                                "Expected instruction at line {}\t\nfound {} instead",
                                                i + 1,
                                                token
                                            ))
                                        }
                                    },
                                }
                            }
                        }
                    }
                    false => match token.ends_with(':') {
                        true => {
                            if ext.contains(token) {
                                return Err(format!(
                                    "EXTERN label already defined in file at line {}",
                                    i + 1
                                ));
                            }
                            match tokens.next() {
                                Some(direc) => match direc {
                                    ".word" => {
                                        words.push_str(token);
                                        while let Some(token) = tokens.next() {
                                            words.push_str(token);

                                            if token.starts_with("//") {
                                                return Err(format!("Expected arguments at line {}", i + 1));
                                            }

                                            if !token.ends_with(',') {
                                                break;
                                            }
                                        }
                                        if let Some(token) = tokens.next() {
                                            if !token.starts_with("//") {
                                                return Err(format!("Expected comma at line {}", i + 1));
                                            }
                                        }
                                        words.push('\n');
                                    }
                                    ".text" => {
                                        texts.push_str(token);
                                        match tokens.next() {
                                            Some(token) => match token.starts_with("//") {
                                                true => return Err(format!("Expected arguments after directive at line {}", i + 1)),
                                                false => texts.push_str(token),
                                            }
                                            None => return Err(format!("Expected arguments after directive at line {}", i + 1))
                                        }
                                        let args = tokens.take_while(|tok| !tok.starts_with("//")).fold(String::new(), |acc, tok| acc + " " + tok);
                                        texts.push_str(args.as_str());
                                        texts.push('\n');
                                    }
                                    _ => {
                                        return Err(format!(
                                        "Expected directive after label at line {}\n\tfound {} instead",
                                        i + 1,
                                        token
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
                                    offset = words.lines().count() + texts.lines().count() + ext.lines().count() + 1;
                                    began = true;
                                }
                                PseudoOps::EXTERN => match tokens.next() {
                                    Some(l) => match l.starts_with("//") {
                                        true => return Err(format!("Expected label at line {}", i + 1)),
                                        false => ext.push_str(l),
                                    }
                                    None => {
                                        return Err(format!(
                                            "Expected label after EXTERN at line {}",
                                            i + 1
                                        ))
                                    }
                                },
                                _ => {
                                    return Err(format!(
                                    "Expected BEGIN or EXTERN statement or label at line {}\n\tfound {} instead",
                                    i + 1, token
                                ))
                                }
                            },
                            Err(_) => {
                                return Err(format!(
                                    "Expected BEGIN or EXTERN statement or label at line {}\n\tfound {} instead",
                                    i + 1,
                                    token
                                ))
                            }
                        },
                    },
                }
            }
        }
        Ok(())
    })?;

    if !ended {
        return Err("END statement missing".to_owned());
    }
    match fs::write(
        breadcrumb.unwrap_or("a.bdc"),
        words.lines().count().to_string()
            + " "
            + texts.lines().count().to_string().as_str()
            + " "
            + labels.lines().count().to_string().as_str()
            + " "
            + ext.lines().count().to_string().as_str()
            + "\n"
            + words.as_str()
            + texts.as_str()
            + labels.as_str()
            + ext.as_str()
            + code.as_str(),
    ) {
        Ok(_) => Ok(()),
        Err(why) => Err(why.to_string()),
    }
}
