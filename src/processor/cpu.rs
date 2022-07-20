use super::{
    assembler::OpCodes,
    memory::{MemoryCache, MEM_SIZE},
};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

pub static mut ACC: u32 = 0;
pub static mut PC: u32 = 0;
pub static mut LA: u32 = 0;
pub static mut SP: u32 = 0;
pub static mut SAVED_REG: u32 = 0;

pub static mut P: bool = false;
pub static mut N: bool = false;
pub static mut Z: bool = false;
pub static mut C: bool = false;
pub static mut V: bool = false;

pub static mut STATE: CPUState = CPUState::IDLE;
pub static mut LAST_STATE: CPUState = CPUState::IDLE;

pub static mut MEM: [MemoryCache; 4] = [
    MemoryCache {
        content: [0; MEM_SIZE],
        msb: 0,
    },
    MemoryCache {
        content: [0; MEM_SIZE],
        msb: 1,
    },
    MemoryCache {
        content: [0; MEM_SIZE],
        msb: 2,
    },
    MemoryCache {
        content: [0; MEM_SIZE],
        msb: 3,
    },
];

#[pyfunction]
pub unsafe fn get_state() -> PyResult<CPUState> {
    Ok(STATE)
}

#[pyfunction]
pub unsafe fn get_acc() -> PyResult<u32> {
    Ok(ACC)
}

#[pyfunction]
pub unsafe fn get_pc() -> PyResult<u32> {
    Ok(PC)
}

#[pyfunction]
pub unsafe fn get_la() -> PyResult<u32> {
    Ok(LA)
}

#[pyfunction]
pub unsafe fn get_sp() -> PyResult<u32> {
    Ok(SP)
}

#[pyfunction]
pub unsafe fn get_saved_reg() -> PyResult<u32> {
    Ok(SAVED_REG)
}

#[pyfunction]
pub unsafe fn get_p() -> PyResult<bool> {
    Ok(P)
}

#[pyfunction]
pub unsafe fn get_n() -> PyResult<bool> {
    Ok(N)
}

#[pyfunction]
pub unsafe fn get_z() -> PyResult<bool> {
    Ok(Z)
}

#[pyfunction]
pub unsafe fn get_c() -> PyResult<bool> {
    Ok(C)
}

#[pyfunction]
pub unsafe fn get_v() -> PyResult<bool> {
    Ok(V)
}

#[pyfunction]
pub unsafe fn cycle() -> PyResult<()> {
    match STATE {
        CPUState::STEP => {
            let instr = read_memory(PC).expect("Error while reading memory");
            let halted = process_instruction(instr);
            PC += 1;

            if halted {
                STATE = CPUState::IDLE;
            }

            Ok(())
        }

        CPUState::RUNNING => {
            loop {
                let instr = read_memory(PC).expect("Error while reading memory");
                let halted = process_instruction(instr);
                PC += 1;

                if halted {
                    STATE = CPUState::IDLE;
                    break;
                }
                if STATE == CPUState::INPUT || STATE == CPUState::OUTPUT {
                    break;
                }
            }

            Ok(())
        }

        _ => Ok(()),
    }
}

#[pyfunction]
pub unsafe fn read_memory(addr: u32) -> PyResult<u32> {
    for page in &MEM {
        if page.in_range(addr) {
            return Ok(page.read(addr));
        }
    }

    Err(PyTypeError::new_err(format!(
        "[CPU] Could not find memory address: {}",
        addr
    )))
}

#[pyfunction]
pub unsafe fn write_memory(addr: u32, val: u32) -> PyResult<()> {
    for page in &mut MEM {
        if page.in_range(addr) {
            page.write(addr, val);
            return Ok(());
        }
    }
    Err(PyTypeError::new_err(format!(
        "[CPU] Could not find memory address: {}",
        addr
    )))
}

#[pyfunction]
pub unsafe fn write_many(addr: u32, val: Vec<u32>) -> PyResult<()> {
    let mut current_addr = addr;

    for word in val {
        write_memory(current_addr, word)?;
        current_addr += 1;
    }

    Ok(())
}

