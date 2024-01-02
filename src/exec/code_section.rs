use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::type_section::{NumType, ValType};

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    LocalGet = 0x20,
    LocalSet = 0x21,
    I32Const = 0x41,
    End = 0x0b,
}
impl Op {
    pub fn from_u8(value: u8) -> Option<Op> {
        match value {
            0x20 => Some(Op::LocalGet),
            0x21 => Some(Op::LocalSet),
            0x41 => Some(Op::I32Const),
            0x0b => Some(Op::End),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
            let op_byte = buf.read_byte()?;
            if op_byte == Op::End as u8 {
                break;
            }

            let opcode = Op::from_u8(op_byte).ok_or(anyhow!("Invalid opcode: {}", op_byte))?;
            let mut intrinsic = IntrinsicNode::new(opcode);
            intrinsic.load(buf)?;
            self.intrinsics.push(intrinsic);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum IntrinsicNode {
    LocalGetIntrinsicNode(LocalGetIntrinsicNode),
    LocalSetIntrinsicNode(LocalSetIntrinsicNode),
    I32ConstIntrinsicNode(I32ConstIntrinsicNode),
}
impl IntrinsicNode {
    pub fn new(opcode: Op) -> IntrinsicNode {
        match opcode {
            Op::I32Const => IntrinsicNode::I32ConstIntrinsicNode(I32ConstIntrinsicNode::new()),
            Op::LocalGet => IntrinsicNode::LocalGetIntrinsicNode(LocalGetIntrinsicNode::new()),
            Op::LocalSet => IntrinsicNode::LocalSetIntrinsicNode(LocalSetIntrinsicNode::new()),
            _ => panic!("Invalid opcode"), // TODO
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        match self {
            IntrinsicNode::I32ConstIntrinsicNode(i) => i.load(buf),
            IntrinsicNode::LocalGetIntrinsicNode(l) => l.load(buf),
            IntrinsicNode::LocalSetIntrinsicNode(l) => l.load(buf),
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct LocalGetIntrinsicNode {
    local_idx: u32,
}
impl Default for LocalGetIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl LocalGetIntrinsicNode {
    pub fn new() -> LocalGetIntrinsicNode {
        LocalGetIntrinsicNode { local_idx: 0 }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.local_idx = buf.read_u32()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LocalSetIntrinsicNode {
    local_idx: u32,
}
impl Default for LocalSetIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl LocalSetIntrinsicNode {
    pub fn new() -> LocalSetIntrinsicNode {
        LocalSetIntrinsicNode { local_idx: 0 }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.local_idx = buf.read_u32()?;
        Ok(())
    }
}
