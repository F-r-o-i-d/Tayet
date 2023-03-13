
pub fn http_encode(data : &str) -> String {
    let mut encoded = String::new();
    for c in data.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => encoded.push(c),
            ' ' => encoded.push_str("%20"),
            _ => encoded.push_str(&format!("%{:X}", c as u32)),
        }
    }
    encoded
}

pub fn http_decode(data : &str) -> String {
    let mut decoded = String::new();
    let mut chars = data.chars();
    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let mut code = String::new();
                code.push(chars.next().unwrap());
                code.push(chars.next().unwrap());
                decoded.push_str(&String::from_utf8_lossy(&[u8::from_str_radix(&code, 16).unwrap()]));
            },
            '+' => decoded.push(' '),
            c => decoded.push(c),
        }
    }
    decoded
}