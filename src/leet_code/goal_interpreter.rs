pub fn solve() {
    interpret("G()(al)".to_owned());
    interpret2("G()(al)".to_owned());
    // println!("interpreted value is : {}", ans);
    // println!("interpreted value is : {}", ans2);
}

pub fn interpret(command: String) -> String {
    let mut stream = command.chars().peekable();
    let mut ans = String::new();

    while let Some(curr) = stream.next() {
        let next = stream.peek();
        match (curr, next) {
            ('G', _) => {
                ans += "G";
            }
            ('(', Some(')')) => {
                ans += "o";
            }
            ('(', Some('a')) => {
                ans += "al";
            }
            (_, _) => {}
        }
    }
    ans
}

pub fn interpret2(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}
