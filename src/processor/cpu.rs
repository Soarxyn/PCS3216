use super::{memory::{MemoryCache, MEM_SIZE}, assembler::OpCodes};
use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;

#[pyclass]
#[derive(Clone, PartialEq, Copy)]
pub enum CPUState {
    IDLE,
    STEP,
    RUNNING,
    INPUT,
    OUTPUT
}

#[pyclass]
pub struct CPU {
    #[pyo3(get, set)] pub acc: u32,
    #[pyo3(get, set)] pub pc: u32,
    #[pyo3(get, set)] pub la: u32,
    #[pyo3(get, set)] pub sp: u32,
    #[pyo3(get, set)] pub saved_reg: u32,
    #[pyo3(get, set)] pub n: bool,
    #[pyo3(get, set)] pub z: bool,
    #[pyo3(get, set)] pub c: bool,
    #[pyo3(get, set)] pub v: bool,
    #[pyo3(get, set)] pub i: bool,
    #[pyo3(get, set)] pub state: CPUState,
    pub last_state: CPUState,
    pub mem: Vec<MemoryCache>,
}

#[pymethods]
impl CPU {

    #[new]
    pub fn create() -> Self {
        CPU {
            acc: 0,
            pc: 0,
            la: 0,
            sp: 0xC0000000,
            saved_reg: 0,
            n: false,
            z: false,
            c: false,
            v: false,
            i: false,
            state: CPUState::IDLE,
            last_state: CPUState::IDLE,
            mem: vec![
                MemoryCache{content: [0; MEM_SIZE], msb: 0},
                MemoryCache{content: [0; MEM_SIZE], msb: 1},
                MemoryCache{content: [0; MEM_SIZE], msb: 2},
                MemoryCache{content: [0; MEM_SIZE], msb: 3},
            ],
        }
    }

    pub fn cycle(&mut self) -> PyResult<()> {
        match self.state {
            CPUState::STEP => {
                let instr = self.read_memory(self.pc).expect("Error while reading memory");
                let halted = self.process_instruction(instr);
                self.pc += 1;

                if halted {
                    self.state = CPUState::IDLE;
                }

                Ok(())
            }

            CPUState::RUNNING => {
                loop {
                    let instr = self.read_memory(self.pc).expect("Error while reading memory");
                    let halted = self.process_instruction(instr);
                    self.pc += 1;

                    if halted {
                        break;
                    }
                }

                Ok(())
            }

            _ => { Ok(()) }
        }
    }

    pub fn feed_read(&mut self, val: u32) -> PyResult<()> {
        if self.state != CPUState::INPUT {
            return Err(PyTypeError::new_err(format!("Invalid Request: Trying to feed a read when not in read state.")));
        }

        self.write_memory(self.saved_reg, val)?;
        self.state = self.last_state;

        Ok(())
    }

    pub fn get_print(&mut self) -> PyResult<Vec<u8>> {
        if self.state != CPUState::OUTPUT {
            return Err(PyTypeError::new_err(format!("Invalid Request: Trying to obtain a write when not in read state.")));
        }

        let mut string_end = false;
        let mut result: Vec<u8> = Vec::new();

        while !string_end {
            let read_byte = self.read_memory(self.saved_reg).expect("Error while reading memory").to_le_bytes();

            for byte in read_byte {
                result.push(byte);

                if byte == 0x00 {
                    string_end = true;
                }
            }

            self.saved_reg += 1;
        }

        self.state = self.last_state;

        Ok(result)
    }

