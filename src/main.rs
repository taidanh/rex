use std::path::Path;
use std::fs;

use clap::{Parser, ArgAction};
use lalrpop_util::lalrpop_mod;
// use regex::Regex;

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

    #[arg(long,action = ArgAction::SetTrue)]
    /// Output regex instead of matching on input
    regex: bool,
}

fn main() {
    let args = Args::parse();

    let input = args.input_file.unwrap();
    let file = Path::new(&input);
    let rex_file = Path::new(&args.rex);

    // println!("input: {:?}", file.is_file());
    // println!("main");
    // rex();
    if file.is_file() {
        let contents = fs::read_to_string(input);
        let rex_match = rex::rexStmtParser::new().parse(contents);
    } else {
        let rex_match = rex::rexStmtParser::new().parse(&input);
    }
}

fn rex() {
    // let re = Regex::new(r"[.[^ ]]+").unwrap();
    // println!("z matches .: '{:?}'", re.find("1203!@)(#  b").unwrap());
    let expr = rex::rexStmtParser::new()
        .parse(r#"create rz or z and rz repeat;"#)
        .unwrap();
    let sm = expr.build_state_machine();
    println!("sm: {:#?}", sm);
    let rex = sm.rex_match("zrzrz".to_string());
    println!("matches 'zrzrz': {:#?}", rex);
    let expr = rex::rexStmtParser::new()
        .parse(r#"create rz or z and y repeat;"#)
        .unwrap();
    println!("matches 'zyyy': {:#?}", expr.build_state_machine().rex_match("zyyy".to_string()));
    // let expr = rex::rexStmtParser::new()
    //     .parse(r#"let $a1 = rz or z and (y repeat);"#)
    //     .unwrap();
    // println!("{:#?}", expr);
}
