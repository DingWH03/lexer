#![allow(special_module_name)]
mod lib;
use lib::lexer::Lexer;

fn main() {
    let filename = "source.c"; // 你可以将文件名作为输入参数，也可以修改 lex 方法以接受文件路径
    let file_content = std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .to_string();
    // file_content = preprocesself.tokens.pushs(&file_content);
    // println!("{file_content}");
    let mut lexer = Lexer::new(&file_content);
    match lexer.lex() {
        Ok((tokens, tokens_location, errors)) => {
            for (token, location) in tokens.iter().zip(tokens_location.iter()) {
                println!("Tokens: {:?}, Location: {:?}", token, location);
            }
            // 处理错误信息
            for err in errors {
                eprintln!("Error: {}", err);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
