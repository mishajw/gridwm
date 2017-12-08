use std::io;

error_chain! {
    foreign_links {
        ExecError(io::Error);
    }

    errors {
        LogicError(s: String) {
            display("Logic error: {}", s)
        }

        ParseError(s: String) {
            display("Parse error: {}", s)
        }
    }
}

