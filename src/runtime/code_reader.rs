use crate::runtime::method::Method;
use std::sync::Arc;

#[derive(Debug)]
pub struct CodeReader {
    code: Arc<Vec<u8>>,
    method: Method,
    pc: usize,
}

impl CodeReader {
    pub fn new(method: Method) -> Self {
        CodeReader {
            code: method.code(),
            pc: 0,
            method,
        }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        let code = self.code.get(self.pc as usize).cloned();
        self.pc += 1;
        code
    }

    pub fn method(&mut self) -> Method {
        self.method.clone()
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let byte1 = *self.code.get(self.pc as usize)? as u16;
        self.pc += 1;
        let byte2 = *self.code.get(self.pc as usize)? as u16;
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
