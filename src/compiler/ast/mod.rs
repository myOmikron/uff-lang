use crate::compiler::ast::definitions::ExprRoot;
use crate::compiler::lexer::{Token, Tokenized};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::support::LLVMString;
use inkwell::targets::TargetMachine;
use inkwell::types::{FloatType, IntType, PointerType};
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::AddressSpace;
use std::cell::RefCell;
use std::collections::HashMap;
use std::process::exit;

mod definitions;
mod parser;

pub struct AST<'ctx> {
    ast: ExprRoot<'ctx>,
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
    fpm: PassManager<FunctionValue<'ctx>>,
    str_type: PointerType<'ctx>,
    str_ptr_type: PointerType<'ctx>,
    i32_type: IntType<'ctx>,
    i64_type: IntType<'ctx>,
    float_type: FloatType<'ctx>,
    printf: FunctionValue<'ctx>,
}

fn gen_ast<'ctx>(tokenized: Vec<Tokenized>) -> ExprRoot<'ctx> {
    let mut root = ExprRoot {
        statements: Vec::new(),
        variables: HashMap::new(),
        pointer: RefCell::new(HashMap::new()),
    };

    let mut errors: Vec<String> = Vec::new();
    let mut tmp: Vec<Tokenized> = Vec::new();

    for token in tokenized {
        if token.token == Token::EOL {
            match parser::parse(&tmp) {
                Ok(expr) => {
                    root.statements.push(expr);
                }
                Err(s) => {
                    errors.push(s);
                }
            }
            tmp.clear();
        } else if token.token != Token::COMMENT {
            tmp.push(token);
        }
    }

    if errors.len() > 0 {
        for err in errors {
            println!("{}", err);
        }
        exit(1);
    }

    return root;
}

impl<'ctx> AST<'ctx> {
    pub fn new(
        context: &'ctx Context,
        builder: Builder<'ctx>,
        module: Module<'ctx>,
        fpm: PassManager<FunctionValue<'ctx>>,
        tokenized: Vec<Tokenized>,
    ) -> Self {
        let printf_type = context.i32_type().fn_type(
            &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
            true,
        );
        return AST {
            ast: gen_ast(tokenized),
            builder,
            fpm,
            str_type: context.i8_type().ptr_type(AddressSpace::Generic),
            str_ptr_type: context
                .i8_type()
                .ptr_type(AddressSpace::Generic)
                .ptr_type(AddressSpace::Generic),
            i32_type: context.i32_type(),
            i64_type: context.i64_type(),
            float_type: context.f64_type(),
            context,
            printf: module.add_function("printf", printf_type, None),
            module,
        };
    }

    fn gen_code(&mut self, fun: FunctionValue, entry: BasicBlock) {
        for expr in &self.ast.statements {
            expr.gen_code(self, fun.clone());
        }
    }

    pub fn compile_ll(&mut self) -> LLVMString {
        let fn_type = self.i64_type.fn_type(&[], false);
        let main = self.module.add_function("main", fn_type, None);
        let main_entry = self.context.append_basic_block(main, "entry");

        self.builder.position_at_end(main_entry);

        // Start AST to IR generation
        self.gen_code(main.clone(), main_entry.clone());

        let ret = self.i64_type.const_int(42, false);
        self.builder.build_return(Some(&ret));

        if main.verify(true) {
            self.fpm.run_on(&main);
        } else {
            main.print_to_stderr();
        }

        let triple = TargetMachine::get_default_triple();
        self.module.set_triple(&triple);

        return self.module.print_to_string();
    }
}
