use crate::compiler::ast::AST;
use crate::compiler::lexer::Token;
use inkwell::context::Context;
use inkwell::passes::PassManager;
use std::fs::create_dir_all;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

mod ast;
mod lexer;

fn check_file_existence(path: &Path) -> bool {
    if !path.exists() {
        println!("{} does not exist", path.to_str().unwrap());
        return false;
    }
    if !path.is_file() {
        println!("{} is not a file", path.to_str().unwrap());
        return false;
    }
    return true;
}

fn run_clang(o_file: &NamedTempFile, output_path: &Path) {
    let clang = which::which("clang").expect("clang was not found");
    let mut cmd = Command::new(clang)
        .arg("-O3")
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .arg("-lc")
        .arg("-x")
        .arg("ir")
        .arg(o_file.path().to_str().unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error while executing ld");

    cmd.wait().unwrap();
}

pub fn run_compiler(
    path: &str,
    out: &str,
    emit_lexer: bool,
    emit_llvm: bool,
    unoptimization_level: u8,
) {
    println!("Unoptimization: {}", unoptimization_level);

    let path = Path::new(path);

    let mut out_path = Path::new(out).to_path_buf();
    if out.ends_with("/") {
        if !out_path.exists() {
            if Path::new(&out[..out.len() - 1]).exists() {
                println!("Cannot create directory: File with that name already exists");
                return;
            }
            create_dir_all(&out_path).expect("Couldn't create output directories");
        }
        out_path = out_path.join(path.file_stem().unwrap());
    }

    if !check_file_existence(path) {
        return;
    }

    let tokenized = lexer::lex(path);
    if emit_lexer {
        let mut counter = 1;
        for tokenized in &tokenized {
            if tokenized.token == Token::EOL {
                counter = counter + 1;
                println!();
                continue;
            }

            print!(" :: {:?} {}", tokenized.token, tokenized.value);
        }
        println!();
    }

    let ctx = Context::create();
    let builder = ctx.create_builder();
    let module = ctx.create_module("uff");

    // Create FPM
    let fpm = PassManager::create(&module);
    if unoptimization_level == 0 {
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.initialize();
    }

    let mut ast = AST::new(&ctx, builder, module, fpm, tokenized);
    let ll = ast.compile_ll();

    if emit_llvm {
        println!("{}", ll.to_string());
    }

    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(&ll.to_bytes()).expect("Could not write ll");
    run_clang(&tmp, &out_path);

    tmp.close().expect("Temporary file could not be deleted");
}

pub fn run_compiler_tmp(path: &str) {
    let path = Path::new(path);
    if !check_file_existence(path) {
        return;
    }
}
