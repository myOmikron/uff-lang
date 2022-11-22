use clap::{ArgAction, Parser, Subcommand};

mod compiler;
mod regex;

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Just run the compiler")]
    Build {
        #[clap(help = "Path to the uff-lang file")]
        path: String,
        #[clap(short = 'o', long = "out-dir")]
        #[clap(default_value_t=String::from("./bin/"))]
        #[clap(help = "Output path.")]
        out_dir: String,
        #[clap(long = "emit-lexer")]
        #[clap(action = ArgAction::SetTrue)]
        #[clap(help = "If set, the generated lexed tokens are included in the output")]
        emit_lexer: bool,
        #[clap(long = "emit-llvm")]
        #[clap(action = ArgAction::SetTrue)]
        #[clap(help = "If set, the generated LLVM IR is added to the output")]
        emit_llvm: bool,
        #[clap(short = 'U', long = "unoptimization-level")]
        #[clap(default_value = "0")]
        #[clap(help = "Increase the value for more operations in the output!")]
        unoptimization_level: u8,
    },
    #[clap(about = "Compile and run")]
    Run {
        #[clap(help = "Path to the uff-lang file")]
        path: String,
        #[clap(short = 'U', long = "unoptimization-level")]
        #[clap(default_value = "0")]
        #[clap(help = "Increase the value for more operations in the output!")]
        unoptimization_level: u8,
    },
}

#[derive(Parser)]
#[clap(version = "0.1.0", about = "Compiler for uff-lang", long_about = None)]
#[clap(arg_required_else_help = true)]
#[clap(name = "uff")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Build { .. } => {}
        Commands::Run { .. } => {}
    }
}
