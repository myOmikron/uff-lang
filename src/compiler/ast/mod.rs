use crate::compiler::lexer::Tokenized;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::support::LLVMString;
use inkwell::types::{FloatType, IntType};
use inkwell::values::FunctionValue;
use inkwell::AddressSpace;
use std::collections::HashMap;

mod definitions;

pub struct AST<'a, 'ctx> {
    tokenized: HashMap<usize, Vec<Tokenized>>,
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    fpm: &'a PassManager<FunctionValue<'ctx>>,
    i32_type: IntType<'ctx>,
    i64_type: IntType<'ctx>,
    float_type: FloatType<'ctx>,
}

impl<'a, 'ctx> AST<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        tokenized: HashMap<usize, Vec<Tokenized>>,
    ) -> Self {
        return AST {
            tokenized,
            context: &context,
            builder: &builder,
            module: &module,
            fpm: &fpm,
            i32_type: context.i32_type(),
            i64_type: context.i64_type(),
            float_type: context.f64_type(),
        };
    }
}
