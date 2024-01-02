use anyhow::Result;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::errors::ExecError;
use crate::exec::buffer::Buffer;
use crate::exec::module::ModuleNode;

pub fn exec(path: PathBuf) -> Result<ModuleNode> {
    let file = fs::read(&path);
    let file_content = match file {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let p = path.file_name().unwrap().to_string_lossy().into_owned();
                return Err(ExecError::FileNotFound(p).into());
            }
            return Err(ExecError::Unknown.into());
        }
    };

    let mut buffer = Buffer::new(file_content);
    let mut module = ModuleNode::new();
    module.load(&mut buffer)?;

    println!("{:#?}", module);

    return Ok(module);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_exec() {
        let path = PathBuf::from("examples/const.wasm");
        let result = exec(path);
        assert!(result.is_ok());

        let module = result.unwrap();

        assert!(module.sections.len() == 3);
    }
}
