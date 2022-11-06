use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub rex);
pub mod ast;

fn main() {
    println!("main");
    rex();
}

fn rex() {
    let expr = rex::createStmtParser::new()
        .parse(r#"create "rz" or "z""#)
        .unwrap();
    print!("{expr}");
}
