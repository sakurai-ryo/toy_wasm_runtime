use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::type_section::TypeSectionNode;

pub trait SectionNode {
    fn create(section_id: u8) -> Box<dyn SectionNode> {
        match section_id {
            // 0 => CustomSectionNode::new(),
            1 => TypeSectionNode::new(),
            // 2 => ImportSectionNode::new(),
            3 => FunctionSectionNode::new(),
            // 4 => TableSectionNode::new(),
            // 5 => MemorySectionNode::new(),
            // 6 => GlobalSectionNode::new(),
            // 7 => ExportSectionNode::new(),
            // 8 => StartSectionNode::new(),
            // 9 => ElementSectionNode::new(),
            10 => CodeSectionNode::new(),
            // 11 => DataSectionNode::new(),
            _ => panic!("Invalid section id"),
        }
    }

    fn load(&self, buf: &mut Buffer) -> Result<()>;
}