    pub fn process_instruction(&mut self, instr: u32) -> bool {
        let opcode: OpCodes = OpCodes::from_repr(((instr >> 27) & 0x1F) as u8).unwrap_or(OpCodes::ADD);
        let irq_field = instr >> 25 & 0x3;
        let mut argument = instr & 0x07FFFFFF;

        println!("{:?} {:?}", opcode, argument);

        match opcode {
            OpCodes::IRQ if irq_field == 0 && instr & 0x1 == 0 => {
                true
            }
            
            OpCodes::IRQ if irq_field == 0 && instr & 0x1 == 1 => {
                self.n = false;
                self.z = false;
                self.c = false;
                self.v = false;
                self.i = false;

                false
            }

            OpCodes::IRQ if irq_field == 1 => {
                self.saved_reg = argument & 0x1FFFFFF; /* Saved register has starting memory position to be read. */
                self.last_state = self.state;
                self.state = CPUState::OUTPUT;

                false
            }

            OpCodes::IRQ if irq_field == 2 => {
                self.saved_reg = argument & 0x1FFFFFF; /* Saved register has memory position to be overwritten. */
                self.last_state = self.state;
                self.state = CPUState::INPUT;

                false
            }

            OpCodes::IRQ if irq_field == 3 => {
                let set_flags = argument & 0x1F;

                self.i = (set_flags & 0x10) >> 4 == 1;
                self.z = (set_flags & 0x8) >> 3 == 1;
                self.n = (set_flags & 0x4) >> 2 == 1;
                self.c = (set_flags & 0x2) >> 1 == 1;
                self.v = (set_flags & 0x1) == 1;
                                
                false
            }

            OpCodes::LDA => {
                if self.i {
                    argument = self.read_memory(argument).expect("Error while reading memory");
                }

                self.acc = self.read_memory(argument).expect("Error while reading memory");

                false
            }

            OpCodes::STA => {
                if self.i {
                    argument = self.read_memory(argument).expect("Error while reading memory");
                }

                self.write_memory(argument, self.acc).expect("Error while writing memory");

                false
            }

            OpCodes::ADD => {
                let operand = self.read_memory(argument).expect("Error while reading memory");

                self.acc = self.acc.wrapping_add(operand);

                false
            }

            OpCodes::SUB => {
                let operand = self.read_memory(argument).expect("Error while reading memory");

                self.acc = self.acc.wrapping_sub(operand);

                false
            }

            OpCodes::MUL => {
                let operand = self.read_memory(argument).expect("Error while reading memory");

                self.acc = self.acc.wrapping_mul(operand);

                false
            }

            OpCodes::DIV => {
                let operand = self.read_memory(argument).expect("Error while reading memory");

                self.acc = self.acc.wrapping_div(operand);

                false
            }

            OpCodes::CMP => {
                let operand = self.read_memory(argument).expect("Error while reading memory");
                
                let result = self.acc.overflowing_sub(operand);

                self.n = result.0 >> 31 == 1;
                self.z = result.0 == 0;
                self.c = result.1;
                self.v = (self.acc & operand ^ result.0) >> 31 == 1;

                false
            }

            OpCodes::NEG => {
                self.acc = 0u32.wrapping_sub(self.acc);

                false
            }

            OpCodes::BEQ => {
                if self.z == true {
                    self.pc = argument - 1;
                }

                false
            }

            OpCodes::BGT => {
                if (self.z == false) && (self.v == self.n) {
                    self.pc = argument - 1;
                }

                false
            }

            OpCodes::BLT => {
                if self.v != self.n {
                    self.pc = argument - 1;
                }

                false
            }

            OpCodes::BHS => {
                if self.c == true {
                    self.pc = argument - 1;
                }

                false
            }

            OpCodes::BMI => {
                if self.n == true {
                    self.pc = argument - 1;
                }

                false
            }

            OpCodes::BVS => {
                if self.v == true {
                    self.pc = argument - 1;
                }

                false
            }

            OpCodes::BHI => {
                if self.c == true && self.z == false {
                    self.pc = argument - 1;
                }

                false
            }
            
            OpCodes::PSH => {
                let content = self.read_memory(argument).expect("Error while writing memory");
                self.write_memory(self.sp, content).expect("Error while writing memory");
                self.sp += 1;

                false
            }

            OpCodes::POP => {
                let content = self.read_memory(self.sp).expect("Error while reading memory");
                self.write_memory(argument, content).expect("Error while writing memory");
                self.sp -= 1;

                false
            }

            OpCodes::JAL => {
                self.la = self.pc;
                self.pc = argument - 1;

                false
            }
            
            OpCodes::JMP => {
                self.pc = argument - 1;

                false
            }

            OpCodes::AND => {
                let operand = self.read_memory(argument).expect("Error while reading memory");
                self.acc &= operand;

                false
            }

            OpCodes::ORR => {
                let operand = self.read_memory(argument).expect("Error while reading memory");
                self.acc |= operand;

                false
            }

            OpCodes::NOT => {
                self.acc = !self.acc;

                false
            }

            OpCodes::XOR => {
                let operand = self.read_memory(argument).expect("Error while reading memory");
                self.acc ^= operand;

                false
            }

            OpCodes::LSL => {
                self.acc <<= argument;

                false
            }

            OpCodes::LSR => {
                let lsr = (self.acc as u32) >> argument;
                self.acc = lsr as u32;

                false
            }

            OpCodes::ASL => {
                let msb = self.acc & 0x80000000;
                self.acc = ((self.acc << argument) & 0x7FFFFFFF) | msb;

                false
            }

            OpCodes::ASR => {
                let lsr = (self.acc as i32) >> argument;
                self.acc = lsr as u32;

                false
            }

            OpCodes::ROR => {
                self.acc = self.acc.rotate_right(argument);
                
                false
            }

            OpCodes::RCR => {
                let mask: u32 = 0xFFFFFFFF >> (33 - argument);
                let leading = (self.acc & mask) << (33 - argument);
                let saved = self.acc & (1 << (argument - 1));
                let carry_bit = if self.c { 1 << (32 - argument) } else { 0 }; 

                self.acc >>= argument | carry_bit | leading;
                self.c = saved != 0;


                false
            }
            
            OpCodes::CLZ => {
                self.write_memory(argument, self.acc.leading_zeros()).expect("Error while writing memory");

                false
            }

            OpCodes::RET => {
                self.pc = self.la;

                false
            }

            _ => { false }
        }
    }

    pub fn read_memory(&self, addr: u32) -> PyResult<u32> {
        for page in &self.mem {
            if page.in_range(addr) {
                return Ok(page.read(addr));
            }
        }
        
        Err(PyTypeError::new_err(format!("[CPU] Could not find memory address: {}", addr)))
    }

    pub fn write_memory(&mut self, addr: u32, val: u32) -> PyResult<()> {
        for page in &mut self.mem {
            if page.in_range(addr) {
                page.write(addr, val);
                return Ok(());
            }
        }
        Err(PyTypeError::new_err(format!("[CPU] Could not find memory address: {}", addr)))
    }

    pub fn write_many(&mut self, addr: u32, val: Vec<u32>) -> PyResult<()> {
        let mut current_addr = addr;
        
        for word in val {
            self.write_memory(current_addr, word)?;
            current_addr += 1;
        }

        Ok(())
    }
    
}
