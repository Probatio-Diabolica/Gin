#[derive(Debug,Clone,PartialEq)]
pub enum Token{
// single char Operators
    PLUS,           //  +
    MINUS,          //  -
    MUL,            //  *
    DIV,            //  /
    MOD,            //  %
    AND,            //  &  
    OR,             //  |
    XOR,            //  ^
    NOT,            //  !
    ASSIGN,         //  =

// //double char Operators
//     EQUALS,          // ==
//     LOGICAL_AND,     // and
//     LOGICAL_OR,      // or

    // parens and others
    LPAREN,         //  (
    RPAREN,         //  )
    LBRACE,         //  {
    RBRACE,         //  }
    LSQUARE,        //  [
    RSQUARE,        //  ]
    COMMA,          //  ,
    SEMICOLON,      //  ;
    
// Literals
    INT(i64),       //
    FLOAT(f64),     //
    STRING(String), //
    CHAR(char),     //
    BOOL(bool),     //
    NONE,           //
// Special
    EOF
}
