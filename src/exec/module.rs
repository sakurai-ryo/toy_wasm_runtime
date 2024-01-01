use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::section::SectionNode;

#[derive(Debug)]
pub struct ModuleNode {
    /// https://webassembly.github.io/spec/core/binary/modules.html#binary-magic
    magic: Vec<u8>,

    /// https://webassembly.github.io/spec/core/binary/modules.html#binary-version
    version: Vec<u8>,

    /// https://webassembly.github.io/spec/core/binary/modules.html#sections
    sections: Vec<SectionNode>,
}

impl ModuleNode {
    pub fn new() -> ModuleNode {
        ModuleNode {
            magic: Vec::new(),
            version: Vec::new(),
            sections: Vec::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.magic = buf.read_bytes(4)?;
        if self.magic != vec![0x00, 0x61, 0x73, 0x6d] {
            return Err(anyhow!("Invalid wasm magic number"));
        }

        self.version = buf.read_bytes(4)?;

        loop {
            if buf.eof() {
                break;
            }

            // let section =
        }

        Ok(())
    }

    pub fn load_section(&mut self, buf: &mut Buffer) -> Result<()> {
        let section_id = buf.read_byte()?;
        let section_size = buf.read_u32()?;
        let section_buf = buf.read_buffer(section_size)?;

        let section = SectionNode::create(section_id)?;
        section.load(buf)?;
        self.sections.push(section);

        Ok(())
    }
}
