use std::io::{self, BufRead, Write};

use interpreter::lexer::token::Token;

fn main() -> io::Result<()> {
    const PROMPT: &'static str = ">>";
    let mut lines = io::stdin().lock().lines();

    println!("Hello this is Monkey programming language!");
    println!("Feel free to type in commands");
    println!("Press 'ctrl + c' to exit");

    print!("{PROMPT} ");
    let _ = io::stdout().flush();

    while let Some(line) = lines.next() {
        match line {
            Ok(line) => {
                let mut l = interpreter::lexer::Lexer::new(&line);
                loop {
                    let token = l.next_token();
                    if token == Token::Eof {
                        break;
                    }
                    println!("{token:?}");
                }
                print!("{PROMPT} ");
                let _ = io::stdout().flush();
            }
            Err(err) => eprintln!("Error reading line: {err}"),
        }
    }

    Ok(())
}
