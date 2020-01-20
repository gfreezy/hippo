use std::sync::Arc;

#[derive(Debug)]
pub struct CodeReader {
    code: Arc<Vec<u8>>,
    pc: usize,
}

impl CodeReader {
    pub fn new(code: Arc<Vec<u8>>) -> Self {
        CodeReader { code, pc: 0 }
    }
    pub fn read_u8(&mut self) -> Option<u8> {
        let code = self.code.get(self.pc as usize).cloned();
        self.pc += 1;
        code
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let byte1 = *self.code.get(self.pc as usize)? as u16;
        self.pc += 1;
        let byte2 = *self.code.get(self.pc as usize)? as u16;
        self.pc += 1;
        Some(byte1 << 8 | byte2)
    }
}
