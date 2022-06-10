pub trait MemoryMapped {
    fn read(&self, addr: u32) -> u32;
    fn write(&mut self, addr: u32, val: u32);
    fn in_range(&self, addr: u32) -> bool;
}

pub const MEM_SIZE: usize = 1 << 25;

macro_rules! map_memory {
    ($mem_name:ident, $msb:expr) => {
        pub struct $mem_name(pub [u32; MEM_SIZE]);

        impl MemoryMapped for $mem_name {
            fn read(&self, addr: u32) -> u32 {
                self.0[(addr % MEM_SIZE as u32) as usize]
            }

            fn write(&mut self, addr: u32, val: u32) {
                self.0[(addr % MEM_SIZE as u32) as usize] = val;
            }

            fn in_range(&self, addr: u32) -> bool {
                (addr >> 25) == $msb
            }
        }
    };
}

map_memory!(IMem, 0);
map_memory!(DMem, 1);
map_memory!(Stack, 2);
map_memory!(IOMem, 3);
