mod lexer;

fn main() {
    let mut lexer = lexer::Lexer::new("cd && true");
    let tokens = lexer.scan();
    println!("Tokens : {:?}", tokens);
}
