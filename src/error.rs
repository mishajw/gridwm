use std::io;
use std::sync;

error_chain! {
    foreign_links {
        ExecError(io::Error);
    }

    errors {
        RuntimeError(s: String) {
            display("Runtime error: {}", s)
        }

        LogicError(s: String) {
            display("Logic error: {}", s)
        }

        ParseError(s: String) {
            display("Parse error: {}", s)
        }
    }
}

