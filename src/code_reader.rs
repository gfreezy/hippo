use crate::class::Method;
use std::sync::Arc;

#[derive(Debug)]
pub struct CodeReader {
    code: Arc<Vec<u8>>,
    pc: usize,
}

impl CodeReader {
    pub fn new(method: Method) -> Self {
        CodeReader {
            code: method.code(),
            pc: 0,
        }
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

    pub fn read_i32(&mut self) -> Option<i32> {
        let byte1 = *self.code.get(self.pc as usize)? as i32;
        self.pc += 1;
        let byte2 = *self.code.get(self.pc as usize)? as i32;
        self.pc += 1;
        let byte3 = *self.code.get(self.pc as usize)? as i32;
        self.pc += 1;
        let byte4 = *self.code.get(self.pc as usize)? as i32;
        self.pc += 1;
        Some((byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4)
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        let byte1 = *self.code.get(self.pc as usize)? as i16;
        self.pc += 1;
        let byte2 = *self.code.get(self.pc as usize)? as i16;
        self.pc += 1;
        Some(byte1 << 8 | byte2)
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }
}
