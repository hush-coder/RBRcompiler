pub mod generator;
pub mod builder;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum};
use std::collections::HashMap;

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    // 用于管理变量和作用域
    pub variables: HashMap<String, BasicValueEnum<'ctx>>,
    // 当前函数
    pub current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        CodeGen {
            context,
            module: context.create_module(name),
            builder: context.create_builder(),
            variables: HashMap::new(),
            current_function: None,
        }
    }
}
