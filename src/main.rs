use sisprog::processor::{assembler::assemble, linker::link};
use std::fs;

fn main() {
    match assemble("ex.qck", Some("ex.bdc")) {
        Ok(_) => match link(vec!["ex.bdc"], Some("ex.fita")) {
            Ok(_) => {
                match fs::read("ex.fita") {
                    Ok(bin) => {
                        let mut chunks = bin.chunks(4);
                        match chunks.next() {
                            Some(first_line) => {
                                let n_code = u32::from_le_bytes([
                                    first_line[0],
                                    first_line[1],
                                    first_line[2],
                                    first_line[3],
                                ]);
                                chunks.by_ref().take(n_code as usize).for_each(|chunk| {
                                    let line = u32::from_le_bytes([
                                        chunk[0], chunk[1], chunk[2], chunk[3],
                                    ]);
                                    let instr = line >> 27;
                                    println!("{:05b} {:027b}", instr, line % (1 << 27));
                                });
                                while let Some(chunk) = chunks.next() {
                                    let data = u32::from_le_bytes([
                                        chunk[0], chunk[1], chunk[2], chunk[3],
                                    ]);
                                    println!(" {:032X}", data);
                                }
                            }
                            None => println!("ex.fita is empty"),
                        }
                    }
                    Err(why) => println!("Read error: {}", why.to_string()),
                };
            }
            Err(why) => println!("Linking error: {}", why),
        },
        Err(why) => println!("Assembly error: {}", why),
    }
}
