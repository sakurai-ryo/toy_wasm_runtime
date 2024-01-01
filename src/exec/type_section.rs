use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;
use crate::exec::section::SectionNode;

static I32: u8 = 0x7f;
static I64: u8 = 0x7e;
static F32: u8 = 0x7d;
static F64: u8 = 0x7c;
enum NumType {
    I32(u8),
    I64(u8),
    F32(u8),
    F64(u8),
}

static FuncRef: u8 = 0x70;
static ExternRef: u8 = 0x6f;
enum RefType {
    FuncRef(u8),
    ExternRef(u8),
}
enum ValType {
    NumType(NumType),
    RefType(RefType),
}

pub struct TypeSectionNode {
    func_types: Vec<FunctionTypeNode>,
}

impl TypeSectionNode {
    pub fn new() -> TypeSectionNode {
        TypeSectionNode {
            func_types: Vec::new(),
        }
    }
}

impl SectionNode for TypeSectionNode {
    fn load(&self, buf: &mut Buffer) -> Result<()> {
        self.func_types = buf.read_vec::<FunctionTypeNode>(|| {
            let mut func_type = FunctionTypeNode::new();
            func_type.load(buf);
            Ok(func_type)
        })?;
        Ok(())
    }
}

pub struct FunctionTypeNode {
    param_type: ResultTypeNode,
    result_type: ResultTypeNode,
}

impl FunctionTypeNode {
    pub fn new() -> FunctionTypeNode {
        FunctionTypeNode {
            param_type: ResultTypeNode::new(),
            result_type: ResultTypeNode::new(),
        }
    }

    pub fn tag(&self) -> u8 {
        0x60
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let byte = buf.read_byte()?;
        if byte != self.tag() {
            return Err(anyhow!("Invalid function type"));
        }

        self.param_type.load(buf);
        self.result_type.load(buf);

        Ok(())
    }
}

pub struct ResultTypeNode {
    val_types: Vec<ValType>,
}

impl ResultTypeNode {
    pub fn new() -> ResultTypeNode {
        ResultTypeNode {
            val_types: Vec::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) {
        self.val_types = buf.read_vec::<ValType>(|| {
            let byte = buf.read_byte()?;
            Ok(byte as ValType)
        });
    }
}
