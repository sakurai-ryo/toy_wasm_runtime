use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::type_section::{NumType, ValType};

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
    end_op: Option<Op>,
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
            end_op: None,
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        loop {
            let op_byte = buf.read_byte()?;
            if op_byte == Op::End as u8 || op_byte == Op::Else as u8 {
                self.end_op = Op::from_u8(op_byte);
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

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    LocalGet = 0x20,
    LocalSet = 0x21,
    I32Const = 0x41,
    I32Eqa = 0x45,
    I32LtS = 0x48,
    I32GeS = 0x4e,
    I32Add = 0x6a,
    I32Rems = 0x6f,
    If = 0x04,
    Else = 0x05,
    Block = 0x02,
    Loop = 0x03,
    Br = 0x0c,
    BrIf = 0x0d,
    End = 0x0b,
}
impl Op {
    pub fn from_u8(value: u8) -> Option<Op> {
        match value {
            0x20 => Some(Op::LocalGet),
            0x21 => Some(Op::LocalSet),
            0x41 => Some(Op::I32Const),
            0x45 => Some(Op::I32Eqa),
            0x48 => Some(Op::I32LtS),
            0x4e => Some(Op::I32GeS),
            0x6a => Some(Op::I32Add),
            0x6f => Some(Op::I32Rems),
            0x04 => Some(Op::If),
            0x05 => Some(Op::Else),
            0x02 => Some(Op::Block),
            0x03 => Some(Op::Loop),
            0x0c => Some(Op::Br),
            0x0d => Some(Op::BrIf),
            0x0b => Some(Op::End),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IntrinsicNode {
    LocalGetIntrinsicNode(LocalGetIntrinsicNode),
    LocalSetIntrinsicNode(LocalSetIntrinsicNode),
    I32ConstIntrinsicNode(I32ConstIntrinsicNode),
    I32EqaIntrinsicNode(I32EqaIntrinsicNode),
    I32LtSIntrinsicNode(I32LtSIntrinsicNode),
    I32GeSIntrinsicNode(I32GeSIntrinsicNode),
    I32AddIntrinsicNode(I32AddIntrinsicNode),
    I32RemsIntrinsicNode(I32RemsIntrinsicNode),
    IfIntrinsicNode(IfIntrinsicNode),
    BlockIntrinsicNode(BlockIntrinsicNode),
    LoopIntrinsicNode(LoopIntrinsicNode),
    BrIntrinsicNode(BrIntrinsicNode),
    BrIfIntrinsicNode(BrIfIntrinsicNode),
}
impl IntrinsicNode {
    pub fn new(opcode: Op) -> IntrinsicNode {
        match opcode {
            Op::I32Const => IntrinsicNode::I32ConstIntrinsicNode(I32ConstIntrinsicNode::new()),
            Op::LocalGet => IntrinsicNode::LocalGetIntrinsicNode(LocalGetIntrinsicNode::new()),
            Op::LocalSet => IntrinsicNode::LocalSetIntrinsicNode(LocalSetIntrinsicNode::new()),
            Op::I32Eqa => IntrinsicNode::I32EqaIntrinsicNode(I32EqaIntrinsicNode::new()),
            Op::I32LtS => IntrinsicNode::I32LtSIntrinsicNode(I32LtSIntrinsicNode::new()),
            Op::I32GeS => IntrinsicNode::I32GeSIntrinsicNode(I32GeSIntrinsicNode::new()),
            Op::I32Add => IntrinsicNode::I32AddIntrinsicNode(I32AddIntrinsicNode::new()),
            Op::I32Rems => IntrinsicNode::I32RemsIntrinsicNode(I32RemsIntrinsicNode::new()),
            Op::If => IntrinsicNode::IfIntrinsicNode(IfIntrinsicNode::new()),
            Op::Block => IntrinsicNode::BlockIntrinsicNode(BlockIntrinsicNode::new()),
            Op::Loop => IntrinsicNode::LoopIntrinsicNode(LoopIntrinsicNode::new()),
            Op::Br => IntrinsicNode::BrIntrinsicNode(BrIntrinsicNode::new()),
            Op::BrIf => IntrinsicNode::BrIfIntrinsicNode(BrIfIntrinsicNode::new()),
            _ => panic!("Invalid opcode"), // TODO
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        match self {
            IntrinsicNode::I32ConstIntrinsicNode(i) => i.load(buf),
            IntrinsicNode::LocalGetIntrinsicNode(l) => l.load(buf),
            IntrinsicNode::LocalSetIntrinsicNode(l) => l.load(buf),
            IntrinsicNode::I32EqaIntrinsicNode(_) => Ok(()),
            IntrinsicNode::I32LtSIntrinsicNode(_) => Ok(()),
            IntrinsicNode::I32GeSIntrinsicNode(_) => Ok(()),
            IntrinsicNode::I32AddIntrinsicNode(_) => Ok(()),
            IntrinsicNode::I32RemsIntrinsicNode(_) => Ok(()),
            IntrinsicNode::IfIntrinsicNode(i) => i.load(buf),
            IntrinsicNode::BlockIntrinsicNode(b) => b.load(buf),
            IntrinsicNode::LoopIntrinsicNode(l) => l.load(buf),
            IntrinsicNode::BrIntrinsicNode(b) => b.load(buf),
            IntrinsicNode::BrIfIntrinsicNode(b) => b.load(buf),
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

#[derive(Debug, Clone)]
pub struct I32EqaIntrinsicNode {}
impl Default for I32EqaIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl I32EqaIntrinsicNode {
    pub fn new() -> I32EqaIntrinsicNode {
        I32EqaIntrinsicNode {}
    }
}

#[derive(Debug, Clone)]
pub struct I32LtSIntrinsicNode {}
impl Default for I32LtSIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl I32LtSIntrinsicNode {
    pub fn new() -> I32LtSIntrinsicNode {
        I32LtSIntrinsicNode {}
    }
}

#[derive(Debug, Clone)]
pub struct I32GeSIntrinsicNode {}
impl Default for I32GeSIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl I32GeSIntrinsicNode {
    pub fn new() -> I32GeSIntrinsicNode {
        I32GeSIntrinsicNode {}
    }
}

#[derive(Debug, Clone)]
pub struct I32AddIntrinsicNode {}
impl Default for I32AddIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl I32AddIntrinsicNode {
    pub fn new() -> I32AddIntrinsicNode {
        I32AddIntrinsicNode {}
    }
}

#[derive(Debug, Clone)]
pub struct I32RemsIntrinsicNode {}
impl Default for I32RemsIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl I32RemsIntrinsicNode {
    pub fn new() -> I32RemsIntrinsicNode {
        I32RemsIntrinsicNode {}
    }
}

// In WebAssembly, S33 represents a signed 33-bit integer.
// However, for simplicity, representing it as a signed 32-bit integer here.
pub type S33 = i32;

#[derive(Debug, Clone)]
pub enum BlockType {
    Empty,
    ValType(ValType),
    S33(S33),
}
impl BlockType {
    pub fn from_u8(value: u8) -> Option<BlockType> {
        match value {
            0x40 => Some(BlockType::Empty),
            _ => {
                let val_type = ValType::from_u8(value)?;
                Some(BlockType::ValType(val_type))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfIntrinsicNode {
    block_type: BlockType,
    then_expr: ExprNode,
    else_expr: ExprNode,
}
impl Default for IfIntrinsicNode {
    fn default() -> Self {
        Self::new()
    }
}
impl IfIntrinsicNode {
    pub fn new() -> IfIntrinsicNode {
        IfIntrinsicNode {
            block_type: BlockType::Empty,
            then_expr: ExprNode::new(),
            else_expr: ExprNode::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let byte = buf.read_byte()?;
        self.block_type =
            BlockType::from_u8(byte).ok_or(anyhow!("Invalid block type: {}", byte))?;

        self.then_expr = ExprNode::new();
        self.then_expr.load(buf)?;

        if self.then_expr.end_op == Some(Op::Else) {
            self.else_expr = ExprNode::new();
            self.else_expr.load(buf)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BlockIntrinsicNode {
    block_type: BlockType,
    expr: ExprNode,
}
impl BlockIntrinsicNode {
    pub fn new() -> BlockIntrinsicNode {
        BlockIntrinsicNode {
            block_type: BlockType::Empty,
            expr: ExprNode::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let byte = buf.read_byte()?;
        self.block_type =
            BlockType::from_u8(byte).ok_or(anyhow!("Invalid block type: {}", byte))?;

        self.expr = ExprNode::new();
        self.expr.load(buf)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoopIntrinsicNode {
    block_type: BlockType,
    expr: ExprNode,
}
impl LoopIntrinsicNode {
    pub fn new() -> LoopIntrinsicNode {
        LoopIntrinsicNode {
            block_type: BlockType::Empty,
            expr: ExprNode::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let byte = buf.read_byte()?;
        self.block_type =
            BlockType::from_u8(byte).ok_or(anyhow!("Invalid block type: {}", byte))?;

        self.expr = ExprNode::new();
        self.expr.load(buf)?;

        Ok(())
    }
}

type LabelIdx = u32;

#[derive(Debug, Clone)]
pub struct BrIntrinsicNode {
    label_idx: LabelIdx,
}
impl BrIntrinsicNode {
    pub fn new() -> BrIntrinsicNode {
        BrIntrinsicNode { label_idx: 0 }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.label_idx = buf.read_u32()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BrIfIntrinsicNode {
    label_idx: LabelIdx,
}
impl BrIfIntrinsicNode {
    pub fn new() -> BrIfIntrinsicNode {
        BrIfIntrinsicNode { label_idx: 0 }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        self.label_idx = buf.read_u32()?;
        Ok(())
    }
}
