// memory.rs
pub struct Memory {
    data: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 0x10000] }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
    pub fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
    pub fn load_rom(&mut self, path: &str) {
        let bytes = std::fs::read(path).expect("ROM not found");
        self.data[..bytes.len()].copy_from_slice(&bytes);
    }
}