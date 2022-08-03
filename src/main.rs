use clap::{Parser, Subcommand};

mod compiler;

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Just run the compiler")]
    Build {
        path: String,
        #[clap(short = 'o', long = "out")]
        #[clap(default_value_t=String::from("./bin/"))]
        out: String,
        #[clap(long = "emit-lexer")]
        #[clap(takes_value = false)]
        emit_lexer: bool,
        #[clap(long = "emit-llvm")]
        #[clap(takes_value = false)]
        emit_llvm: bool,
        #[clap(short = 'U', long = "unoptimization-level")]
        #[clap(default_value = "0")]
        unoptimization_level: u8,
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
            out,
            emit_lexer,
            emit_llvm,
            unoptimization_level,
        }) => {
            compiler::run_compiler(&path, &out, emit_lexer, emit_llvm, unoptimization_level);
        }
        Some(Commands::Run { path }) => compiler::run_compiler_tmp(&path),
        _ => {}
    }
}
