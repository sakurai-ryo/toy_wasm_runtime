use crate::exec::buffer::Buffer;
use crate::exec::code_section::CodeSectionNode;
use crate::exec::export_section::ExportSectionNode;
use crate::exec::func_section::FunctionSectionNode;
use crate::exec::type_section::TypeSectionNode;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub enum SectionNode {
    TypeSectionNode(TypeSectionNode),
    FunctionSectionNode(FunctionSectionNode),
    CodeSectionNode(CodeSectionNode),
    ExportSectionNode(ExportSectionNode),
}
impl SectionNode {
    pub fn create(section_id: u8) -> Result<SectionNode> {
        match section_id {
            // 0 => CustomSectionNode::new(),
            1 => Ok(SectionNode::TypeSectionNode(TypeSectionNode::new())),
            // 2 => ImportSectionNode::new(),
            3 => Ok(SectionNode::FunctionSectionNode(FunctionSectionNode::new())),
            // 4 => TableSectionNode::new(),
            // 5 => MemorySectionNode::new(),
            // 6 => GlobalSectionNode::new(),
            7 => Ok(SectionNode::ExportSectionNode(ExportSectionNode::new())),
            // 8 => StartSectionNode::new(),
            // 9 => ElementSectionNode::new(),
            10 => Ok(SectionNode::CodeSectionNode(CodeSectionNode::new())),
            // 11 => DataSectionNode::new(),
            _ => Err(anyhow!("Invalid section id: {:?}", section_id)),
        }
    }

    pub fn load(&mut self, _buf: &mut Buffer) -> Result<()> {
        match self {
            SectionNode::TypeSectionNode(t) => t.load(_buf),
            SectionNode::FunctionSectionNode(f) => f.load(_buf),
            SectionNode::CodeSectionNode(c) => c.load(_buf),
            SectionNode::ExportSectionNode(e) => e.load(_buf),
        }
    }
}
