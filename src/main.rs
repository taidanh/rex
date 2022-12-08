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
        .parse(r#"match rz or z and rz repeat;"#)
        .unwrap();
    let sm = expr.build_state_machine();
    println!("sm: {:#?}", sm);
    let rex = sm.rex_match("zrzrz".to_string());
    println!("matches 'zrzrz': {:#?}", rex);
    // let expr = rex::rexStmtParser::new()
    //     .parse(r#"create rz or z and y repeat;"#)
    //     .unwrap();
    // println!("matches 'zyyy': {:#?}", expr.build_state_machine().rex_match("zyyy".to_string()));
    // let expr = rex::rexStmtParser::new()
    //     .parse(r#"let $a1 = rz or z and (y repeat);"#)
    //     .unwrap();
    // println!("{:#?}", expr);
}
