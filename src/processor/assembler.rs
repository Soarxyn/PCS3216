use std::{fs, str::FromStr};

#[repr(u8)]
#[derive(EnumString, FromRepr, Debug)]
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

#[repr(u8)]
#[derive(EnumString, PartialEq, FromRepr)]
pub enum PseudoOps {
    HALT,
    PRINT,
    READ,
    SET,
    CLEAR,
    BEGIN,
    END,
    EXTERN,
}

pub fn assemble(in_asm: &str, breadcrumb: Option<&str>) -> Result<(), String> {
    let s = match fs::read_to_string(in_asm) {
        Ok(s) => s,
        Err(why) => return Err(why.to_string()),
    };

    let mut buf = String::with_capacity(s.len());
    buf.push('\n');

    let mut began = false;
    let mut ended = false;

    let mut header_len = 1;
    let mut offset = 0;

    s.lines().enumerate().try_for_each(|(i, line)| {
        let line = match line.split_once("//") {
            Some((code, _comment)) => code.trim(),
            None => line.trim()
        };

        if !line.is_empty() {
            if ended {
                return Err("File continues after END".to_owned());
            } else if let Some((label, words)) = line.split_once(".word") {
                if began {
                    return Err(format!("Found .word directive after BEGIN statement at line {}", i + 1));
                }

                let label = label.trim_end();
                match label.strip_suffix(':') {
                    None => return Err(format!("Expected label before .word directive at line {}", i + 1)),

                    Some(label) => match label.trim_end().chars().any(|c| c.is_whitespace()) {
                        true => return Err(format!("Found whitespace in label at line {}\n\t{}", i + 1, label)),

                        false => if buf.lines().take(header_len).any(|line| line.starts_with(label)) {
                            return Err(format!("Found label redefinition at line {}\n\t{}", i + 1, label));
                        }
                    }
                }

                match words.starts_with(|c: char| c.is_whitespace()) {
                    false => return Err(format!("Expected whitespace after .word directive at line {}", i + 1)),

                    true => words.split(',').try_for_each(|word| match word.trim().parse::<u32>() {
                        Err(_) => Err(format!("Couldn't parse word at line {}\n\t{}", i + 1, word)),
                        Ok(_) => Ok(()),
                    })?,
                }

                buf.extend(label.chars().chain(words.chars()).filter(|c| !c.is_whitespace()));
                buf.push('\n');
                header_len += 1;
            } else if let Some((label, text)) = line.split_once(".text") {
                if began {
                    return Err(format!("Found .text directive after BEGIN statement at line {}", i + 1));
                }

                let label = label.trim_end();
                match label.strip_suffix(':') {
                    None => return Err(format!("Expected label before .text directive at line {}", i + 1)),

                    Some(label) => match label.trim_end().chars().any(|c| c.is_whitespace()) {
                        true => return Err(format!("Found whitespace in label at line {}\n\t{}", i + 1, label)),

                        false => if buf.lines().take(header_len).any(|line| line.starts_with(label)) {
                            return Err(format!("Found label redefinition at line {}\n\t{}", i + 1, label));
                        }
                    }
                }

                let text = match text.strip_prefix(|c: char| c.is_whitespace()) {
                    None => return Err(format!("Expected whitespace after .text directive at line {}", i + 1)),

                    Some(text) => text,
                };

                buf.extend(label.chars().filter(|c| !c.is_whitespace()));
                buf.push_str(text);
                buf.push_str("\"\n");
                header_len += 1;
            } else if let Some((label, text)) = line.split_once(':') {
                if !began {
                    return Err(format!("Expected directive after label at line {}", i + 1));
                }

                let label = label.trim_end();
                match label.chars().any(|c| c.is_whitespace()) {
                    true => return Err(format!("Found whitespace in label at line {}\n\t{}", i + 1, label)),

                    false => if buf.lines().take(header_len).any(|line| line.starts_with(label)) {
                        return Err(format!("Label {} found at line {} previously defined", label, i + 1));
                    }
                }

                let string = label.to_owned() + " " + (i - offset).to_string().as_str() + "\n";

                buf.insert_str(buf.lines().take(header_len).map(|line| line.bytes().count() + 1).reduce(|acc, n| acc + n).unwrap_or(0), string.as_str());
                header_len += 1;

                let mut tokens = text.split_whitespace();
                match tokens.next() {
                    None => offset += 1,
                    Some(token) => {
                        if let Ok(op) = OpCodes::from_str(token) {
                            match op {
                                OpCodes::IRQ => match tokens.next() {
                                    None => return Err(format!("Expected integer at line {}", i + 1)),

                                    Some(irq_type) => match irq_type.parse::<u8>() {
                                        Err(_) => return Err(format!("Expected integer at line {}\n\tfound {} instead", i + 1, irq_type)),

                                        Ok(v) => match tokens.next() {
                                            Some(arg) => match v {
                                                1..=3 => {
                                                    buf.push_str("IRQ ");
                                                    buf.push_str(irq_type);
                                                    buf.push(' ');
                                                    buf.push_str(arg);
                                                    buf.push('\n');
                                                }
                                                0 | 4 => return Err(format!("Unexpected argument at line {}\n\t{}", i + 1, token)),
                                                _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                            }
                                            None => match v {
                                                0 | 4 => {
                                                    buf.push_str("IRQ ");
                                                    buf.push_str(irq_type);
                                                    buf.push('\n');
                                                }
                                                1..=3 => return Err(format!("Expected label at line {}", i + 1)),
                                                _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                            }
                                        }
                                    }
                                }
                                _ => match tokens.next() {
                                    None => return Err(format!("Expected label at line {}", i + 1)),

                                    Some(arg) => {
                                        buf.push_str(token);
                                        buf.push(' ');
                                        buf.push_str(arg);
                                        buf.push('\n');
                                    }
                                }
                            }
                        } else {
                            match PseudoOps::from_str(token) {
                                Err(_) => return Err(format!("Expected instruction at line {}\t\nfound {} instead", i + 1, token)),

                                Ok(psop) => match psop {
                                    PseudoOps::EXTERN => return Err(format!("Found EXTERN statement after BEGIN at line {}", i + 1)),

                                    PseudoOps::BEGIN => return Err(format!("Found repeated BEGIN statement at line {}", i + 1)),

                                    PseudoOps::END => ended = true,

                                    PseudoOps::SET => match tokens.next() {
                                        None => return Err(format!("Expected label at line {}", i + 1)),

                                        Some(arg) => match u32::from_str_radix(arg, 2) {
                                            Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token)),

                                            Ok(_) => {
                                                buf.push_str("IRQ 3 ");
                                                buf.push_str(arg);
                                                buf.push('\n');
                                            }
                                        }
                                    }
                                    PseudoOps::PRINT | PseudoOps::READ => match tokens.next() {
                                        None => return Err(format!("Expected label at line {}", i + 1)),

                                        Some(arg) => {
                                            buf.push_str("IRQ ");
                                            buf.push_str((psop as u8).to_string().as_str());
                                            buf.push(' ');
                                            buf.push_str(arg);
                                            buf.push('\n');
                                        }
                                    }
                                    _ => {
                                        buf.push_str("IRQ ");
                                        buf.push_str((psop as u8).to_string().as_str());
                                        buf.push('\n');
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(token) = tokens.next() {
                    return Err(format!("Unexpected argument at line {}\n\t{}", i + 1, token));
                }
            } else {
                let mut tokens = line.split_whitespace();
                if let Some(token) = tokens.next() {
                    if let Ok(op) = OpCodes::from_str(token) {
                        if !began {
                            return Err(format!("Found instruction before BEGIN statement at line {}", i + 1));
                        }
                        match op {
                            OpCodes::IRQ => match tokens.next() {
                                None => return Err(format!("Expected integer at line {}", i + 1)),

                                Some(irq_type) => match irq_type.parse::<u8>() {
                                    Err(_) => return Err(format!("Expected integer at line {}\n\tfound {} instead", i + 1, irq_type)),

                                    Ok(v) => match tokens.next() {
                                        Some(arg) => match v {
                                            1..=3 => {
                                                buf.push_str("IRQ ");
                                                buf.push_str(irq_type);
                                                buf.push(' ');
                                                buf.push_str(arg);
                                                buf.push('\n');
                                            }
                                            0 | 4 => return Err(format!("Unexpected argument at line {}\n\t{}", i + 1, token)),
                                            _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                        }
                                        None => match v {
                                            0 | 4 => {
                                                buf.push_str("IRQ ");
                                                buf.push_str(irq_type);
                                                buf.push('\n');
                                            }
                                            1..=3 => return Err(format!("Expected label at line {}", i + 1)),
                                            _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                        }
                                    }
                                }
                            }
                            _ => match tokens.next() {
                                None => return Err(format!("Expected label at line {}", i + 1)),

                                Some(arg) => {
                                    buf.push_str(token);
                                    buf.push(' ');
                                    buf.push_str(arg);
                                    buf.push('\n');
                                }
                            }
                        }
                    } else {
                        match PseudoOps::from_str(token) {
                            Err(_) => return Err(format!("Expected label or instruction at line {}\t\nfound {} instead", i + 1, token)),

                            Ok(psop) => match began {
                                true => match psop {
                                    PseudoOps::EXTERN => return Err(format!("Found EXTERN statement after BEGIN at line {}", i + 1)),

                                    PseudoOps::BEGIN => return Err(format!("Found repeated BEGIN statement at line {}", i + 1)),

                                    PseudoOps::END => ended = true,

                                    PseudoOps::SET => match tokens.next() {
                                        None => return Err(format!("Expected label at line {}", i + 1)),

                                        Some(arg) => match u32::from_str_radix(arg, 2) {
                                            Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token)),

                                            Ok(_) => {
                                                buf.push_str("IRQ 3 ");
                                                buf.push_str(arg);
                                                buf.push('\n');
                                            }
                                        }
                                    }
                                    PseudoOps::PRINT | PseudoOps::READ => match tokens.next() {
                                        None => return Err(format!("Expected label at line {}", i + 1)),

                                        Some(arg) => {
                                            buf.push_str("IRQ ");
                                            buf.push_str((psop as u8).to_string().as_str());
                                            buf.push(' ');
                                            buf.push_str(arg);
                                            buf.push('\n');
                                        }
                                    }
                                    _ => {
                                        buf.push_str("IRQ ");
                                        buf.push_str((psop as u8).to_string().as_str());
                                        buf.push('\n');
                                    }
                                }
                                false => match psop {
                                    PseudoOps::BEGIN => {
                                        offset = i + 1;
                                        began = true;
                                    }
                                    PseudoOps::EXTERN => match tokens.next() {
                                        None => return Err(format!("Expected label after EXTERN at line {}", i + 1)),

                                        Some(label) => buf.push_str(label),
                                    }
                                    _ => return Err(format!("Expected BEGIN or EXTERN statement or label at line {}\n\tfound {} instead", i + 1, token))
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    })?;

    if !ended {
        return Err("END statement missing".to_owned());
    }

    buf.insert_str(0, (header_len - 1).to_string().as_str());

    match fs::write(breadcrumb.unwrap_or("a.bdc"), buf) {
        Ok(_) => Ok(()),
        Err(why) => Err(why.to_string()),
    }
}
