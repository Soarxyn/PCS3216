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

    let mut word_labels = String::new();
    let mut texts = String::new();
    let mut ext = String::new();
    let mut labels = String::new();
    let mut code = String::new();

    let mut began = false;
    let mut ended = false;
    let mut offset = 0;

    s.lines().enumerate().try_for_each(|(i, line)| {
        let line = match line.split_once("//") {
            Some((code, _comment)) => code.trim(),
            None => line.trim()
        };

        if !line.is_empty() && began && ended {
            return Err("File continues after END".to_owned());
        } else if let Some((label, words)) = line.split_once(".word") {
            if began {
                return Err(format!("Found .word directive after BEGIN statement at line {}", i + 1));
            }

            let label = label.trim_end();
            match label.ends_with(':') {
                false => return Err(format!("Expected label before .word directive at line {}", i + 1)),
                true => match label.trim_end_matches(':').trim_end().chars().any(|c| c.is_whitespace()) {
                    true => return Err(format!("Parsing error at line {}", i + 1)),
                    false => if word_labels.lines().any(|line| line.starts_with(label))
                        || texts.lines().any(|line| line.starts_with(label))
                        || ext.lines().any(|line| line.starts_with(label)) {

                        return Err(format!("Label {} found at line {} previously defined", label, i + 1));
                    }
                }
            }

            if words.split(',').any(|word| word.trim().parse::<u32>().is_err()) {
                return Err(format!("Unexpected character at line {}", i + 1));
            }

            word_labels.extend(label.chars().filter(|c| !c.is_whitespace()));
            word_labels.extend(words.chars().filter(|c| !c.is_whitespace()));
            word_labels.push('\n');
        } else if let Some((label, text)) = line.split_once(".text") {
            if began {
                return Err(format!("Found .text directive after BEGIN statement at line {}", i + 1));
            }

            let label = label.trim_end();
            match label.ends_with(':') {
                false => return Err(format!("Expected label before .text directive at line {}", i + 1)),
                true => match label.trim_end_matches(':').trim_end().chars().any(|c| c.is_whitespace()) {
                    true => return Err(format!("Parsing error at line {}", i + 1)),
                    false => if word_labels.lines().any(|line| line.starts_with(label))
                        || texts.lines().any(|line| line.starts_with(label))
                        || ext.lines().any(|line| line.starts_with(label)) {

                        return Err(format!("Label {} found at line {} previously defined", label, i + 1));
                    }
                }
            }

            texts.extend(label.chars().filter(|c| !c.is_whitespace()));
            texts.push_str(text.trim_start());
            texts.push('\n');
        } else if let Some((label, text)) = line.split_once(':') {
            if !began {
                return Err(format!("Expected directive after label at line {}", i + 1));
            }

            let label = label.trim_end();
            match label.chars().any(|c| c.is_whitespace()) {
                true => return Err(format!("Parsing error at line {}", i + 1)),
                false => if word_labels.lines().any(|line| line.starts_with(label))
                    || texts.lines().any(|line| line.starts_with(label))
                    || ext.lines().any(|line| line.starts_with(label))
                    || labels.lines().any(|line| line.starts_with(label)) {

                    return Err(format!("Label {} found at line {} previously defined", label, i + 1));
                }
            }

            labels.extend(label.chars().filter(|c| !c.is_whitespace()));
            labels.push(':');
            labels.push_str((i - offset).to_string().as_str());
            labels.push('\n');

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
                                        Some(arg) => {
                                            code.push_str(match v {
                                                1 => "PRINT ",
                                                2 => "READ ",
                                                3 => "SET ",
                                                _ => return Err(format!("IRQ parsing error at line {}", i + 1)),
                                            });
                                            code.push_str(arg);
                                            code.push('\n');
                                        }
                                        None => code.push_str(match v {
                                            0 => "HALT\n",
                                            4 => "CLEAR\n",
                                            _ => return Err(format!("IRQ parsing error at line {}", i + 1)),
                                        }),
                                    }
                                }
                            }
                            _ => match tokens.next() {
                                None => return Err(format!("Expected label at line {}", i + 1)),
                                Some(arg) => {
                                    code.push_str(token);
                                    code.push(' ');
                                    code.push_str(arg);
                                    code.push('\n');
                                }
                            }
                        }
                    } else {
                        match PseudoOps::from_str(token) {
                            Err(_) => return Err(format!( "Expected instruction at line {}\t\nfound {} instead", i + 1, token)),

                            Ok(psop) => match psop {
                                PseudoOps::EXTERN => return Err(format!("Found EXTERN statement after BEGIN at line {}", i + 1)),

                                PseudoOps::BEGIN => return Err("BEGIN statement already used".to_owned()),

                                PseudoOps::END => ended = true,

                                PseudoOps::SET => match tokens.next() {
                                    None => return Err(format!( "Expected label at line {}", i + 1)),

                                    Some(arg) => match u32::from_str_radix(arg, 2) {
                                        Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token)),

                                        Ok(_) => {
                                            code.push_str(token);
                                            code.push(' ');
                                            code.push_str(arg);
                                            code.push('\n');
                                        }
                                    }
                                }
                                PseudoOps::PRINT | PseudoOps::READ => match tokens.next() {
                                    None => return Err(format!("Expected label at line {}", i + 1)),

                                    Some(arg) => {
                                        code.push_str(token);
                                        code.push(' ');
                                        code.push_str(arg);
                                        code.push('\n');
                                    }
                                }
                                _ => {
                                    code.push_str(token);
                                    code.push('\n');
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
                            None => return Err(format!("Expected IRQ type at line {}", i + 1)),

                            Some(irq_type) => match irq_type.parse::<u8>() {
                                Err(_) => return Err(format!("Expected IRQ type at line {}\n\tfound {} instead", i + 1, irq_type)),

                                Ok(v) => match tokens.next() {
                                    Some(arg) => {
                                        code.push_str(match v {
                                            1 => "PRINT ",
                                            2 => "READ ",
                                            3 => "SET ",
                                            0 | 4 => return Err(format!("Unexpected argument at line {}\n\t{}", i + 1, arg)),
                                            _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                        });
                                        code.push_str(arg);
                                        code.push('\n');
                                    }
                                    None => code.push_str(match v {
                                        0 => "HALT\n",
                                        4 => "CLEAR\n",
                                        1..=3 => return Err(format!("Expected argument at line {}", i + 1)),
                                        _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                    }),
                                }
                            }
                        }
                        _ => match tokens.next() {
                            None => return Err(format!("Expected label at line {}", i + 1)),

                            Some(arg) => {
                                code.push_str(token);
                                code.push(' ');
                                code.push_str(arg);
                                code.push('\n');
                            }
                        }
                    }
                } else {
                    match PseudoOps::from_str(token) {
                        Err(_) => return Err(format!( "Expected label or instruction at line {}\t\nfound {} instead", i + 1, token)),

                        Ok(psop) => match began {
                            true => match psop {
                                PseudoOps::EXTERN => return Err(format!("Found EXTERN statement after BEGIN at line {}", i + 1)),

                                PseudoOps::BEGIN => return Err("BEGIN statement already used".to_owned()),

                                PseudoOps::END => ended = true,

                                PseudoOps::SET => match tokens.next() {
                                    None => return Err(format!( "Expected label at line {}", i + 1)),

                                    Some(arg) => match u32::from_str_radix(arg, 2) {
                                        Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token)),

                                        Ok(_) => {
                                            code.push_str(token);
                                            code.push(' ');
                                            code.push_str(arg);
                                            code.push('\n');
                                        }
                                    }
                                }
                                PseudoOps::PRINT | PseudoOps::READ => match tokens.next() {
                                    None => return Err(format!("Expected label at line {}", i + 1)),

                                    Some(arg) => {
                                        code.push_str(token);
                                        code.push(' ');
                                        code.push_str(arg);
                                        code.push('\n');
                                    }
                                }
                                _ => {
                                    code.push_str(token);
                                    code.push('\n');
                                }
                            }
                            false => match psop {
                                PseudoOps::BEGIN => {
                                    offset = word_labels.lines().count() + texts.lines().count() + ext.lines().count() + 1;
                                    began = true;
                                }
                                PseudoOps::EXTERN => match tokens.next() {
                                    None => return Err(format!("Expected label after EXTERN at line {}", i + 1)),

                                    Some(l) => ext.push_str(l),
                                }
                                _ => return Err(format!("Expected BEGIN or EXTERN statement or label at line {}\n\tfound {} instead", i + 1, token))
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

    match fs::write(
        breadcrumb.unwrap_or("a.bdc"),
        word_labels.lines().count().to_string()
            + " "
            + texts.lines().count().to_string().as_str()
            + " "
            + labels.lines().count().to_string().as_str()
            + " "
            + ext.lines().count().to_string().as_str()
            + "\n"
            + word_labels.as_str()
            + texts.as_str()
            + labels.as_str()
            + ext.as_str()
            + code.as_str(),
    ) {
        Ok(_) => Ok(()),
        Err(why) => Err(why.to_string()),
    }
}
