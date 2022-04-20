use clap::{Parser, Subcommand};

mod compiler;

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Just run the compiler")]
    Build {
        path: String,
        #[clap(short = 'o', long = "out")]
        #[clap(default_value_t=String::from("./bin/"))]
        out_dir: String,
        #[clap(long = "emit-lexer")]
        #[clap(takes_value = false)]
        emit_lexer: bool,
        #[clap(long = "emit-llvm")]
        #[clap(takes_value = false)]
        emit_llvm: bool,
    },
    #[clap(about = "Compile and run")]
    Run { path: String },
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
        Some(Commands::Build {
            path,
            out_dir,
            emit_lexer,
            emit_llvm,
        }) => {
            compiler::run_compiler(&path, &out_dir, emit_lexer, emit_llvm);
        }
        Some(Commands::Run { path }) => compiler::run_compiler_tmp(&path),
        _ => {}
    }
}
