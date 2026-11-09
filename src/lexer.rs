enum Token {
    tok_eof,                // End of Line
    tok_lparen,
    tok_rparen,
    tok_lbrace,             // {
    tok_rbrace,             // }
    tok_comma,              // ,
    tok_plus,               // +
    tok_increment,          // ++
    tok_minus,              // -
    tok_decrement,          // --
    tok_star,               // *
    tok_slash,              // /
    tok_equal,              // =
    tok_plus_equal,         // +=
    tok_minus_equal,        // -=
    tok_star_equal,         // *=
    tok_slash_equal,        // /=
    tok_equality,           // ==
    tok_inequality,         // !=
    tok_lessthan,           // <
    tok_lessthan_equal,     // <=
    tok_greaterthan,        // >
    tok_greaterthan_equal,  // >=
    tok_period,
    tok_semi,
    tok_and,
    tok_or,
    tok_not,
    tok_xor,
    tok_mod,
    tok_func,
    tok_return,
    tok_colon,
    tok_if,
    tok_else,
    tok_while
}

struct Location{}

struct TokenData{}

struct Lexer{}