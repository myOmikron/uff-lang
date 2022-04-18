use std::path::Path;

mod ast;
mod lexer;

fn check_file_existence(path: &str) -> bool {
    let file = Path::new(path);
    if !file.exists() {
        println!("{} does not exist", path);
        return false;
    }
    if !file.is_file() {
        println!("{} is not a file", path);
        return false;
    }
    return true;
}

pub fn run_compiler(path: &str, out_dir: &str, emit_lexer: bool) {
    if !check_file_existence(path) {
        return;
    }
    let tokenized = lexer::lex(path);
    if emit_lexer {
        for i in 0..tokenized.len() {
            print!("{}", i + 1);
            for t in &tokenized[&i] {
                print!(" :: {:?} {}", t.token, t.value)
            }
            println!()
        }
    }
}

pub fn run_compiler_tmp(path: &str) {
    if !check_file_existence(path) {
        return;
    }
}
