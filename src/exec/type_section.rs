use anyhow::{anyhow, Result};

use crate::exec::buffer::Buffer;

const I32: u8 = 0x7f;
const I64: u8 = 0x7e;
const F32: u8 = 0x7d;
const F64: u8 = 0x7c;
#[derive(Debug, Clone)]
pub enum NumType {
    I32(u8),
    I64(u8),
    F32(u8),
    F64(u8),
}
impl NumType {
    pub fn from_u8(value: u8) -> Option<NumType> {
        match value {
            I32 => Some(NumType::I32(value)),
            I64 => Some(NumType::I64(value)),
            F32 => Some(NumType::F32(value)),
            F64 => Some(NumType::F64(value)),
            _ => None,
        }
    }
}

const FuncRef: u8 = 0x70;
const ExternRef: u8 = 0x6f;

#[derive(Debug, Clone)]
pub enum RefType {
    FuncRef(u8),
    ExternRef(u8),
}
impl RefType {
    pub fn from_u8(value: u8) -> Option<RefType> {
        match value {
            FuncRef => Some(RefType::FuncRef(value)),
            ExternRef => Some(RefType::ExternRef(value)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ValType {
    NumType(NumType),
    RefType(RefType),
}
impl ValType {
    pub fn from_u8(value: u8) -> Option<ValType> {
        match (NumType::from_u8(value), RefType::from_u8(value)) {
            (Some(num_type), _) => Some(ValType::NumType(num_type)),
            (_, Some(ref_type)) => Some(ValType::RefType(ref_type)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeSectionNode {
    func_types: Vec<FunctionTypeNode>,
}
impl Default for TypeSectionNode {
    fn default() -> Self {
        Self::new()
    }
}
impl TypeSectionNode {
    pub fn new() -> TypeSectionNode {
        TypeSectionNode {
            func_types: Vec::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let f = |buf: &mut Buffer| -> Result<FunctionTypeNode> {
            let mut func_type = FunctionTypeNode::new();
            func_type.load(buf)?;
            Ok(func_type)
        };
        self.func_types = buf.read_vec::<FunctionTypeNode>(Box::new(f))?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FunctionTypeNode {
    param_type: ResultTypeNode,
    result_type: ResultTypeNode,
}
impl Default for FunctionTypeNode {
    fn default() -> Self {
        Self::new()
    }
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
            return Err(anyhow!("Invalid function type: {}", byte));
        }

        self.param_type.load(buf)?;
        self.result_type.load(buf)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ResultTypeNode {
    val_types: Vec<ValType>,
}
impl Default for ResultTypeNode {
    fn default() -> Self {
        Self::new()
    }
}
impl ResultTypeNode {
    pub fn new() -> ResultTypeNode {
        ResultTypeNode {
            val_types: Vec::new(),
        }
    }

    pub fn load(&mut self, buf: &mut Buffer) -> Result<()> {
        let f = |buf: &mut Buffer| -> Result<ValType> {
            let byte = buf.read_byte()?;
            ValType::from_u8(byte).ok_or(anyhow!("Invalid value type: {}", byte))
        };
        self.val_types = buf.read_vec::<ValType>(Box::new(f))?;

        Ok(())
    }
}
