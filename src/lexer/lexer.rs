use super::token::Token;


//I see, it
pub struct Lexer{
    source: String,
    position: usize,
    current_char : Option<char>
}


impl Lexer{
    //this is the constructor for the struct Lexer, don forget
    pub fn new(input: &str) ->Self{
        let mut lexer = Self{
            source: input.to_string(),
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        return lexer;
    }

    pub fn read_char(&mut self){
        self.current_char = self.source.chars().nth(self.position);
        self.position += 1;
    }

    pub fn  next_token(&mut self)-> super::token::Token{
        //if file still have some chars
        self.skip_whitespaces();

        let token = match self.current_char {
            Some('+')    => Token::PLUS,
            Some('-')    => Token::MINUS,
            Some('*')    => Token::MUL,
            Some('/')    => Token::DIV,
            Some('%')    => Token::MOD,
            Some('&')    => Token::AND,
            Some('|')    => Token::OR,
            Some('^')    => Token::XOR,
            Some('!')    => Token::NOT,
            Some('=')    => Token::ASSIGN,
            Some('(')     => Token::LPAREN,
            Some(')')     => Token::RPAREN,
            Some('{')     => Token::LBRACE,
            Some('}')     => Token::RBRACE,
            Some('[')     => Token::LSQUARE,
            Some(']')     => Token::RSQUARE,
            Some(',')     => Token::COMMA,
            Some(';')     => Token::SEMICOLON,
            
            //if other than that
            Some(ch) => {
                println!("Unknown character: {}", ch);
                Token::NONE
            }
            None => Token::EOF,
        };
        //if file ends
        // return token::Token::EOF;
        self.read_char();
        token
    }

    fn skip_whitespaces(&mut self) {
        while let Some(ch)  = self.current_char{
            if ch.is_whitespace(){
                self.read_char();
            }
            else{
                break;
            }
        }
    }
}

