pub fn sendMessage(msg: &str) {
    println!("Content Length : {}\r\n\r\n{}", msg.as_bytes().len(), msg)
}

pub fn encode(msg: &str) -> String {
    format!("Content Length : {}\r\n\r\n{}", msg.as_bytes().len(), msg)
}

pub fn decode(string: &str) -> Result<(String, i32), Box<dyn std::error::Error>> {
    let (header, content) = string.split_once("\r\n\r\n").ok_or("err")?;
    let content_len: i32 = header.split_once(":").ok_or("err")?.1.parse()?;

    return Ok((content.to_string(), content_len));
}
