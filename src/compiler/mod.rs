use std::path::Path;

mod lexer;

pub fn run_compiler(path: &str, out_dir: &str) {
    // Check if file exists
    let file = Path::new(path);
    if !file.exists() {
        println!("{} does not exist", path);
        return;
    }
    if !file.is_file() {
        println!("{} is not a file", path);
        return;
    }

    let tokenized = lexer::lex(path);
    for token in tokenized {
        println!("{}", token)
    }
}
