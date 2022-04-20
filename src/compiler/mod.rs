use std::path::Path;

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

pub fn run_compiler(path: &str, out_dir: &str, emit_lexer: bool, emit_llvm: bool) {
    let path = Path::new(path);
    let out_path = Path::new(out_dir).join(path.file_stem().unwrap());
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
    let path = Path::new(path);
    if !check_file_existence(path) {
        return;
    }
}
