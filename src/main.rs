use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub rex);
pub mod ast;

fn main() {
    println!("main");
    rex();
}

fn rex() {
    let expr = rex::rexStmtParser::new()
        .parse(r#"create rz or z and y repeat;"#)
        .unwrap();
    println!("{expr}");
    let expr = rex::rexStmtParser::new()
        .parse(r#"let a1 = rz or z and (y repeat);"#)
        .unwrap();
    println!("{expr}");
}
