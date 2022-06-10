use super::memory::{DMem, IMem, IOMem, MemoryMapped, Stack, MEM_SIZE};

pub struct CPU {
    pub acc: u32,
    pub pc: u32,
    pub mem: Vec<Box<dyn MemoryMapped>>,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            acc: 0,
            pc: 0,
            mem: vec![
                Box::new(IMem([0; MEM_SIZE])),
                Box::new(DMem([0; MEM_SIZE])),
                Box::new(Stack([0; MEM_SIZE])),
                Box::new(IOMem([0; MEM_SIZE])),
            ],
        }
    }
}

impl CPU {
    pub fn read_memory(&self, addr: u32) -> Result<u32, String> {
        for page in &self.mem {
            if page.in_range(addr) {
                return Ok(page.read(addr));
            }
        }
        Err(format!("[CPU] Could not find memory address: {}", addr))
    }

    pub fn write_memory(&mut self, addr: u32, val: u32) -> Result<(), String> {
        for page in &mut self.mem {
            if page.in_range(addr) {
                page.write(addr, val);
                return Ok(());
            }
        }
        Err(format!("[CPU] Could not find memory address: {}", addr))
    }
}
