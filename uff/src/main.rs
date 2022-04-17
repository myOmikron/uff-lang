use clap::{Parser, Subcommand};

mod ast;
mod lexer;

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Just run the compiler")]
    Build {
        file: String,
        #[clap(short = 'o', long = "out")]
        #[clap(default_value_t=String::from("./bin/"))]
        output: String,
    },
    #[clap(about = "Compile and run")]
    Run { file: Option<String> },
}

#[derive(Parser)]
#[clap(version = "0.1.0", about = "Compiler for uff-lang", long_about = None)]
#[clap(arg_required_else_help = true)]
#[clap(name = "uff")]
struct CLI {
    #[clap(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli: CLI = CLI::parse();

    match cli.command {
        Some(Commands::Build { file, output }) => {
            let _vec = lexer::lex(&file);
            for v in _vec {
                println!("{}", v);
            }
        }
        Some(Commands::Run { file }) => {}
        _ => {}
    }
}
