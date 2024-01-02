use anyhow::Result;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::errors::ExecError;
use crate::exec::buffer::Buffer;
use crate::exec::module::ModuleNode;

pub struct ExecInput {
    pub path: PathBuf,
    pub print: bool,
}

pub fn exec(input: ExecInput) -> Result<ModuleNode> {
    let file = fs::read(&input.path);
    let file_content = match file {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let p = input
                    .path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                return Err(ExecError::FileNotFound(p).into());
            }
            return Err(ExecError::Unknown.into());
        }
    };

    let mut buffer = Buffer::new(file_content);
    let mut module = ModuleNode::new();
    module.load(&mut buffer)?;

    if input.print {
        print_module(module.clone());
    }

    return Ok(module);
}

fn print_module(module: ModuleNode) {
    println!("Magic: {:#?}", module.magic);
    println!("Version: {:#?}", module.version);
    println!("Sections: {:#?}", module.sections);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_exec_const() {
        let path = PathBuf::from("examples/const.wasm");
        let result = exec(ExecInput { path, print: false });
        assert!(result.is_ok());

        let module = result.unwrap();

        assert!(module.sections.len() == 3);
    }

    #[test]
    fn test_exec_local_var() {
        let path = PathBuf::from("examples/local_var.wasm");
        let result = exec(ExecInput { path, print: false });
        assert!(result.is_ok());

        let module = result.unwrap();

        assert!(module.sections.len() == 3);
    }

    #[test]
    fn test_exec_add() {
        let path = PathBuf::from("examples/add.wasm");
        let result = exec(ExecInput { path, print: false });
        assert!(result.is_ok());

        let module = result.unwrap();

        assert!(module.sections.len() == 4);
    }

    #[test]
    fn test_exec_if() {
        let path = PathBuf::from("examples/if.wasm");
        let result = exec(ExecInput { path, print: false });
        assert!(result.is_ok());

        let module = result.unwrap();

        assert!(module.sections.len() == 4);
    }

    #[test]
    fn test_exec_loop() {
        let path = PathBuf::from("examples/loop.wasm");
        let result = exec(ExecInput { path, print: false });
        assert!(result.is_ok());

        let module = result.unwrap();

        assert!(module.sections.len() == 4);
    }
}
