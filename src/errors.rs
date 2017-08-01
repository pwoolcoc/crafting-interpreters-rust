#![allow(unused_doc_comment)]

error_chain!{
    foreign_links {
        ParseInt(::std::num::ParseIntError);
        ParseFloat(::std::num::ParseFloatError);
    }

    errors {
        LoxError(line: usize, msg: String) {
            description("lox error")
            display("[line: {}] Error: {}", line, msg)
        }
    }
}

