use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;

type TypeIdx = u32;

#[derive(Debug, Clone)]
pub struct FunctionSectionNode {
    type_indices: Vec<TypeIdx>,
}
impl Default for FunctionSectionNode {
    fn default() -> Self {
        Self::new()
    }
}
impl FunctionSectionNode {
    pub fn new() -> FunctionSectionNode {
        FunctionSectionNode {
            type_indices: Vec::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let f = |buf: &mut Buffer| {
            let type_idx = buf.read_u32()?;
            Ok(type_idx)
        };

        self.type_indices = buf.read_vec::<TypeIdx>(Box::new(f))?;
        Ok(())
    }
}
