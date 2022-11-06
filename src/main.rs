use lalrpop_util::lalrpop_mod;
use regex::Regex;

lalrpop_mod!(pub rex);
pub mod ast;

fn main() {
    println!("main");
    rex();
}

fn rex() {
    let re = Regex::new(r"\$([[:alnum:]]+)").unwrap();
    println!("re match: {}", re.is_match("$a1"));
    let expr = rex::rexStmtParser::new().parse(r#"let $a1 match a1;"#).unwrap();
    println!("{expr}");
    let expr = rex::rexStmtParser::new()
        .parse(r#"create x or z and y repeat;"#)
        .unwrap();
    println!("{expr}");
    let expr = rex::rexStmtParser::new()
        .parse(r#"create rz or $a1 and y repeat;"#)
        .unwrap();
    println!("{expr}");
}