pub unsafe fn process_instruction(instr: u32) -> bool {
    let opcode: OpCodes = OpCodes::from_repr(((instr >> 18) & 0x3FFF) as u16).unwrap_or(OpCodes::ADD);
    let irq_field = (instr >> 16) & 0x3;
    let mut argument = instr & 0x3FFFF;

    //println!("{:?} {:?}", opcode, argument);

    match opcode {
        OpCodes::IRQ if irq_field == 0 && instr & 0x1 == 0 => true,

        OpCodes::IRQ if irq_field == 0 && instr & 0x1 == 1 => {
            N = false;
            Z = false;
            C = false;
            V = false;
            P = false;

            false
        }

        OpCodes::IRQ if irq_field == 1 => {
            SAVED_REG = argument; /* Saved register has starting memory position to be read. */
            LAST_STATE = STATE;
            STATE = CPUState::OUTPUT;

            false
        }

        OpCodes::IRQ if irq_field == 2 => {
            SAVED_REG = argument & 0xFFFF | 0x10000; /* Saved register has memory position to be overwritten. */
            LAST_STATE = STATE;
            STATE = CPUState::INPUT;

            false
        }

        OpCodes::IRQ if irq_field == 3 => {
            let set_flags = argument & 0x1F;

            P = (set_flags & 0x10) >> 4 == 1;
            Z = (set_flags & 0x8) >> 3 == 1;
            N = (set_flags & 0x4) >> 2 == 1;
            C = (set_flags & 0x2) >> 1 == 1;
            V = (set_flags & 0x1) == 1;

            false
        }

        OpCodes::LDA => {
            if P {
                argument = read_memory(argument).expect("Error while reading memory");
            }

            ACC = read_memory(argument).expect("Error while reading memory");

            false
        }

        OpCodes::STA => {
            if P {
                argument = read_memory(argument).expect("Error while reading memory");
            }

            write_memory(argument, ACC).expect("Error while writing memory");

            false
        }

        OpCodes::ADD => {
            let operand = read_memory(argument).expect("Error while reading memory");

            ACC = ACC.wrapping_add(operand);

            false
        }

        OpCodes::SUB => {
            let operand = read_memory(argument).expect("Error while reading memory");

            ACC = ACC.wrapping_sub(operand);

            false
        }

        OpCodes::MUL => {
            let operand = read_memory(argument).expect("Error while reading memory");

            ACC = ACC.wrapping_mul(operand);

            false
        }

        OpCodes::DIV => {
            let operand = read_memory(argument).expect("Error while reading memory");

            ACC = ACC.wrapping_div(operand);

            false
        }

        OpCodes::CMP => {
            let operand = read_memory(argument).expect("Error while reading memory");

            let result = ACC.overflowing_sub(operand);

            N = result.0 >> 31 == 1;
            Z = result.0 == 0;
            C = result.1;
            V = (ACC & operand ^ result.0) >> 31 == 1;

            false
        }

        OpCodes::NEG => {
            ACC = 0u32.wrapping_sub(ACC);

            false
        }

        OpCodes::BEQ => {
            if Z == true {
                PC = argument - 1;
            }

            false
        }

        OpCodes::BGT => {
            if (Z == false) && (V == N) {
                PC = argument - 1;
            }

            false
        }

        OpCodes::BLT => {
            if V != N {
                PC = argument - 1;
            }

            false
        }

        OpCodes::BHS => {
            if C == true {
                PC = argument - 1;
            }

            false
        }

        OpCodes::BMI => {
            if N == true {
                PC = argument - 1;
            }

            false
        }

        OpCodes::BVS => {
            if V == true {
                PC = argument - 1;
            }

            false
        }

        OpCodes::BHI => {
            if C == true && Z == false {
                PC = argument - 1;
            }

            false
        }

        OpCodes::PSH => {
            let content = read_memory(argument).expect("Error while writing memory");
            write_memory(SP, content).expect("Error while writing memory");
            SP += 1;

            false
        }

        OpCodes::POP => {
            let content = read_memory(SP).expect("Error while reading memory");
            write_memory(argument, content).expect("Error while writing memory");
            SP -= 1;

            false
        }

        OpCodes::JAL => {
            LA = PC;
            PC = argument - 1;

            false
        }

        OpCodes::JMP => {
            PC = argument - 1;

            false
        }

        OpCodes::AND => {
            let operand = read_memory(argument).expect("Error while reading memory");
            ACC &= operand;

            false
        }

        OpCodes::ORR => {
            let operand = read_memory(argument).expect("Error while reading memory");
            ACC |= operand;

            false
        }

        OpCodes::NOT => {
            ACC = !ACC;

            false
        }

        OpCodes::XOR => {
            let operand = read_memory(argument).expect("Error while reading memory");
            ACC ^= operand;

            false
        }

        OpCodes::LSL => {
            let operand = read_memory(argument).expect("Error while reading memory");
            ACC <<= operand;

            false
        }

        OpCodes::LSR => {
            let operand = read_memory(argument).expect("Error while reading memory");
            let lsr = (ACC as u32) >> operand;
            ACC = lsr as u32;

            false
        }

        OpCodes::ASL => {
            let operand = read_memory(argument).expect("Error while reading memory");
            let msb = ACC & 0x80000000;
            ACC = ((ACC << operand) & 0x7FFFFFFF) | msb;

            false
        }

        OpCodes::ASR => {
            let operand = read_memory(argument).expect("Error while reading memory");
            let lsr = (ACC as i32) >> operand;
            ACC = lsr as u32;

            false
        }

        OpCodes::ROR => {
            let operand = read_memory(argument).expect("Error while reading memory");
            ACC = ACC.rotate_right(operand);

            false
        }

        OpCodes::RCR => {
            let operand = read_memory(argument).expect("Error while reading memory");
            let mask: u32 = 0xFFFFFFFF >> (33 - operand);
            let leading = (ACC & mask) << (33 - operand);
            let saved = ACC & (1 << (operand - 1));
            let carry_bit = if C { 1 << (32 - operand) } else { 0 };

            ACC >>= operand | carry_bit | leading;
            C = saved != 0;

            false
        }

        OpCodes::CLZ => {
            write_memory(argument, ACC.leading_zeros()).expect("Error while writing memory");

            false
        }

        OpCodes::RET => {
            PC = LA;

            false
        }

        _ => false,
    }
}

