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
    let mut ed = DefaultEditor::new().unwrap();
    loop {
        match ed.readline("> ") {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let expr = match calc::parser::parse(&line) {
                    Ok(expr) => expr,
                    Err(e) => {
                        eprintln!("\n{}", e.report());
                        continue;
                    }
                };
                let ops = calc::compiler::compile(&expr);
                let value = calc::vm::eval(&ops);
                println!("{value}");
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }
}
