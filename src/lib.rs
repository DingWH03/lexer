#![allow(dead_code)]
pub mod lexer {
    use std::io::{self};
    // 定义关键字枚举类型
    #[derive(Debug, Clone, PartialEq)]
    pub enum Keyword {
        Auto,
        Break,
        Case,
        Char,
        Const,
        Continue,
        Default,
        Do,
        Double,
        Else,
        Enum,
        Extern,
        Float,
        For,
        Goto,
        If,
        Int,
        Long,
        Register,
        Return,
        Short,
        Signed,
        Sizeof,
        Static,
        Struct,
        Switch,
        Typedef,
        Union,
        Unsigned,
        Void,
        Volatile,
        While,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Operator {
        // Arithmetic Operators
        Add,      // +
        Subtract, // -
        Multiply, // *
        Divide,   // /
        Modulus,  // %

        // Relational Operators
        Equal,              // ==
        NotEqual,           // !=
        LessThan,           // <
        GreaterThan,        // >
        LessThanOrEqual,    // <=
        GreaterThanOrEqual, // >=

        // Logical Operators
        LogicalAnd, // &&
        LogicalOr,  // ||
        LogicalNot, // !

        // Bitwise Operators
        BitwiseAnd, // &
        BitwiseOr,  // |
        BitwiseXor, // ^
        BitwiseNot, // ~
        LeftShift,  // <<
        RightShift, // >>

        // Assignment Operators
        Assign,           // =
        AddAssign,        // +=
        Increment,        // ++
        Decrement,        // --
        SubtractAssign,   // -=
        MultiplyAssign,   // *=
        DivideAssign,     // /=
        ModulusAssign,    // %=
        LeftShiftAssign,  // <<=
        RightShiftAssign, // >>=
        BitwiseAndAssign, // &=
        BitwiseOrAssign,  // |=
        BitwiseXorAssign, // ^=

        // Other Operators
        AddressOf,           // &
        Dereference,         // *
        MemberAccess,        // 表示使用 '.' 进行结构体成员访问
        PointerMemberAccess, // 表示使用 '->' 进行指针结构体成员访问
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Delimiter {
        //界符
        Semicolon,            // ;
        Comma,                // ,
        LeftParenthesis,      // (
        RightParenthesis,     // )
        LeftBracket,          // [
        RightBracket,         // ]
        LeftBrace,            // {
        RightBrace,           // }
        Backslash,            // \
        ConditionalOperator,  // ?
        ConditionalSeparator, // :
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Number {
        Integer(i64), // 整数类型，使用 i64 表示
        Float(f64),   // 浮点数类型，使用 f64 表示
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        Keywords(Keyword),
        Identifiers(String),
        Numbers(Number),
        Operators(Operator),
        Delimiters(Delimiter),
        Strings(String),
        EOF
    }

    #[derive(Debug, Clone)]
    pub struct TokenLocation {
        row: usize,
        col: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum State {
        Start,  //开始匹配
        State1, //开始->字母或数字或下划线 State1->字母或数字或下划线
        State11,
        State12,
        State2, //开始->数字
        State20,
        State21,
        State22,
        State23,
        State24,
        State25,
        State26,
        State261,
        State27,  //匹配二进制
        State28,  //匹配8进制
        State281, //8进制异常处理
        // State29, // 非多个e或是小数点的错误处理
        State3,  //开始->运算符
        State31, // State31可匹配<<=
        State32, // State32可匹配>>=
        State33, //处理注释/* */
        State4,  //开始->界符
        State41, //匹配单引号
        State42, //匹配双引号
    }

    pub struct Lexer {
        state: State,
        start_index: usize,
        index: usize,
        row: usize,
        col: usize,
        chars: Vec<char>,
        chars_len: usize,
        tokens: Vec<Token>,
        tokens_location: Vec<TokenLocation>,
        errors: Vec<String>, // 记录错误信息
    }
    impl Lexer {
        pub fn new(input: &str) -> Self {
            let mut chars: Vec<char> = input.chars().collect();
            chars.push(' ');
            let chars_len: usize = chars.len();
            Lexer {
                state: State::Start,
                start_index: 0,
                index: 0,
                row: 1,
                col: 1,
                chars,                       // 输入字符序列
                chars_len,                   // 输入字符序列长度
                tokens_location: Vec::new(), // 输出token所在位置
                tokens: Vec::new(),          // 输出token
                errors: Vec::new(),          // 初始化错误向量
            }
        }

        // 获取关键字的函数
        fn get_keyword(&mut self, keyword_str: &str) -> Option<Keyword> {
            match keyword_str.to_lowercase().as_str() {
                "auto" => Some(Keyword::Auto),
                "break" => Some(Keyword::Break),
                "case" => Some(Keyword::Case),
                "char" => Some(Keyword::Char),
                "const" => Some(Keyword::Const),
                "continue" => Some(Keyword::Continue),
                "default" => Some(Keyword::Default),
                "do" => Some(Keyword::Do),
                "double" => Some(Keyword::Double),
                "else" => Some(Keyword::Else),
                "enum" => Some(Keyword::Enum),
                "extern" => Some(Keyword::Extern),
                "float" => Some(Keyword::Float),
                "for" => Some(Keyword::For),
                "goto" => Some(Keyword::Goto),
                "if" => Some(Keyword::If),
                "int" => Some(Keyword::Int),
                "long" => Some(Keyword::Long),
                "register" => Some(Keyword::Register),
                "return" => Some(Keyword::Return),
                "short" => Some(Keyword::Short),
                "signed" => Some(Keyword::Signed),
                "sizeof" => Some(Keyword::Sizeof),
                "static" => Some(Keyword::Static),
                "struct" => Some(Keyword::Struct),
                "switch" => Some(Keyword::Switch),
                "typedef" => Some(Keyword::Typedef),
                "union" => Some(Keyword::Union),
                "unsigned" => Some(Keyword::Unsigned),
                "void" => Some(Keyword::Void),
                "volatile" => Some(Keyword::Volatile),
                "while" => Some(Keyword::While),
                _ => None,
            }
        }

        pub fn lex(&mut self) -> io::Result<(Vec<Token>, Vec<TokenLocation>, Vec<String>)> {
            // let mut next_index = 0;
            while self.index < self.chars_len {
                self.index = self.process_char(self.index);
            }
            self.tokens.push(Token::EOF);
            self.tokens_location.push(TokenLocation {row: self.row, col: self.col});
            Ok((
                self.tokens.clone(),
                self.tokens_location.clone(),
                self.errors.clone(),
            )) // 返回 tokens 和错误信息的元组
        }

        fn process_char(&mut self, ptr_index: usize) -> usize {
            // 在方法中使用 index 参数来获取当前字符的序号
            // let c = self.chars[index]; // 通过索引获取字符
            let mut next_index: usize = ptr_index + 1;
            match self.state {
                State::Start => match self.chars[ptr_index] {
                    'a'..='z' | 'A'..='Z' | '_' => {
                        self.start_index = ptr_index;
                        self.state = State::State1; // 匹配到字母或下划线-进入State1
                        return ptr_index;
                    }
                    '0'..='9' => {
                        self.start_index = ptr_index;
                        self.state = State::State2; // 匹配到数字-进入State2
                        return ptr_index;
                    }
                    '+' | '-' | '*' | '/' | '%' | '=' | '!' | '<' | '>' | '&' | '|' | '^' | '~'
                    | '.' => {
                        self.start_index = ptr_index;
                        self.state = State::State3; // 匹配到运算符-进入State3
                        self.col += 1;
                        return next_index;
                    }
                    ';' | ',' | '(' | ')' | '[' | ']' | '{' | '}' | '\'' | '"' | ':' | '?' => {
                        self.start_index = ptr_index;
                        self.state = State::State4; // 匹配到界符-进入State4
                        self.col += 1;
                        return next_index;
                    }
                    ' ' => {
                        //匹配到空格-跳过
                        self.state = State::Start;
                        self.col += 1;
                        return next_index;
                    }
                    '\n' => {
                        //匹配到回车-跳过
                        self.state = State::Start;
                        self.row += 1;
                        self.col = 1;
                        return next_index;
                    }
                    _ => {
                        self.errors.push(format!(
                            "Unknown key: {} in {} index(State::Start)",
                            &self.chars[ptr_index], ptr_index
                        ));
                        self.col += 1;
                        return next_index;
                    }
                },
                State::State1 => match self.chars[ptr_index] {
                    'a'..='z' => {
                        self.state = State::State11;
                        self.col += 1;
                        return next_index;
                    }
                    'A'..='Z' | '_' => {
                        self.state = State::State12;
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.errors
                            .push("Entered a wrong pattern: State1._".to_string());
                        self.state = State::Start;
                        return ptr_index;
                    }
                },
                State::State11 => match self.chars[ptr_index] {
                    'a'..='z' => {
                        self.state = State::State11;
                        self.col += 1;
                        return next_index;
                    }
                    'A'..='Z' | '_' | '0'..='9' => {
                        self.state = State::State12;
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        let identifier = &self.chars[self.start_index..ptr_index]; // 直接获取字符切片
                        let identifier_str: String = identifier.iter().collect(); // 将切片转换为字符串
                        if ptr_index - self.start_index > 10 {
                            self.tokens.push(Token::Identifiers(identifier_str));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.start_index + self.col - ptr_index,
                            });
                        } else {
                            if let Some(keyword) = self.get_keyword(&identifier_str) {
                                // 如果 get_keyword 返回 Some，表示找到了关键字
                                self.tokens.push(Token::Keywords(keyword));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            } else {
                                self.tokens.push(Token::Identifiers(identifier_str));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            }
                        }
                        self.state = State::Start;
                        return ptr_index;
                    }
                },
                State::State12 => match self.chars[ptr_index] {
                    'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                        self.state = State::State12;
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        let identifier = &self.chars[self.start_index..ptr_index]; // 直接获取字符切片
                        let identifier_str: String = identifier.iter().collect(); // 将切片转换为字符串
                        self.tokens.push(Token::Identifiers(identifier_str));
                        self.tokens_location.push(TokenLocation {
                            row: self.row,
                            col: self.start_index + self.col - ptr_index,
                        });
                        self.state = State::Start;
                        return ptr_index;
                    }
                },
                State::State2 => match self.chars[ptr_index] {
                    '1'..='9' => {
                        self.state = State::State20; // State20开始匹配十进制整数/浮点数
                        self.col += 1;
                        return next_index;
                    }
                    '0' => {
                        self.state = State::State25; // State25尝试匹配多进制数
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.errors
                            .push("Entered a wrong pattern: State2._".to_string()); // 不可能金进入该匹配
                        self.state = State::Start;
                        return ptr_index;
                    }
                },
                State::State20 => {
                    match self.chars[ptr_index] {
                        '0'..='9' => {
                            self.state = State::State20;
                            self.col += 1;
                            return next_index;
                        }
                        '.' => {
                            self.state = State::State21; // State21不匹配.和e,匹配完第一个字符转为State21
                            self.col += 1;
                            return next_index;
                        }
                        'e' | 'E' => {
                            self.state = State::State23; // State23不匹配e和.但匹配-或正
                            self.col += 1;
                            return next_index;
                        }
                        _ => {
                            let mut number: i64 = 0;
                            for &c in &self.chars[self.start_index..ptr_index] {
                                if let Some(digit) = c.to_digit(10) {
                                    number = number * 10 + digit as i64;
                                }
                            }
                            if self.chars[self.start_index] == '-' {
                                number = 0 - number;
                            }
                            self.tokens.push(Token::Numbers(Number::Integer(number)));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.start_index + self.col - ptr_index,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                    }
                }
                State::State21 => match self.chars[ptr_index] {
                    '0'..='9' => {
                        self.state = State::State22;
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        let error_number: String =
                            self.chars[self.start_index..ptr_index].iter().collect();
                        self.errors.push(format!("Error number: {}", error_number));
                        self.state = State::Start;
                        return ptr_index;
                    }
                },
                State::State22 => {
                    match self.chars[ptr_index] {
                        '0'..='9' => {
                            self.state = State::State22;
                            self.col += 1;
                            return next_index;
                        }
                        'e' | 'E' => {
                            self.state = State::State23; // State23不匹配e和.
                            self.col += 1;
                            return next_index;
                        }
                        _ => {
                            let number_str: String =
                                self.chars[self.start_index..ptr_index].iter().collect();
                            if let Ok(number) = number_str.parse::<f64>() {
                                self.tokens.push(Token::Numbers(Number::Float(number)));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            }
                            self.state = State::Start;
                            return ptr_index;
                        }
                    }
                }
                State::State23 => {
                    match self.chars[ptr_index] {
                        '0'..='9' | '-' | '+' => {
                            self.state = State::State24; // State23不匹配- +
                            self.col += 1;
                            return next_index;
                        }
                        _ => {
                            let error_number: String =
                                self.chars[self.start_index..ptr_index].iter().collect();
                            self.errors.push(format!("Error number: {}", error_number));
                            self.state = State::Start;
                            return ptr_index;
                        }
                    }
                }
                State::State24 => {
                    match self.chars[ptr_index] {
                        '0'..='9' => {
                            self.state = State::State24; // State24不匹配-
                            self.col += 1;
                            return next_index;
                        }
                        _ => {
                            let number_str: String =
                                self.chars[self.start_index..ptr_index].iter().collect();
                            if let Ok(number) = number_str.parse::<f64>() {
                                self.tokens.push(Token::Numbers(Number::Float(number)));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            }
                            self.state = State::Start;
                            return ptr_index;
                        }
                    }
                }
                State::State25 => {
                    match self.chars[ptr_index] {
                        '.' => {
                            self.state = State::State21; // 0.x匹配十进制浮点数 State20不匹配.和e,匹配完第一个字符转为State21
                            self.col += 1;
                            return next_index;
                        }
                        'x' | 'X' => {
                            self.state = State::State26; // State26匹配16进制数
                            self.col += 1;
                            return next_index;
                        }
                        'b' | 'B' => {
                            self.state = State::State27; // State27匹配2进制数
                            self.col += 1;
                            return next_index;
                        }
                        '0'..='7' => {
                            self.state = State::State28; // State28匹配8进制数
                            self.col += 1;
                            return next_index;
                        }
                        'e' | 'E' => {
                            self.state = State::State23; // 0e State23不匹配e和.但匹配-或正
                            self.col += 1;
                            return next_index;
                        }
                        '8' | '9' => {
                            // 不存在的数字
                            while self.chars[next_index] >= '0' && self.chars[next_index] <= '9' {
                                next_index += 1;
                                self.col += 1;
                            }
                            let number_str: String =
                                self.chars[self.start_index..next_index].iter().collect();
                            self.state = State::Start;
                            self.errors
                                .push(format!("Error octal number: {}", number_str));
                            // self.col += 1;
                            return next_index;
                        }
                        _ => {
                            // 直接匹配数字整形0
                            self.tokens.push(Token::Numbers(Number::Integer(0)));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.start_index + self.col - ptr_index,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                    }
                }
                State::State26 => match self.chars[ptr_index] {
                    // 主要确定十六进制数字不能是0x后面不跟数字
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        self.state = State::State261; // State261继续处理16进制数
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.state = State::Start;
                        self.errors.push(format!(
                            "Error hexadecimal number in State26 in {}",
                            self.start_index
                        ));
                        return ptr_index;
                    }
                },
                State::State261 => match self.chars[ptr_index] {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        self.state = State::State261; // State261继续处理16进制数
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.state = State::Start;

                        if self.chars[self.start_index] != '-' {
                            let number_str: String =
                                self.chars[self.start_index..ptr_index].iter().collect();
                            if let Ok(number) = i64::from_str_radix(&number_str[2..], 16) {
                                self.tokens
                                    .push(Token::Numbers(Number::Integer(number as i64)));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            } else {
                                self.errors
                                    .push(format!("Error number: {} in State261", number_str));
                            }
                        } else {
                            let number_str: String =
                                self.chars[self.start_index + 1..ptr_index].iter().collect();
                            if let Ok(number) = i64::from_str_radix(&number_str[2..], 16) {
                                let number_negetive = 0 - number;
                                self.tokens
                                    .push(Token::Numbers(Number::Integer(number_negetive as i64)));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            } else {
                                self.errors
                                    .push(format!("Error number: {} in State261", number_str));
                            }
                        }
                        return ptr_index;
                    }
                },
                State::State27 => match self.chars[ptr_index] {
                    '0' | '1' => {
                        while self.chars[next_index] == '0' || self.chars[next_index] == '1' {
                            // 直接使用循环，避免递归次数过多堆栈溢出(现已将整个词法处理程序更改为循环)
                            next_index += 1;
                            self.col += 1;
                        }
                        let number_str: String =
                            self.chars[self.start_index..next_index].iter().collect();
                        if let Ok(number) = i64::from_str_radix(&number_str[2..], 2) {
                            self.tokens
                                .push(Token::Numbers(Number::Integer(number as i64)));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.start_index + self.col - ptr_index,
                            });
                        } else {
                            self.errors
                                .push(format!("Error number: {} in State27", number_str));
                        }
                        self.state = State::Start;
                        // self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.state = State::Start;
                        let number_str: String =
                            self.chars[self.start_index..next_index].iter().collect();
                        self.errors
                            .push(format!("Error number: {} in State27", number_str));
                        return ptr_index;
                    }
                },
                State::State28 => match self.chars[ptr_index] {
                    '0'..='7' => {
                        self.state = State::State28; // State28继续处理8进制数
                        self.col += 1;
                        return next_index;
                    }
                    '8' | '9' => {
                        self.state = State::State281; // 进入到8进制异常处理State281
                        self.col += 1;
                        return next_index;
                    }
                    '.' => {
                        // 按照C语言标准，0开头但带小数点看作十进制浮点数
                        self.state = State::State21; // State20不匹配.和e,匹配完第一个字符转为State21
                        self.col += 1;
                        return next_index;
                    }
                    'e' | 'E' => {
                        // 按照C语言标准，0开头但e看作十进制浮点数
                        self.state = State::State23; // State23不匹配e和.但匹配-或正
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.state = State::Start;
                        let number_str: String =
                            self.chars[self.start_index..ptr_index].iter().collect();
                        // println!("{}", &number_str);
                        if let Ok(number) = i64::from_str_radix(&number_str[1..], 8) {
                            if self.chars[self.start_index] == '-' {
                                self.tokens
                                    .push(Token::Numbers(Number::Integer(0 - number as i64)));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            } else {
                                self.tokens
                                    .push(Token::Numbers(Number::Integer(number as i64)));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.start_index + self.col - ptr_index,
                                });
                            }
                        } else {
                            self.errors
                                .push(format!("Error number: {} in State28", number_str));
                        }
                        return ptr_index;
                    }
                },
                State::State281 => {
                    match self.chars[ptr_index] {
                        '0'..='9' => {
                            self.state = State::State281; // State281继续收集错误数字
                            self.col += 1;
                            return next_index;
                        }
                        _ => {
                            self.state = State::Start;
                            let number_str: String =
                                self.chars[self.start_index..next_index].iter().collect();
                            self.errors
                                .push(format!("Invalid octal number: {} in State28", number_str));
                            return ptr_index;
                        }
                    }
                }
                // State::State29 => {