#[pyfunction]
pub unsafe fn execute(pc: u32, step: bool) -> PyResult<()> {
    PC = pc;
    STATE = if step { CPUState::STEP } else { CPUState::RUNNING };    
    Ok(())
}

#[pyfunction]
pub unsafe fn feed_read(val: u32) -> PyResult<()> {
    if STATE != CPUState::INPUT {
        return Err(PyTypeError::new_err(format!(
            "Invalid Request: Trying to feed a read when not in read state."
        )));
    }

    write_memory(SAVED_REG, val)?;
    STATE = LAST_STATE;

    Ok(())
}

#[pyfunction]
pub unsafe fn get_print() -> PyResult<Vec<u8>> {
    if STATE != CPUState::OUTPUT {
        return Err(PyTypeError::new_err(format!(
            "Invalid Request: Trying to obtain a write when not in read state."
        )));
    }

    let mut string_end = false;
    let mut result: Vec<u8> = Vec::new();

    while !string_end {
        let read_byte = read_memory(SAVED_REG)
            .expect("Error while reading memory")
            .to_le_bytes();

        for byte in read_byte {
            result.push(byte);

            if byte == 0x00 {
                string_end = true;
            }
        }

        SAVED_REG += 1;
    }

    STATE = LAST_STATE;

    Ok(result)
}

#[pyclass]
#[derive(Clone, PartialEq, Copy)]
pub enum CPUState {
    IDLE,
    STEP,
    RUNNING,
    INPUT,
    OUTPUT,
}
