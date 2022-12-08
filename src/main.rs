use std::path::Path;
use std::fs;

use clap::Parser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub rex);
pub mod ast;

#[derive(Parser)]
#[command(about)]
struct Args {
    #[arg(group = "input")]
    input_file: Option<String>,

    #[arg(short,long)]
    /// Input your Rex program
    rex: String,
}

fn main() {
    let args = Args::parse();

    let mut input = args.input_file.unwrap();
    let file = Path::new(&input);
    let rex_file = Path::new(&args.rex);

    if file.is_file() {
        input = fs::read_to_string(input).unwrap();
    }

    let rexpr = if rex_file.is_file() {
        let rex_contents = fs::read_to_string(rex_file).unwrap();
        rex::rexStmtParser::new()
            .parse(&rex_contents)
            .unwrap()
    } else {
        rex::rexStmtParser::new()
            .parse(&args.rex)
            .unwrap()
    };
    let rex_match = rexpr.build_state_machine().rex_match(input);
    println!("Rex matches: {:?}", rex_match);
}