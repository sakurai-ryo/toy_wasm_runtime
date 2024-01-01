use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::func_section::FunctionSectionNode;
use crate::exec::type_section::TypeSectionNode;

#[derive(Debug)]
pub enum SectionNode {
    TypeSectionNode,
    FunctionSectionNode,
}

impl SectionNode {
    pub fn create(section_id: u8) -> Result<SectionNode> {
        match section_id {
            // 0 => CustomSectionNode::new(),
            1 => Ok(SectionNode::TypeSectionNode),
            // 2 => ImportSectionNode::new(),
            3 => Ok(SectionNode::FunctionSectionNode),
            // 4 => TableSectionNode::new(),
            // 5 => MemorySectionNode::new(),
            // 6 => GlobalSectionNode::new(),
            // 7 => ExportSectionNode::new(),
            // 8 => StartSectionNode::new(),
            // 9 => ElementSectionNode::new(),
            // 10 => CodeSectionNode::new(),
            // 11 => DataSectionNode::new(),
            _ => Err(anyhow!("Invalid section id: {:?}", section_id)),
        }
    }

    pub fn load(&self, buf: &mut Buffer) -> Result<()> {
        match self {
            SectionNode::TypeSectionNode => {
                let mut type_section = TypeSectionNode::new();
                type_section.load(buf)?;
            }
            SectionNode::FunctionSectionNode => {
                let mut function_section = FunctionSectionNode::new();
                function_section.load(buf)?;
            }
        }
        Ok(())
    }
}
