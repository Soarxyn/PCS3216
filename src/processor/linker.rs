use super::assembler::{OpCodes, PseudoOps};
use std::{collections::HashMap, fs, str::FromStr};

pub fn link(breadcrumbs: Vec<&str>, out: Option<&str>) -> Result<(), String> {
    let mut offset = 0;
    let mut labels = HashMap::new();
    let mut data_labels = HashMap::new();
    let mut externs = Vec::new();
    let mut buf = Vec::new();

    breadcrumbs.iter().try_for_each(|bdc| {
        let s = match fs::read_to_string(bdc) {
            Err(why) => return Err(why.to_string()),

            Ok(s) => s,
        };

        let mut lines = s.lines().enumerate();

        let header_len = match lines.next() {
            None => return Err(format!("{} is empty", bdc)),

            Some((_, line)) => match line.trim().parse::<usize>() {
                Err(_) => {
                    return Err(format!(
                        "Expected integer at first line in {}\n\tfound {} instead",
                        bdc, line
                    ));
                }

                Ok(n) => n,
            },
        };

        lines.by_ref().take(header_len).try_for_each(|(i, line)| {
            if let Some((label, data)) = line.split_once(':') {
                if let Some(text) = data.strip_suffix('\"') {
                    if labels.keys().any(|k| k == label) || externs.iter().any(|k| k == label) {
                        return Err(format!(
                            "Found label redefinition in {} at line {}\n\t{}",
                            bdc,
                            i + 1,
                            label
                        ));
                    }

                    if data_labels
                        .insert(label.to_owned(), (buf.len() >> 2) as u32)
                        .is_some()
                    {
                        return Err(format!(
                            "Found label redefinition in {} at line {}\n\t{}",
                            bdc,
                            i + 1,
                            label
                        ));
                    }

                    buf.extend(text.bytes());
                    buf.push(0);
                    while buf.len() & 3 != 0 {
                        buf.push(0);
                    }
                } else {
                    if labels.keys().any(|k| k == label) || externs.iter().any(|k| k == label) {
                        return Err(format!(
                            "Found label redefinition in {} at line {}\n\t{}",
                            bdc,
                            i + 1,
                            label
                        ));
                    }

                    if data_labels
                        .insert(label.to_owned(), (buf.len() >> 2) as u32)
                        .is_some()
                    {
                        return Err(format!(
                            "Found label redefinition in {} at line {}\n\t{}",
                            bdc,
                            i + 1,
                            label
                        ));
                    }

                    data.split(',').try_for_each(|word| {
                        match word.parse::<u32>() {
                            Err(_) => {
                                return Err(format!(
                                    "Expected integer as argument at line {}\n\tfound {} instead",
                                    i + 1,
                                    word
                                ))
                            }

                            Ok(val) => buf.extend(val.to_le_bytes().into_iter()),
                        }
                        Ok(())
                    })?;
                }
                Ok(())
            } else if let Some((label, line_number)) = line.split_once(' ') {
                if data_labels.keys().any(|k| k == label) || externs.iter().any(|k| k == label) {
                    return Err(format!(
                        "Found label redefinition in {} at line {}\n\t{}",
                        bdc,
                        i + 1,
                        label
                    ));
                }

                match line_number.parse::<u32>() {
                    Err(_) => {
                        return Err(format!(
                            "Expected integer at line {} in {}, found {} instead",
                            i + 1,
                            bdc,
                            line_number
                        ))
                    }

                    Ok(val) => {
                        if labels.insert(label.to_owned(), val + offset).is_some() {
                            return Err(format!(
                                "Found label redefinition in {} at line {}\n\t{}",
                                bdc,
                                i + 1,
                                label
                            ));
                        }
                    }
                }
                Ok(())
            } else {
                if !externs.iter().any(|e| e == line) {
                    externs.push(line.to_owned());
                }
                Ok(())
            }
        })?;

        offset += lines.count() as u32;

        Ok(())
    })?;

    externs.into_iter().try_for_each(|ext| {
        if !labels.contains_key(ext.as_str()) && !data_labels.contains_key(ext.as_str()) {
            return Err(format!("EXTERN label {} not defined in object files", ext));
        }
        Ok(())
    })?;

    for byte in ((buf.len() as u32) >> 2).to_be_bytes() {
        buf.insert(0, byte);
    }

    breadcrumbs.into_iter().try_for_each(|bdc| {
        let s = match fs::read_to_string(bdc) {
            Err(why) => return Err(why.to_string()),

            Ok(s) => s,
        };

        let mut lines = s.lines();

        let header_len = match lines.next() {
            None => return Err(format!("{} is empty", bdc)),

            Some(line) => match line.trim().parse::<usize>() {
                Err(_) => {
                    return Err(format!(
                        "Expected integer at first line in {}\n\tfound {} instead",
                        bdc, line
                    ));
                }

                Ok(n) => n,
            },
        };

        lines.skip(header_len)
            .enumerate()
            .try_for_each(|(i, line)| {
                let mut tokens = line.split_whitespace();
                if let Some(token) = tokens.next() {
                    if let Ok(_) = PseudoOps::from_str(token) {
                        return Err(format!("Found non-parsed pseudoinstruction during linking at line {} in {}", i + 1, bdc));
                    } else if let Ok(op) = OpCodes::from_str(token) {
                        match tokens.next() {
                            None => return Err(format!("Expected argument at line {} in {}", i + 1, bdc)),

                            Some(arg) => match op {
                                OpCodes::IRQ => match arg.parse::<u8>() {
                                    Err(_) => return Err(format!("Expected integer at line {}\n\tfound {} instead", i + 1, arg)),

                                    Ok(irq_type) => match tokens.next() {
                                        Some(label) => match irq_type {
                                            1..=2 => match labels.get(label) {
                                                None => match data_labels.get(label) {
                                                    None => return Err(format!("Missing declaration for label {} used at line {} in {}", label, i + 1, bdc)),

                                                    Some(field) => buf.extend(((irq_type as u32) << 25 | field).to_le_bytes().into_iter()),
                                                }
                                                Some(field) => buf.extend(((irq_type as u32) << 25 | field).to_le_bytes().into_iter()),
                                            }
                                            3 => match u32::from_str_radix(label, 2) {
                                                Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token)),

                                                Ok(field) => buf.extend((3 << 25 | field).to_le_bytes().into_iter()),
                                            }
                                            0 | 4 => return Err(format!("Unexpected argument at line {}\n\t{}", i + 1, label)),
                                            _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, arg)),
                                        }
                                        None => match irq_type {
                                            0 | 4 => buf.extend(((irq_type as u32) >> 2).to_le_bytes().into_iter()),
                                            1..=3 => return Err(format!("Expected label at line {}", i + 1)),
                                            _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                        }
                                    }
                                }
                                _ => match labels.get(arg) {
                                    None => match data_labels.get(arg) {
                                        None => return Err(format!("Label {} used at line {} in {} not defined in object files", arg, i + 1, bdc)),

                                        Some(field) => buf.extend(((op as u32) << 27 | 1 << 25 | field).to_le_bytes().into_iter()),
                                    }
                                    Some(field) => buf.extend(((op as u32) << 27 | field).to_le_bytes().into_iter()),
                                }
                            }
                        }
                        if let Some(token) = tokens.next() {
                            return Err(format!("Unexpected argument at line {}\n\t{}", i + 1, token))
                        }
                    } else {
                        return Err(format!("Expected instruction at line {} in {}\n\tfound {} instead", i + 1, bdc, token))
                    }
                }
            Ok(())
        })?;
        Ok(())
    })?;

    match fs::write(out.unwrap_or("a.fita"), buf) {
        Ok(_) => (),
        Err(why) => return Err(why.to_string()),
    };
    Ok(())
}
