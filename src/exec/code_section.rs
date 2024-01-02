use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::type_section::{NumType, ValType};

#[derive(Debug, PartialEq)]
pub enum Op {
    I32Const = 0x41,
    End = 0x0b,
}
impl Op {
    pub fn from_u8(value: u8) -> Option<Op> {
        match value {
            0x41 => Some(Op::I32Const),
            0x0b => Some(Op::End),
            _ => None,
        }
    }
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
impl Default for CodeNode {
    fn default() -> Self {
        Self::new()
    }
}
impl CodeNode {
    pub fn new() -> CodeNode {
        CodeNode {
            size: 0,
            func: FuncNode::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.size = buf.read_u32()?;
        let mut func_buf = buf.read_buffer(self.size)?;
        self.func.load(&mut func_buf)?;
        Ok(())
    }
}

pub struct FuncNode {
    locals: Vec<LocalNode>,
    expr: ExprNode,
}
impl Default for FuncNode {
    fn default() -> Self {
        Self::new()
    }
}
impl FuncNode {
    pub fn new() -> FuncNode {
        FuncNode {
            locals: Vec::new(),
            expr: ExprNode::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let f = |buf: &mut Buffer| -> Result<LocalNode> {
            let mut local = LocalNode::new();
            local.load(buf)?;
            Ok(local)
        };
        self.locals = buf.read_vec::<LocalNode>(Box::new(f))?;

        self.expr.load(buf)?;

        Ok(())
    }
}

pub struct LocalNode {
    num: u32,
    val_type: ValType,
}
impl Default for LocalNode {
    fn default() -> Self {
        Self::new()
    }
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
        let val_type = buf.read_byte()?;
        self.val_type =
            ValType::from_u8(val_type).ok_or(anyhow!("Invalid value type: {}", val_type))?;
        Ok(())
    }
}

pub struct ExprNode {
    intrinsics: Vec<IntrinsicNode>,
}
impl Default for ExprNode {
    fn default() -> Self {
        Self::new()
    }
}
impl ExprNode {
    pub fn new() -> ExprNode {
        ExprNode {
            intrinsics: Vec::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        loop {
            let op = buf.read_byte()?;
            if op == Op::End as u8 {
                break;
            }

            let mut intrinsic = IntrinsicNode::new(Op::I32Const);
            intrinsic.load(buf)?;
            self.intrinsics.push(intrinsic);
        }

        Ok(())
    }
}

pub enum IntrinsicNode {
    I32ConstIntrinsicNode,
}
impl IntrinsicNode {
    pub fn new(opcode: Op) -> IntrinsicNode {
        match opcode {
            Op::I32Const => IntrinsicNode::I32ConstIntrinsicNode,
            _ => panic!("Invalid opcode"), // TODO
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        match self {
            IntrinsicNode::I32ConstIntrinsicNode => {
                let mut node = I32ConstIntrinsicNode::new();
                node.load(buf)?;
                Ok(())
            }
        }
    }
}

pub struct I32ConstIntrinsicNode {
    val: i32,
}
impl Default for I32ConstIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl I32ConstIntrinsicNode {
    pub fn new() -> I32ConstIntrinsicNode {
        I32ConstIntrinsicNode { val: 0 }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.val = buf.read_i32()?;
        Ok(())
    }
}
