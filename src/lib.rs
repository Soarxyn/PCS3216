use std::{collections::HashMap, fs, io, str::FromStr};
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

pub fn assemble(in_asm: &str, out: &str) -> io::Result<()> {
    let mut symbol_table = HashMap::new();

    let s = fs::read_to_string(in_asm)?;
    s.lines().enumerate().for_each(|(i, line)| {
        let mut tokens = line.split_whitespace();
        if let Some(token) = tokens.next() {
            if token.ends_with(':') {
                symbol_table.insert(token.trim_end_matches(':'), i as u32);
            }
        }
    });
    let mut buf = Vec::new();
    s.lines().for_each(|line| {
        let mut tokens = line.split_whitespace();
        if let Some(token) = tokens.next() {
            let instr = token;
            if let Some(token) = tokens.next() {
                let op = OpCodes::from_str(instr).unwrap();
                let field = match symbol_table.contains_key(token) {
                    true => symbol_table[token],
                    false => u32::from_str_radix(token, 16).unwrap(),
                };
                buf.push((op as u32) << 27 | field);
            }
        }
    });
    fs::write(
        out,
        buf.into_iter()
            .map(|seq| seq.to_le_bytes())
            .flatten()
            .collect::<Vec<_>>(),
    )?;
    Ok(())
}
