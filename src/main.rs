extern crate libc;
extern crate llvm_sys;

use std::io::{self, Write};
use std::ptr;

mod parser;

macro_rules! c_str {
    ($string:expr) => (concat!($string, '\0').as_ptr() as *const ::libc::c_char)
}

// TODO(tsion): Use the readline library.
fn prompt(line: &mut String) -> io::Result<usize> {
    print!("rl> ");
    try!(io::stdout().flush());
    io::stdin().read_line(line)
}

fn compile_expr(expr: parser::Expr) {
    use llvm_sys::core::*;
    use llvm_sys::analysis::*;
    use llvm_sys::analysis::LLVMVerifierFailureAction::*;

    let n = match expr {
        parser::Expr::Number(n) => n,
        _ => unimplemented!(),
    };

    unsafe {
        let context = LLVMContextCreate();
        let module = LLVMModuleCreateWithName(c_str!("rl"));
        let builder = LLVMCreateBuilderInContext(context);

        let i64_type = LLVMInt64TypeInContext(context);
        let function_type = LLVMFunctionType(i64_type, ptr::null_mut(), 0, 0);
        let function = LLVMAddFunction(module, c_str!("foo"), function_type);

        let bb = LLVMAppendBasicBlockInContext(context, function, c_str!("entry"));
        LLVMPositionBuilderAtEnd(builder, bb);

        let number = LLVMConstInt(i64_type, n, 0);
        LLVMBuildRet(builder, number);

        LLVMVerifyModule(module, LLVMPrintMessageAction, ptr::null_mut());
        LLVMDumpModule(module);

        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}

fn main() {
    let mut line = String::new();

    loop {
        prompt(&mut line).unwrap();

        {
            let parser = parser::Parser::new(&line);
            let expr_results: Vec<parser::ParseResult<parser::Expr>> = parser.collect();
            for expr_result in expr_results {
                match expr_result {
                    Ok(expr) => compile_expr(expr),
                    Err(e)   => println!("error: {:?}", e),
                }
            }
        }

        line.clear();
    }
}
