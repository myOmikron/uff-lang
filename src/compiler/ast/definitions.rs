use crate::compiler::ast::AST;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::{FunctionType, PointerType};
use inkwell::values::{FunctionValue, PointerValue};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;

pub struct ExprRoot<'ctx> {
    pub statements: Vec<Statement>,
    pub variables: HashMap<String, DataTypeValue>,
    pub pointer: RefCell<HashMap<String, (DataType, PointerValue<'ctx>)>>,
}

pub enum Expression {
    String(String),
    Integer(i64),
    Float(f64),
    Ident(String),
}

pub enum Statement {
    None {},
    AssignString { ident: String, value: String },
    AssignInteger { ident: String, value: i64 },
    AssignFloat { ident: String, value: f64 },
    ExitCode { code: i64 },
    ExitVar { variable: String },
    Say { expressions: Vec<Expression> },
}

impl Statement {
    pub fn gen_code<'ctx>(&self, ast: &AST<'ctx>, func: FunctionValue) {
        match self {
            Statement::AssignInteger { ident, value } => {
                if ast.ast.pointer.borrow().contains_key(ident) {
                } else {
                    let ptr = ast.builder.build_alloca(ast.context.i64_type(), ident);
                    let const_int = ast.context.i64_type().const_int(*value as u64, true);
                    ast.builder.build_store(ptr, const_int);
                    ast.ast
                        .pointer
                        .borrow_mut()
                        .insert(String::from(ident), (DataType::Integer, ptr));
                }
            }
            Statement::AssignString { ident, value } => {
                if ast.ast.pointer.borrow().contains_key(ident) {
                } else {
                    let ptr = ast.builder.build_alloca(ast.str_type, ident);
                    let gv = ast.builder.build_global_string_ptr(value, "foo");
                    ast.builder.build_store(ptr, gv);
                    ast.ast
                        .pointer
                        .borrow_mut()
                        .insert(String::from(ident), (DataType::String, ptr));
                }
            }
            Statement::AssignFloat { ident, value } => {
                if ast.ast.pointer.borrow().contains_key(ident) {
                } else {
                    let ptr = ast.builder.build_alloca(ast.float_type, ident);
                    let const_float = ast.context.f64_type().const_float(*value);
                    ast.builder.build_store(ptr, const_float);
                    ast.ast
                        .pointer
                        .borrow_mut()
                        .insert(ident.clone(), (DataType::Float, ptr));
                }
            }
            Statement::ExitCode { code } => {
                let ret = ast.context.i64_type().const_int(*code as u64, true);
                ast.builder.build_return(Some(&ret));

                let dummy = ast.context.append_basic_block(func, "dummy.after.return");
                ast.builder.position_at_end(dummy);
            }
            Statement::ExitVar { variable } => {
                match ast.ast.pointer.borrow().get(variable.as_str()) {
                    None => {}
                    Some(kvp) => {
                        if kvp.0 != DataType::Integer {
                            println!("The answer must be an integer!");
                            return;
                        }
                        let val = ast.builder.build_load(kvp.1.clone(), "load");
                        ast.builder.build_return(Some(&val));
                    }
                }
                let dummy = ast.context.append_basic_block(func, "dummy.after.return");
                ast.builder.position_at_end(dummy);
            }
            Statement::Say { expressions } => {
                let mut args = vec![];

                let mut format = "".to_string();

                for expression in expressions {
                    match expression {
                        Expression::String(val) => {
                            let ptr = ast.builder.build_alloca(ast.str_type, "litptr");
                            let gv = ast.builder.build_global_string_ptr(val, "lit");
                            ast.builder.build_store(ptr, gv);
                            let val = ast.builder.build_load(ptr.clone(), "load");
                            args.push(val.into());
                            format = format + "%s ";
                        }
                        Expression::Integer(val) => {
                            args.push(ast.context.i64_type().const_int(*val as u64, true).into());
                            format = format + "%d ";
                        }
                        Expression::Float(val) => {
                            args.push(ast.context.f64_type().const_float(*val).into());
                            format = format + "%f ";
                        }
                        Expression::Ident(val) => {
                            let ptr = ast.ast.pointer.borrow().get(val).cloned();
                            match ptr {
                                None => {}
                                Some(kvp) => {
                                    args.push(ast.builder.build_load(kvp.1.clone(), "load").into());
                                    match kvp.0 {
                                        DataType::Integer => {
                                            format = format + "%d ";
                                        }
                                        DataType::Float => {
                                            format = format + "%f ";
                                        }
                                        DataType::String => {
                                            format = format + "%s ";
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                let ptr = ast.builder.build_alloca(ast.str_type, "formatptr");
                let gv = ast
                    .builder
                    .build_global_string_ptr(&format!("{}\n", format), "format");
                ast.builder.build_store(ptr, gv);
                let val = ast.builder.build_load(ptr.clone(), "load");
                args.insert(0, val.into());
                ast.builder.build_call(ast.printf, &args, "printf");
            }
            _ => {}
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum DataType {
    Integer,
    Float,
    String,
}

pub enum DataTypeValue {
    Integer { value: i64 },
    Float { value: f64 },
    String { value: String },
}
