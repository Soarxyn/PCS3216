use super::assembler::{OpCodes, PseudoOps};
use pyo3::prelude::*;
use std::{collections::HashMap, fs, str::FromStr};

#[pyfunction]
pub fn link(breadcrumbs: Vec<&str>, out: Option<&str>) -> PyResult<(bool, String)> {
    let mut offset: u32 = 0;

    let mut labels = HashMap::new();
    let mut extern_labels = Vec::new();

    let mut buf = Vec::new();

    match breadcrumbs.iter().try_for_each(|bdc| {
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
                    if labels.keys().any(|k| k == label) {
                        return Err(format!(
                            "Found label redefinition in {} at line {}\n\t{}",
                            bdc,
                            i + 1,
                            label
                        ));
                    }

                    if labels
                        .insert(
                            label.to_owned(),
                            1 << 25
                                | match u32::try_from(buf.len() >> 2) {
                                    Err(_) => return Err("File too big!".to_owned()),

                                    Ok(v) => {
                                        if v.leading_zeros() < 7 {
                                            return Err("File too big!".to_owned());
                                        }

                                        v
                                    }
                                },
                        )
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
                    if labels.keys().any(|k| k == label) {
                        return Err(format!(
                            "Found label redefinition in {} at line {}\n\t{}",
                            bdc,
                            i + 1,
                            label
                        ));
                    }

                    if labels
                        .insert(
                            label.to_owned(),
                            1 << 25
                                | match u32::try_from(buf.len() >> 2) {
                                    Err(_) => return Err("File too big!".to_owned()),

                                    Ok(v) => {
                                        if v.leading_zeros() < 7 {
                                            return Err("File too big!".to_owned());
                                        }

                                        v
                                    }
                                },
                        )
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
                if labels.keys().any(|k| k == label) {
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
                if !extern_labels.iter().any(|e| e == line) {
                    extern_labels.push(line.to_owned());
                }
                Ok(())
            }
        })?;

        offset += match u32::try_from(lines.count()) {
            Err(_) => return Err("File too big!".to_owned()),

            Ok(v) => {
                if v.leading_zeros() < 7 {
                    return Err("File too big!".to_owned());
                }

                v
            }
        };

        Ok(())
    }) {
        Ok(()) => (),
        Err(why) => return Ok((false, why)),
    }

    match extern_labels.into_iter().try_for_each(|ext| {
        if !labels.contains_key(ext.as_str()) {
            return Err(format!("EXTERN label {} not defined in object files", ext));
        }
        Ok(())
    }) {
        Ok(()) => (),
        Err(why) => return Ok((false, why)),
    }

    for byte in match u32::try_from(buf.len() >> 2) {
        Err(_) => return Ok((false, "File too big!".to_owned())),

        Ok(v) => {
            if v.leading_zeros() < 7 {
                return Ok((false, "File too big!".to_owned()));
            }

            v
        }
    }
    .to_be_bytes()
    {
        buf.insert(0, byte);
    }

    match breadcrumbs.into_iter().try_for_each(|bdc| {
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
                                                None => return Err(format!("Missing declaration for label {} used at line {} in {}", label, i + 1, bdc)),

                                                Some(field) => {
                                                    if field.leading_zeros() < 5 {
                                                        return Err("File too big!".to_owned());
                                                    }

                                                    buf.extend((u32::from(irq_type) << 25 | field).to_le_bytes().into_iter());
                                                }
                                            }
                                            3 => match u32::from_str_radix(label, 2) {
                                                Err(_) => return Err(format!("Expected binary number as argument at line {}\n\tfound {} instead", i + 1, token)),

                                                Ok(field) => {
                                                    if field.leading_zeros() < 5 {
                                                        return Err("File too big!".to_owned());
                                                    }

                                                    buf.extend((3 << 25 | field).to_le_bytes().into_iter());
                                                }
                                            }
                                            0 | 4 => return Err(format!("Unexpected argument at line {}\n\t{}", i + 1, label)),
                                            _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, arg)),
                                        }
                                        None => match irq_type {
                                            0 | 4 => buf.extend((u32::from(irq_type >> 2)).to_le_bytes().into_iter()),
                                            1..=3 => return Err(format!("Expected label at line {}", i + 1)),
                                            _ => return Err(format!("Unknown IRQ type at line {}\n\t{}", i + 1, irq_type)),
                                        }
                                    }
                                }
                                _ => match labels.get(arg) {
                                    None => return Err(format!("Label {} used at line {} in {} not defined in object files", arg, i + 1, bdc)),

                                    Some(field) => {
                                        if field.leading_zeros() < 5 {
                                            return Err("File too big!".to_owned());
                                        }

                                        buf.extend((u32::from(op as u8) << 27 | field).to_le_bytes().into_iter());
                                    }
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
    }) {
        Ok(()) => (),
        Err(why) => return Ok((false, why)),
    }

    match fs::write(out.unwrap_or("a.fita"), buf) {
        Ok(_) => Ok((true, "Linking successful".to_owned())),
        Err(why) => Ok((false, why.to_string())),
    }
}
