use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;

#[derive(Debug, Clone)]
pub struct ExportSectionNode {
    exports: Vec<ExportNode>,
}
impl Default for ExportSectionNode {
    fn default() -> Self {
        Self::new()
    }
}
impl ExportSectionNode {
    pub fn new() -> ExportSectionNode {
        ExportSectionNode {
            exports: Vec::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let f = |buf: &mut Buffer| -> Result<ExportNode> {
            let mut export = ExportNode::new();
            export.load(buf)?;
            Ok(export)
        };
        self.exports = buf.read_vec::<ExportNode>(Box::new(f))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ExportNode {
    name: String,
    desc: ExportDescNode,
}
impl Default for ExportNode {
    fn default() -> Self {
        Self::new()
    }
}
impl ExportNode {
    pub fn new() -> ExportNode {
        ExportNode {
            name: String::new(),
            desc: ExportDescNode::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.name = buf.read_name()?;
        self.desc.load(buf)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ExportDescNode {
    tag: u8,
    index: u32,
}
impl Default for ExportDescNode {
    fn default() -> Self {
        Self::new()
    }
}
impl ExportDescNode {
    pub fn new() -> ExportDescNode {
        ExportDescNode { tag: 0, index: 0 }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.tag = buf.read_byte()?;
        self.index = buf.read_u32()?;
        Ok(())
    }
}