                // }
                State::State3 => {
                    match self.chars[self.start_index] {
                        '+' => {
                            // 处理 '+' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens.push(Token::Operators(Operator::AddAssign));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '+' => {
                                    self.tokens.push(Token::Operators(Operator::Increment));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '0'..='9' => {
                                    if matches!(
                                        self.tokens.last(),
                                        Some(Token::Identifiers(_))
                                            | Some(Token::Delimiters(Delimiter::RightParenthesis))
                                            | Some(Token::Numbers(_))
                                    ) {
                                        self.tokens.push(Token::Operators(Operator::Add));
                                        self.tokens_location.push(TokenLocation {
                                            row: self.row,
                                            col: self.col - 1,
                                        });
                                        self.state = State::Start;
                                        return ptr_index;
                                    } else {
                                        self.state = State::State2; // State2匹配负数
                                        return ptr_index;
                                    }
                                }
                                _ => {
                                    self.tokens.push(Token::Operators(Operator::Add));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    return ptr_index;
                                }
                            }
                        }
                        '-' => {
                            // 处理 '-' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens.push(Token::Operators(Operator::SubtractAssign));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '-' => {
                                    self.tokens.push(Token::Operators(Operator::Decrement));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '>' => {
                                    self.tokens
                                        .push(Token::Operators(Operator::PointerMemberAccess));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '0'..='9' => {
                                    if matches!(
                                        self.tokens.last(),
                                        Some(Token::Identifiers(_))
                                            | Some(Token::Delimiters(Delimiter::RightParenthesis))
                                            | Some(Token::Numbers(_))
                                    ) {
                                        self.tokens.push(Token::Operators(Operator::Subtract));
                                        self.tokens_location.push(TokenLocation {
                                            row: self.row,
                                            col: self.col - 1,
                                        });
                                        self.state = State::Start;
                                        return ptr_index;
                                    } else {
                                        self.state = State::State2; // State2匹配负数
                                        return ptr_index;
                                    }
                                }
                                _ => {
                                    self.tokens.push(Token::Operators(Operator::Subtract));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    return ptr_index;
                                }
                            }
                        }
                        '*' => {
                            // 处理 '*' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens.push(Token::Operators(Operator::MultiplyAssign));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                _ => {
                                    if matches!(
                                        self.tokens.last(),
                                        Some(Token::Identifiers(_))
                                            | Some(Token::Delimiters(Delimiter::RightParenthesis))
                                            | Some(Token::Numbers(_))
                                    ) {
                                        self.tokens.push(Token::Operators(Operator::Multiply));
                                        self.tokens_location.push(TokenLocation {
                                            row: self.row,
                                            col: self.col - 1,
                                        });
                                        self.state = State::Start;
                                        return ptr_index;
                                    } else {
                                        self.tokens.push(Token::Operators(Operator::Dereference));
                                        self.tokens_location.push(TokenLocation {
                                            row: self.row,
                                            col: self.col - 1,
                                        });
                                        self.state = State::Start;
                                        return ptr_index;
                                    }
                                }
                            }
                        }
                        '/' => {
                            // 处理 '/' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens.push(Token::Operators(Operator::DivideAssign));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '/' => {
                                    while self.chars[next_index] != '\n' {
                                        self.col += 1;
                                        next_index += 1;
                                    }
                                    self.row += 1;
                                    self.state = State::Start;
                                    // self.col += 1;
                                    return next_index;
                                } // 处理注释
                                '*' => {
                                    self.state = State::State33;
                                    self.col += 1;
                                    return next_index;
                                } // 处理注释
                                _ => {
                                    self.tokens.push(Token::Operators(Operator::Divide));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    return ptr_index;
                                }
                            }
                        }
                        '%' => match self.chars[ptr_index] {
                            '=' => {
                                self.tokens.push(Token::Operators(Operator::ModulusAssign));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                self.col += 1;
                                return next_index;
                            }
                            _ => {
                                self.tokens.push(Token::Operators(Operator::Modulus));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                return ptr_index;
                            }
                        },
                        '=' => {
                            // 处理 '=' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens.push(Token::Operators(Operator::Equal));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                _ => {
                                    self.tokens.push(Token::Operators(Operator::Assign));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    return ptr_index;
                                }
                            }
                        }
                        '!' => {
                            // 处理 '!' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens.push(Token::Operators(Operator::NotEqual));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                _ => {
                                    self.tokens.push(Token::Operators(Operator::LogicalNot));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    return ptr_index;
                                }
                            }
                        }
                        '<' => {
                            // 处理 '<' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens
                                        .push(Token::Operators(Operator::LessThanOrEqual));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '<' => {
                                    self.state = State::State31; // State31可匹配<<=
                                    self.col += 1;
                                    return next_index;
                                }
                                _ => {
                                    self.tokens.push(Token::Operators(Operator::LessThan));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    return ptr_index;
                                }
                            }
                        }
                        '>' => {
                            // 处理 '>' 的逻辑...
                            match self.chars[ptr_index] {
                                '=' => {
                                    self.tokens
                                        .push(Token::Operators(Operator::GreaterThanOrEqual));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                                '>' => {
                                    self.state = State::State32; // State32可匹配>>=
                                    self.col += 1;
                                    return next_index;
                                }
                                _ => {
                                    self.tokens.push(Token::Operators(Operator::GreaterThan));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    return ptr_index;
                                }
                            }
                        }
                        '&' => match self.chars[ptr_index] {
                            '=' => {
                                self.tokens
                                    .push(Token::Operators(Operator::BitwiseAndAssign));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col,
                                });
                                self.state = State::Start;
                                self.col += 1;
                                return next_index;
                            }
                            '&' => {
                                self.tokens.push(Token::Operators(Operator::LogicalAnd));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                return ptr_index;
                            }
                            _ => {
                                if matches!(
                                    self.tokens.last(),
                                    Some(Token::Identifiers(_))
                                        | Some(Token::Delimiters(Delimiter::RightParenthesis))
                                        | Some(Token::Strings(_))
                                ) {
                                    self.tokens.push(Token::Operators(Operator::BitwiseAnd));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                } else {
                                    self.tokens.push(Token::Operators(Operator::AddressOf));
                                    self.tokens_location.push(TokenLocation {
                                        row: self.row,
                                        col: self.col - 1,
                                    });
                                    self.state = State::Start;
                                    self.col += 1;
                                    return next_index;
                                }
                            }
                        },
                        '|' => match self.chars[ptr_index] {
                            '=' => {
                                self.tokens
                                    .push(Token::Operators(Operator::BitwiseOrAssign));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                self.col += 1;
                                return next_index;
                            }
                            '|' => {
                                self.tokens.push(Token::Operators(Operator::LogicalOr));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                self.col += 1;
                                return next_index;
                            }
                            _ => {
                                self.tokens.push(Token::Operators(Operator::BitwiseOr));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                return ptr_index;
                            }
                        },
                        '^' => match self.chars[ptr_index] {
                            '=' => {
                                self.tokens
                                    .push(Token::Operators(Operator::BitwiseXorAssign));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                self.col += 1;
                                return next_index;
                            }
                            _ => {
                                self.tokens.push(Token::Operators(Operator::BitwiseXor));
                                self.tokens_location.push(TokenLocation {
                                    row: self.row,
                                    col: self.col - 1,
                                });
                                self.state = State::Start;
                                return ptr_index;
                            }
                        },
                        '.' => {
                            self.tokens.push(Token::Operators(Operator::MemberAccess));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '~' => {
                            self.tokens.push(Token::Operators(Operator::BitwiseNot));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        _ => {
                            self.errors.push(format!(
                                "Unkown key: {} in {} (State::State3)",
                                self.chars[ptr_index], ptr_index
                            ));
                            self.col += 1;
                            return next_index;
                        }
                    }
                }
                State::State31 => match self.chars[ptr_index] {
                    '=' => {
                        self.tokens
                            .push(Token::Operators(Operator::LeftShiftAssign));
                        self.tokens_location.push(TokenLocation {
                            row: self.row,
                            col: self.col - 2,
                        });
                        self.state = State::Start;
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.tokens.push(Token::Operators(Operator::LeftShift));
                        self.tokens_location.push(TokenLocation {
                            row: self.row,
                            col: self.col - 1,
                        });
                        self.state = State::Start;
                        return ptr_index;
                    }
                },
                State::State32 => match self.chars[ptr_index] {
                    '=' => {
                        self.tokens
                            .push(Token::Operators(Operator::RightShiftAssign));
                        self.tokens_location.push(TokenLocation {
                            row: self.row,
                            col: self.col - 2,
                        });
                        self.state = State::Start;
                        self.col += 1;
                        return next_index;
                    }
                    _ => {
                        self.tokens.push(Token::Operators(Operator::RightShift));
                        self.tokens_location.push(TokenLocation {
                            row: self.row,
                            col: self.col - 1,
                        });
                        self.state = State::Start;
                        return ptr_index;
                    }
                },
                State::State33 => {
                    //处理/* */注释
                    match self.chars[ptr_index] {
                        '*' => match self.chars[next_index] {
                            '/' => {
                                self.state = State::Start;
                                self.col += 2;
                                return next_index + 1;
                            }
                            _ => {
                                self.state = State::State33;
                                self.col += 1;
                                return next_index;
                            }
                        },
                        '\n' => {
                            self.col = 1;
                            self.row += 1;
                            self.state = State::State33;
                            return next_index;
                        }
                        _ => {
                            self.state = State::State33;
                            self.col += 1;
                            return next_index;
                        }
                    }
                }
                State::State4 => {
                    match self.chars[self.start_index] {
                        ';' => {
                            // 处理分号的逻辑...
                            self.tokens.push(Token::Delimiters(Delimiter::Semicolon));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        ',' => {
                            // 处理逗号的逻辑...
                            self.tokens.push(Token::Delimiters(Delimiter::Comma));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '(' => {
                            // 处理左括号的逻辑...
                            self.tokens
                                .push(Token::Delimiters(Delimiter::LeftParenthesis));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        ')' => {
                            // 处理右括号的逻辑...
                            self.tokens
                                .push(Token::Delimiters(Delimiter::RightParenthesis));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '[' => {
                            // 处理左方括号的逻辑...
                            self.tokens.push(Token::Delimiters(Delimiter::LeftBracket));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        ']' => {
                            // 处理右方括号的逻辑...
                            self.tokens.push(Token::Delimiters(Delimiter::RightBracket));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '{' => {
                            // 处理左大括号的逻辑...
                            self.tokens.push(Token::Delimiters(Delimiter::LeftBrace));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '}' => {
                            // 处理右大括号的逻辑...
                            self.tokens.push(Token::Delimiters(Delimiter::RightBrace));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '\\' => {
                            self.tokens.push(Token::Delimiters(Delimiter::Backslash));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col - 1,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '\'' => {
                            // 处理单引号的逻辑...
                            self.state = State::State41;
                            self.col += 1;
                            return next_index;
                        }
                        '"' => {
                            // 处理双引号的逻辑...
                            self.state = State::State42;
                            self.col += 1;
                            return next_index;
                        }
                        ':' => {
                            self.tokens
                                .push(Token::Delimiters(Delimiter::ConditionalSeparator));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        '?' => {
                            self.tokens
                                .push(Token::Delimiters(Delimiter::ConditionalOperator));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.col,
                            });
                            self.state = State::Start;
                            return ptr_index;
                        }
                        _ => {
                            self.errors.push(format!(
                                "Unkown key: {} in {}",
                                self.chars[ptr_index], ptr_index
                            ));
                            self.col += 1;
                            return next_index;
                        }
                    }
                }
                State::State41 => {
                    //处理单引号
                    match self.chars[ptr_index] {
                        '\'' => {
                            let str_slice: String =
                                self.chars[self.start_index..next_index].iter().collect();
                            self.tokens.push(Token::Strings(str_slice));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.start_index + self.col - ptr_index,
                            });
                            self.state = State::Start;
                            self.col += 1;
                            return next_index;
                        }

                        _ => {
                            if next_index == self.chars_len {
                                self.errors.push(format!("Unmatched key: '"));
                                return self.chars_len;
                            } else {
                                self.state = State::State41;
                                self.col += 1;
                                return next_index;
                            }
                        }
                    }
                }
                State::State42 => {
                    //处理双引号
                    match self.chars[ptr_index] {
                        '"' => {
                            let str_slice: String =
                                self.chars[self.start_index..next_index].iter().collect();
                            self.tokens.push(Token::Strings(str_slice));
                            self.tokens_location.push(TokenLocation {
                                row: self.row,
                                col: self.start_index + self.col - ptr_index,
                            });
                            self.state = State::Start;
                            self.col += 1;
                            return next_index;
                        }

                        _ => {
                            if next_index == self.chars_len {
                                self.errors.push(format!("Unmatched key: \""));
                                return self.chars_len;
                            } else {
                                self.state = State::State42;
                                self.col += 1;
                                return next_index;
                            }
                        }
                    }
                }
            }
        }
    }
}