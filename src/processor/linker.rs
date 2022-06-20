use super::assembler::{OpCodes, PseudoOps};
use std::{collections::HashMap, fs, str::FromStr};

pub fn link(breadcrumbs: Vec<&str>, out: Option<&str>) -> Result<(), String> {
    let mut offset = 0;
    let mut labels = HashMap::new();
    let mut data_labels = HashMap::new();
    let mut externs = Vec::new();
    let mut data = Vec::new();

    breadcrumbs.iter().try_for_each(|bdc| {
        let s = match fs::read_to_string(bdc) {
            Ok(s) => s,
            Err(why) => return Err(why.to_string()),
        };
        let mut lines = s.lines().enumerate();
        let first_line = match lines.next() {
            Some((_, line)) => line,
            None => return Err(format!("{} is empty", bdc)),
        };

        let mut values = first_line.split_whitespace();
        let n_words = match values.next() {
            Some(token) => match token.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(format!(
                        "Expected an integer as the first value in {} header, found {} instead",
                        bdc, token
                    ))
                }
            },
            None => return Err(format!("First line in {} is empty", bdc)),
        };
        let n_texts = match values.next() {
            Some(token) => match token.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(format!(
                        "Expected an integer as the second value in {} header, found {} instead",
                        bdc, token
                    ))
                }
            },
            None => return Err(format!("First line in {} contains only one value", bdc)),
        };
        let n_labels = match values.next() {
            Some(token) => match token.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(format!(
                        "Expected an integer as the third value in {} header, found {} instead",
                        bdc, token
                    ))
                }
            },
            None => return Err(format!("First line in {} contains only two values", bdc)),
        };
        let n_ext = match values.next() {
            Some(token) => match token.parse::<u32>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(format!(
                        "Expected an integer as the fourth value in {} header, found {} instead",
                        bdc, token
                    ))
                }
            },
            None => return Err(format!("First line in {} contains only three values", bdc)),
        };
        lines
            .by_ref()
            .take(n_words as usize)
            .try_for_each(|(i, line)| {
                match line.split_once(':') {
                    Some((label, words)) => {
                        if let Some(v) = data_labels.get(label) {
                            return Err(format!(
                                "Label {} at line {} in {} already defined in previous file ({})",
                                label,
                                i + 1,
                                bdc,
                                v
                            ));
                        }
                        data_labels.insert(label.to_owned(), (data.len() >> 2) as u32);
                        words.splitn(u32::MAX as usize, ',').try_for_each(|word| {
                            match word.parse::<u32>() {
                                Ok(val) => val
                                    .to_le_bytes()
                                    .into_iter()
                                    .for_each(|byte| data.push(byte)),
                                Err(_) => {
                                    return Err(format!(
                                        "Expected integer at line {} in {}, found {} instead",
                                        i + 1,
                                        bdc,
                                        word
                                    ))
                                }
                            }
                            Ok(())
                        })?
                    }
                    None => return Err(format!("Missing colon at line {} in {}", i + 1, bdc)),
                }
                Ok(())
            })?;

        lines
            .by_ref()
            .take(n_texts as usize)
            .try_for_each(|(i, line)| {
                match line.split_once(':') {
                    Some((label, text)) => {
                        if let Some(v) = data_labels.get(label) {
                            return Err(format!(
                                "Label {} at line {} in {} already defined in previous file ({})",
                                label,
                                i + 1,
                                bdc,
                                v
                            ));
                        }
                        data_labels.insert(label.to_owned(), (data.len() >> 2) as u32);
                        text.bytes().for_each(|byte| data.push(byte));
                    }
                    None => return Err(format!("Missing colon at line {} in {}", i + 1, bdc)),
                }
                Ok(())
            })?;

        lines
            .by_ref()
            .take(n_labels as usize)
            .try_for_each(|(i, line)| {
                match line.split_once(':') {
                    Some((label, line_number)) => {
                        if let Some(v) = labels.get(label) {
                            return Err(format!(
                                "Label {} at line {} in {} already defined in previous file ({})",
                                label,
                                i + 1,
                                bdc,
                                v
                            ));
                        }
                        match line_number.parse::<u32>() {
                            Ok(val) => labels.insert(label.to_owned(), val + offset as u32),
                            Err(_) => {
                                return Err(format!(
                                    "Expected integer at line {} in {}, found {} instead",
                                    i + 1,
                                    bdc,
                                    line_number
                                ))
                            }
                        }
                    }
                    None => return Err(format!("Missing colon at line {} in {}", i + 1, bdc)),
                };
                Ok(())
            })?;

        lines.by_ref().take(n_ext as usize).for_each(|(_, line)| {
            if !externs.iter().any(|e| e == line) {
                externs.push(line.to_owned());
            }
        });

        offset += lines.count();
        Ok(())
    })?;

    externs
        .into_iter()
        .try_for_each(|ext| match data_labels.contains_key(ext.as_str()) {
            true => Ok(()),
            false => match labels.contains_key(ext.as_str()) {
                true => Ok(()),
                false => Err(format!("EXTERN label {} not defined in object files", ext)),
            },
        })?;

    let mut buf = vec![offset as u32];

    breadcrumbs.into_iter().try_for_each(|bdc| {
        let s = match fs::read_to_string(bdc) {
            Ok(s) => s,
            Err(why) => return Err(why.to_string()),
        };
        let mut lines = s.lines();
        let first_line = match lines.next() {
            Some(line) => line,
            None => return Err(format!("{} is empty", bdc)),
        };
        lines.skip(first_line.split_whitespace().map(|tok| {
            match tok.parse::<u32>() {
                Ok(v) => v,
                Err(_) => unreachable!()
            }
        }).fold(0, |acc, val| acc + val) as usize)
            .enumerate()
            .try_for_each(|(i, line)| {
                let mut tokens = line.split_whitespace();
                match tokens.next() {
                    Some(token) => match OpCodes::from_str(token) {
                        Ok(op) => match tokens.next() {
                            Some(token) => {
                                match labels.get(token) {
                                    Some(field) => buf.push((op as u32) << 27 | field),
                                    None => match data_labels.get(token) {
                                        Some(field) => buf.push((op as u32) << 27 | 1 << 25 | field),
                                        None => return Err(format!("Label {} used at line {} in {} not defined in object files", token, i + 1, bdc))
                                    }
                                }
                            }
                            None => unreachable!()
                        },
                        Err(_) => match PseudoOps::from_str(token) {
                            Ok(psop) => match psop {
                                PseudoOps::HALT => buf.push(0),
                                PseudoOps::PRINT => match tokens.next() {
                                    Some(token) => {
                                        match labels.get(token) {
                                            Some(field) => buf.push(1 << 24| field),
                                            None => match data_labels.get(token) {
                                                Some(field) => buf.push(1 << 24| field),
                                                None => return Err(format!("Label {} used at line {} in {} not defined in object files", token, i + 1, bdc))
                                            }
                                        }
                                    }
                                    None => unreachable!()
                                }
                                PseudoOps::READ => match tokens.next() {
                                    Some(token) => {
                                        match labels.get(token) {
                                            Some(field) => buf.push(2 << 24| field),
                                            None => match data_labels.get(token) {
                                                Some(field) => buf.push(2 << 24| field),
                                                None => return Err(format!("Label {} used at line {} in {} not defined in object files", token, i + 1, bdc))
                                            }
                                        }
                                    }
                                    None => unreachable!()
                                }
                                PseudoOps::SET => {
                                    let field = match tokens.next() {
                                        Some(token) => match u32::from_str_radix(token, 2) {
                                            Ok(field) => field,
                                            Err(_) => unreachable!()
                                        }
                                        None => unreachable!()
                                    };
                                    buf.push(3 << 24 | field);
                                }
                                PseudoOps::CLEAR => buf.push(4 << 24),
                                _ => (),
                            },
                            Err(_) => unreachable!()
                        },
                    }
                    None => unreachable!(),
                }
            Ok(())
        })?;
        Ok(())
    })?;

    let mut buf = buf
        .into_iter()
        .map(|seq| seq.to_le_bytes())
        .flatten()
        .collect::<Vec<_>>();

    buf.extend(data);

    while buf.len() % 4 != 0 {
        buf.push(0);
    }

    match fs::write(out.unwrap_or("a.fita"), buf) {
        Ok(_) => (),
        Err(why) => return Err(why.to_string()),
    };
    Ok(())
}
