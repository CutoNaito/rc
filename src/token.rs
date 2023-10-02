pub struct Token {
    pub tok_type: Option<&'static str>,
    pub tok_lit: Option<&'static str>
}

pub fn create_token(tok_type: &'static str, tok_lit: &'static str) -> Token {
    return Token {
        tok_type: Some(tok_type),
        tok_lit: Some(tok_lit)
    }
}

pub fn reserved_tokens() -> Vec<Token> {
    let tokens = vec![
        create_token("ASSIGN", "="),
        create_token("PLUS", "+"),
        create_token("MINUS", "-"),
        create_token("BANG", "!"),
        create_token("LESSTHAN", "<"),
        create_token("GREATERTHAN", ">"),
        create_token("NOTEQUAL", "!="),
        create_token("EQUAL", "=="),
        create_token("FUNCTION", "func"),
        create_token("VAR", "var"),
        create_token("IF", "if"),
        create_token("ELSE", "else"),
        create_token("TRUE", "true"),
        create_token("FALSE", "false"),
        create_token("RETURN", "return")
    ];

    return tokens;
}

pub fn single_line_tokens() -> Vec<Token> {
    let tokens = vec![
        create_token("SLASH", "/"),
        create_token("ASTERISK", "*"),
        create_token("COMMA", ","),
        create_token("SEMICOLON", ";"),
        create_token("LPAREN", "("),
        create_token("RPAREN", ")"),
        create_token("LSQUIRLY", "{"),
        create_token("RSQUIRLY", "}"),
        create_token("LBRACE", "["),
        create_token("RBRACE", "]")
    ];

    return tokens;
}
