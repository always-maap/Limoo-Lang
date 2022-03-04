pub enum Precedence {
    LOWEST,
    EQUALS,      // '==' or '!='
    LESSGREATER, // '>' or '<'
    SUM,         // '+' or '-'
    PRODUCT,     // '*' or '/'
    PREFIX,      // '-x' or '!x'
    CALL,        // 'myFunc(x)'
}
