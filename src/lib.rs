extern crate strum;

#[macro_use]
extern crate strum_macros;

pub mod processor;

use processor::{
    assembler::{assemble, OpCodes},
    cpu::{CPUState, cycle, read_memory, write_many, write_memory, get_acc, get_c, get_la, get_n, get_p, get_pc, get_print, feed_read, get_saved_reg, get_sp, get_state,  get_v, get_z, execute},
    linker::link,
};
use std::fs;

#[pyfunction]
fn print_debug(fita: &str) -> PyResult<()> {
    match fs::read(fita) {
        Ok(bin) => {
            let mut chunks = bin.chunks_exact(4);
            match chunks.next() {
                Some(first_line) => {
                    let n_data = u32::from_le_bytes([
                        first_line[0],
                        first_line[1],
                        first_line[2],
                        first_line[3],
                    ]);
                    chunks.by_ref().take(n_data as usize).for_each(|chunk| {
                        let data = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                        println!("{:032X}", data);
                    });
                    while let Some(chunk) = chunks.next() {
                        let line = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                        let instr = line >> 18;
                        println!(
                            "{:?}  {:018b}",
                            OpCodes::from_repr(instr as u16).unwrap(),
                            line % (1 << 18)
                        );
                    }
                }
                None => println!("ex.fita is empty"),
            }
        }
        Err(why) => println!("Read error: {}", why.to_string()),
    }
    Ok(())
}

#[pyfunction]
fn parse_binary(bin_file: &str) -> PyResult<(u32, u32, Vec<u32>, Vec<u32>)> {
    let mut n_data = 0;
    let mut n_instr = 0;
    let mut data: Vec<u32> = Vec::new();
    let mut instr: Vec<u32> = Vec::new();

    match fs::read(bin_file) {
        Ok(bin) => {
            let mut chunks = bin.chunks_exact(4);

            match chunks.next() {
                Some(first_line) => {
                    n_data = u32::from_le_bytes([
                        first_line[0],
                        first_line[1],
                        first_line[2],
                        first_line[3],
                    ]);

                    chunks.by_ref().take(n_data as usize).for_each(|chunk| {
                        data.push(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
                    });

                    while let Some(chunk) = chunks.next() {
                        n_instr += 1;
                        instr.push(u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
                    }
                }

                None => println!("Trying to parse empty binary file."),
            }
        }

        Err(why) => println!("Read error: {}", why.to_string()),
    }

    Ok((n_data, n_instr, data, instr))
}

use pyo3::prelude::*;

#[pymodule]
fn sisprog(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(assemble, m)?)?;
    m.add_function(wrap_pyfunction!(link, m)?)?;
    m.add_function(wrap_pyfunction!(print_debug, m)?)?;
    m.add_function(wrap_pyfunction!(parse_binary, m)?)?;
    m.add_function(wrap_pyfunction!(cycle, m)?)?;
    m.add_function(wrap_pyfunction!(read_memory, m)?)?;
    m.add_function(wrap_pyfunction!(write_many, m)?)?;
    m.add_function(wrap_pyfunction!(write_memory, m)?)?;
    m.add_function(wrap_pyfunction!(get_acc, m)?)?;
    m.add_function(wrap_pyfunction!(get_c, m)?)?;
    m.add_function(wrap_pyfunction!(get_la, m)?)?;
    m.add_function(wrap_pyfunction!(get_n, m)?)?;
    m.add_function(wrap_pyfunction!(get_p, m)?)?;
    m.add_function(wrap_pyfunction!(get_pc, m)?)?;
    m.add_function(wrap_pyfunction!(get_print, m)?)?;
    m.add_function(wrap_pyfunction!(feed_read, m)?)?;
    m.add_function(wrap_pyfunction!(get_saved_reg, m)?)?;
    m.add_function(wrap_pyfunction!(get_sp, m)?)?;
    m.add_function(wrap_pyfunction!(get_state, m)?)?;
    m.add_function(wrap_pyfunction!(get_v, m)?)?;
    m.add_function(wrap_pyfunction!(get_z, m)?)?;
    m.add_function(wrap_pyfunction!(execute, m)?)?;
    m.add_class::<CPUState>()?;
    Ok(())
}
