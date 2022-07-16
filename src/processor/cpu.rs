use super::{memory::{MemoryCache, MEM_SIZE}, assembler::OpCodes};
use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;

#[pyclass]
#[derive(Clone, PartialEq, Copy)]
pub enum CPUState {
    IDLE,
    STEP,
    RUNNING,
    READ,
    WRITE
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
                self.pc += 4;

                if halted {
                    self.state = CPUState::IDLE;
                }

                Ok(())
            }

            CPUState::RUNNING => {
                loop {
                    let instr = self.read_memory(self.pc).expect("Error while reading memory");
                    let halted = self.process_instruction(instr);
                    self.pc += 4;

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
        if self.state != CPUState::READ {
            return Err(PyTypeError::new_err(format!("Invalid Request: Trying to feed a read when not in read state.")));
        }

        self.write_memory(self.saved_reg, val)?;
        self.state = self.last_state;

        Ok(())
    }

    pub fn get_print(&mut self) -> PyResult<u32> {
        if self.state != CPUState::WRITE {
            return Err(PyTypeError::new_err(format!("Invalid Request: Trying to obtain a write when not in read state.")));
        }

        self.state = self.last_state;

        Ok(self.saved_reg)
    }

    pub fn process_instruction(&mut self, instr: u32) -> bool {
        let opcode: OpCodes = OpCodes::from_repr(((instr >> 27) & 0x1F) as u8).unwrap_or(OpCodes::ADD);
        let argument = instr & 0x07FFFFFF;

        println!("{:?} {:?}", opcode, argument);

        match opcode {
            OpCodes::IRQ if instr >> 27 == 0 && instr & 0x1 == 0 => {
                true
            }
            
            OpCodes::IRQ if instr >> 27 == 0 && instr & 0x1 == 1 => {
                self.n = false;
                self.z = false;
                self.c = false;
                self.v = false;

                false
            }

            OpCodes::IRQ if instr >> 27 == 1 => {
                self.saved_reg = self.read_memory(argument).expect("Error while reading memory"); /* Saved register has memory content to be printed. */
                self.last_state = self.state;
                self.state = CPUState::READ;

                false
            }

            OpCodes::IRQ if instr >> 27 == 2 => {
                self.saved_reg = argument; /* Saved register has memory position to be overwritten. */
                self.last_state = self.state;
                self.state = CPUState::WRITE;

                false
            }

            OpCodes::IRQ if instr >> 27 == 3 => {
                let set_flags = argument & 0xF;

                self.z = (set_flags & 0x8) >> 3 == 1;
                self.n = (set_flags & 0x4) >> 2 == 1;
                self.c = (set_flags & 0x2) >> 1 == 1;
                self.v = (set_flags & 0x1) == 1;
                                
                false
            }

            OpCodes::LDA => {
                self.acc = self.read_memory(argument).expect("Error while reading memory");

                false
            }

            OpCodes::STA => {
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
                    self.pc = argument;
                }

                false
            }

            OpCodes::BGT => {
                if (self.z == false) && (self.v == self.n) {
                    self.pc = argument;
                }

                false
            }

            OpCodes::BLT => {
                if self.v != self.n {
                    self.pc = argument;
                }

                false
            }

            OpCodes::BHS => {
                if self.c == true {
                    self.pc = argument;
                }

                false
            }

            OpCodes::BMI => {
                if self.n == true {
                    self.pc = argument;
                }

                false
            }

            OpCodes::BVS => {
                if self.v == true {
                    self.pc = argument;
                }

                false
            }

            OpCodes::BHI => {
                if self.c == true && self.z == false {
                    self.pc = argument;
                }

                false
            }
            
            OpCodes::PSH => {
                self.write_memory(self.sp, self.acc).expect("Error while writing memory");
                self.sp += 4;

                false
            }

            OpCodes::POP => {
                self.acc = self.read_memory(self.sp).expect("Error while reading memory");
                self.sp -= 4;

                false
            }

            OpCodes::JAL => {
                self.la = self.pc + 4;
                self.pc = argument;

                false
            }
            
            OpCodes::JMP => {
                self.pc = argument;

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

            OpCodes::ASL => todo!(),
            OpCodes::ASR => todo!(),
            OpCodes::ROL => todo!(),
            OpCodes::ROR => todo!(),
            OpCodes::RCL => todo!(),
            OpCodes::RCR => todo!(),

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
            current_addr += 4;
        }

        Ok(())
    }
    
}
