
pub const MEM_SIZE: usize = 1 << 10;

#[derive(Clone)]
pub struct MemoryCache {
    pub content: [u32; MEM_SIZE],
    pub msb: u32
}

impl MemoryCache {
    pub fn read(&self, addr: u32) -> u32 {
        self.content[(addr % MEM_SIZE as u32) as usize]
    }

    pub fn write(&mut self, addr: u32, val: u32) {
        self.content[(addr % MEM_SIZE as u32) as usize] = val;
    }

    pub fn in_range(&self, addr: u32) -> bool {
        (addr >> 10) == self.msb
    }
}
