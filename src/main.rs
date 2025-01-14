use std::path::Path;
use std::fs;

use clap::Parser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub rex);
pub mod ast;
pub mod regex_ast;

#[derive(Parser)]
#[command(about)]
struct Args {
    #[arg(group = "input")]
    input_file: Option<String>,

    #[arg(short,long)]
    /// Input your Rex program
    rex: String,

    #[arg(short='e',long)]
    /// Use Rust's regex engine to find matches
    use_regex: bool
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

#[cfg(test)]
mod tests {
    use crate::rex;

    #[test]
    fn basic_nomatch() {
        let rex_code = "match { a and { b or c } };";
        let program = rex::rexStmtParser::new().parse(rex_code).unwrap();
        let fail = program.build_state_machine().rex_match("cb".to_string());
        // println!("program: {:#?}", program);
        assert_eq!(fail, false);
    }

    #[test]
    fn basic_match() {
        let rex_code = "match a and { b or c };";
        let program = rex::rexStmtParser::new().parse(rex_code).unwrap();
        let pass = program.build_state_machine().rex_match("ab".to_string());
        // println!("program: {:#?}", program);
        assert_eq!(pass, true);
    }

    #[test]
    fn simple_match() {
        let rex_code = "match a and { b or c };";
        let input = ("bac").to_string();
        let program = rex::rexStmtParser::new().parse(rex_code).unwrap();
        let pass = program.build_state_machine().rex_match(input);
        println!("program: {:#?}", program);
        assert_eq!(pass, true);
    }
}
