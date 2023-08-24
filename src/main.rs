use clap::Parser;
use clap::Subcommand;
use rustyline::{error::ReadlineError, DefaultEditor};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    Repl,
    Gen,
}

fn main() {
    match Cli::parse().cmd {
        Some(Cmd::Gen) => gen(),
        Some(Cmd::Repl) => repl(),
        _ => repl(),
    }
}

fn gen() {
    let expr = calc::expr::Expr::generate();
    println!("{expr}");
}

fn repl() {
    fn run_and_print(src: &str) {
        if src.is_empty() {
            return;
        }
        let expr = match calc::parser::parse(src) {
            Ok(expr) => expr,
            Err(e) => {
                eprintln!("\n{}", e.report());
                return;
            }
        };
        let ops = calc::compiler::compile(&expr);
        let value = calc::vm::eval(&ops);
        println!("{value}");
    }

    let mut ed = DefaultEditor::new().unwrap();
    loop {
        match ed.readline("> ") {
            Ok(line) => run_and_print(&line),
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}
