use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::type_section::{NumType, ValType};

#[derive(Debug, PartialEq)]
enum Op {
    I32Const = 0x41,
    End = 0x0b,
}

pub struct CodeSectionNode {
    codes: Vec<CodeNode>,
}
impl Default for CodeSectionNode {
    fn default() -> Self {
        Self::new()
    }
}
impl CodeSectionNode {
    pub fn new() -> CodeSectionNode {
        CodeSectionNode { codes: Vec::new() }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let f = |buf: &mut Buffer| -> Result<CodeNode> {
            let mut code = CodeNode::new();
            code.load(buf)?;
            Ok(code)
        };

        self.codes = buf.read_vec::<CodeNode>(Box::new(f))?;
        Ok(())
    }
}

pub struct CodeNode {
    size: u32,
    func: FuncNode,
}

pub struct FuncNode {
    locals: Vec<LocalNode>,
    expr: ExprNode,
}

pub struct LocalNode {
    num: u32,
    val_type: ValType,
}
impl LocalNode {
    pub fn new() -> LocalNode {
        LocalNode {
            num: 0,
            val_type: ValType::NumType(NumType::I32(0)),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.num = buf.read_u32()?;
        self.val_type = ValType::from_u8(buf.read_byte()?).ok_or(anyhow!("Invalid value type"))?;
        Ok(())
    }
}
