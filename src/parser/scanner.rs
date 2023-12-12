#[derive(Debug, Clone)]
pub enum Token {
    Semicolon(String),
    Whitespace(String),
    Newline(String),
    Begsr(String),
    Endsr(String),
    Exsr(String),
    Raw(String),
}

impl Token {
    pub fn unwrap(&self) -> &String {
        match self {
            Token::Semicolon(s) => s,
            Token::Whitespace(s) => s,
            Token::Newline(s) => s,
            Token::Begsr(s) => s,
            Token::Endsr(s) => s,
            Token::Exsr(s) => s,
            Token::Raw(s) => s,
        }
    }
}

fn find_token(fragment: &str) -> Option<Token> {
    match fragment.to_lowercase().as_str() {
        ";" => Some(Token::Semicolon(fragment.to_string())),
        " " => Some(Token::Whitespace(fragment.to_string())),
        "\n" => Some(Token::Newline(fragment.to_string())),
        "begsr" => Some(Token::Begsr(fragment.to_string())),
        "endsr" => Some(Token::Endsr(fragment.to_string())),
        "exsr" => Some(Token::Exsr(fragment.to_string())),
        _ => None
    }
}

fn find_token_candidate(fragment: &str) -> Option<()> {
    let candidates = vec![
        ";",
        " ",
        "\n",
        "begsr",
        "endsr",
        "exsr",
    ];
    for c in candidates {
        let frag = fragment.to_lowercase();
        if c.starts_with(&frag) {
            return Some(())
        }
    }
    None
}

pub fn scan(input_string: &str) -> Vec<Token> {
    let input: Vec<char> = input_string.chars().collect();
    let input_len = input_string.chars().count();
    let mut out: Vec<Token> = vec![];
    let mut rawbuf: Vec<char> = vec![];
    let mut buf: Vec<char> = vec![];
    let mut cursor: usize = 0;

    while cursor < input_len {
        buf.append(&mut vec![input.get(cursor).unwrap().clone()]);
        let fragment: String = buf.iter().collect();
        let _ = match find_token(&fragment) {
            Some(token) => {
                let rawstring: String = rawbuf.iter().collect();
                if rawstring.len() > 0 {
                    out.append(&mut vec![Token::Raw(rawstring)]);
                    rawbuf = vec![];
                }
                out.append(&mut vec![token]);
                buf = vec![];
                true
            },
            None => {
                match find_token_candidate(&fragment) {
                    Some(()) => {
                        // pass
                        true
                    },
                    None => {
                        rawbuf.append(&mut buf);
                        buf = vec![];
                        true
                    }
                }
            }
        };
        cursor += 1;
    }
    rawbuf.append(&mut buf);
    if rawbuf.len() > 0 {
        out.append(&mut vec![Token::Raw(rawbuf.iter().collect())]);
    }
    out
}
