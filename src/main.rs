use lalrpop_util::lalrpop_mod;
// use regex::Regex;

lalrpop_mod!(pub rex);
pub mod ast;

fn main() {
    println!("main");
    rex();
}

fn rex() {
    // let re = Regex::new(r"[.[^ ]]+").unwrap();
    // println!("z matches .: '{:?}'", re.find("1203!@)(#  b").unwrap());
    let expr = rex::rexStmtParser::new()
        .parse(r#"create rz or z and y repeat;"#)
        .unwrap();
    println!("{:#?}", expr.get_states().get_accepts());
    // let expr = rex::rexStmtParser::new()
    //     .parse(r#"let $a1 = rz or z and (y repeat);"#)
    //     .unwrap();
    // println!("{:#?}", expr);
}
