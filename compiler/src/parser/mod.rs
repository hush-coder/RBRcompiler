pub mod ast;
pub mod precedence;

// LALRPOP生成的grammar模块
include!(concat!(env!("OUT_DIR"), "/parser/grammar.rs"));

use crate::error::error::CompileError;

pub fn parse(input: &str) -> Result<ast::ASTNode, CompileError> {
    // Use the LALRPOP generated parser directly with string input
    match crate::parser::CompUnitParser::new().parse(input) {
        Ok(ast) => Ok(ast),
        Err(e) => Err(CompileError::Parse(format!("Parse error: {}", e))),
    }
} 
