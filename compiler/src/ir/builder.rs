use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::{FunctionValue, BasicValueEnum, IntValue, PointerValue};
use inkwell::types::IntType;
use inkwell::basic_block::BasicBlock;
use crate::semantic::types::CType;
use std::collections::HashMap;

pub struct IRBuilder<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub i32_type: IntType<'ctx>,
    pub functions: HashMap<String, FunctionValue<'ctx>>,
    pub variables: HashMap<String, BasicValueEnum<'ctx>>,
    pub current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> IRBuilder<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self {
        let module = context.create_module(name);
        let builder = context.create_builder();
        let i32_type = context.i32_type();
        
        Self {
            context,
            module,
            builder,
            i32_type,
            functions: HashMap::new(),
            variables: HashMap::new(),
            current_function: None,
        }
    }

    pub fn get_llvm_type(&self, c_type: &CType) -> IntType<'ctx> {
        match c_type {
            CType::Int => self.i32_type,
            CType::Float => self.i32_type, // 简化处理，都用i32
            CType::Char => self.i32_type,
            CType::Void => self.i32_type,
        }
    }

    pub fn create_function(&mut self, name: &str, return_type: &CType, params: &[CType]) -> FunctionValue<'ctx> {
        let llvm_return_type = self.get_llvm_type(return_type);
        let llvm_param_types: Vec<inkwell::types::BasicMetadataTypeEnum<'ctx>> = params.iter()
            .map(|p| self.get_llvm_type(p).into())
            .collect();
        
        let function_type = llvm_return_type.fn_type(&llvm_param_types, false);
        let function = self.module.add_function(name, function_type, None);
        
        self.functions.insert(name.to_string(), function);
        self.current_function = Some(function);
        function
    }

    pub fn create_entry_block(&self, function: FunctionValue<'ctx>) -> BasicBlock<'ctx> {
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        entry_block
    }

    pub fn create_int_constant(&self, value: i64) -> IntValue<'ctx> {
        self.i32_type.const_int(value as u64, false)
    }

    // 为局部变量创建alloca
    pub fn create_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let function = self.current_function.expect("No current function");
        let entry_block = function.get_first_basic_block().expect("Function has no basic blocks");
        
        // 在函数入口创建alloca
        self.builder.position_at_end(entry_block);
        let alloca = self.builder.build_alloca(self.i32_type, name).unwrap();
        
        alloca
    }

    // 算术运算
    pub fn build_add(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_add(lhs, rhs, "add").unwrap()
    }

    pub fn build_sub(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_sub(lhs, rhs, "sub").unwrap()
    }

    pub fn build_mul(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_mul(lhs, rhs, "mul").unwrap()
    }

    pub fn build_div(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_signed_div(lhs, rhs, "div").unwrap()
    }

    // 比较运算
    pub fn build_eq(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_compare(inkwell::IntPredicate::EQ, lhs, rhs, "eq").unwrap()
    }

    pub fn build_ne(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_compare(inkwell::IntPredicate::NE, lhs, rhs, "ne").unwrap()
    }

    pub fn build_lt(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_compare(inkwell::IntPredicate::SLT, lhs, rhs, "lt").unwrap()
    }

    pub fn build_le(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_compare(inkwell::IntPredicate::SLE, lhs, rhs, "le").unwrap()
    }

    pub fn build_gt(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_compare(inkwell::IntPredicate::SGT, lhs, rhs, "gt").unwrap()
    }

    pub fn build_ge(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_compare(inkwell::IntPredicate::SGE, lhs, rhs, "ge").unwrap()
    }

    // 逻辑运算
    pub fn build_and(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_and(lhs, rhs, "and").unwrap()
    }

    pub fn build_or(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_or(lhs, rhs, "or").unwrap()
    }

    // 一元运算符
    pub fn build_not(&self, value: IntValue<'ctx>) -> IntValue<'ctx> {
        // 逻辑非：!x = (x == 0) ? 1 : 0
        // 实现为：icmp eq x, 0
        // 使用与输入相同位宽的零常量，避免 i1 与 i32 比较不一致导致的 LLVM 验证错误
        let zero = value.get_type().const_int(0, false);
        self.builder.build_int_compare(inkwell::IntPredicate::EQ, value, zero, "not").unwrap()
    }

    pub fn build_neg(&self, value: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_neg(value, "neg").unwrap()
    }

    // 内存操作
    pub fn build_load(&self, ptr: PointerValue<'ctx>, name: &str) -> IntValue<'ctx> {
        self.builder.build_load(self.i32_type, ptr, name).unwrap().into_int_value()
    }

    pub fn build_store(&self, ptr: PointerValue<'ctx>, value: IntValue<'ctx>) {
        self.builder.build_store(ptr, value).unwrap();
    }

    pub fn build_return(&self, value: Option<IntValue<'ctx>>) {
        match value {
            Some(v) => {
                self.builder.build_return(Some(&v)).unwrap();
            }
            None => {
                self.builder.build_return(None).unwrap();
            }
        }
    }

    // 控制流操作
    pub fn build_icmp_ne(&self, lhs: IntValue<'ctx>, rhs: IntValue<'ctx>) -> IntValue<'ctx> {
        self.builder.build_int_compare(inkwell::IntPredicate::NE, lhs, rhs, "icmp_ne").unwrap()
    }

    pub fn create_basic_block(&self, name: &str) -> BasicBlock<'ctx> {
        let function = self.current_function.expect("No current function");
        self.context.append_basic_block(function, name)
    }

    pub fn build_cond_br(&self, condition: IntValue<'ctx>, then_block: BasicBlock<'ctx>, else_block: BasicBlock<'ctx>) {
        self.builder.build_conditional_branch(condition, then_block, else_block).unwrap();
    }

    pub fn build_br(&self, block: BasicBlock<'ctx>) {
        self.builder.build_unconditional_branch(block).unwrap();
    }

    pub fn set_insert_point(&mut self, block: BasicBlock<'ctx>) {
        self.builder.position_at_end(block);
    }

    pub fn get_current_function(&self) -> FunctionValue<'ctx> {
        self.current_function.expect("No current function")
    }
}