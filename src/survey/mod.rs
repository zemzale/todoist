use std::io::{self, Write};

#[derive(Debug)]
pub struct Question {
    prompt: String,
}

impl Question {
    pub fn new(prompt: String) -> Question {
        return Question{prompt}
    }
}


pub fn ask(question: Question) -> String {
    print!("{} : ", question.prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            return buf;
        }
        Err(..) => {
            panic!("we fucking did it wrong");
        }
    }
}

