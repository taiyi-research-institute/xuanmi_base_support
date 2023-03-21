use std::{
    error::Error as StdError,
    result::Result as StdResult,
    fmt,
};

macro_rules! register_exception_names {
    ($($arg:ident),+) => {
        $(
            pub const $arg: &'static str = stringify!($arg);
        )+
    };
}

/// Register new exception names here.
pub mod ExceptionNames {
    register_exception_names!(
        UncategorizedException,
        HttpPostException,
        DeserializationException
    );
}

/// Make every fail-able function return StdResult<T, Box<dyn StdError>>.
pub type Outcome<T> = StdResult<T, Box<dyn StdError>>;
pub type Result<T> = StdResult<T, Box<dyn StdError>>;

pub struct Exception {
    name: String,
    file: String,
    line: u32,
    column: u32,
    // function: &'static str,
    context: Option<String>,
    inner: Option<Box<dyn StdError>>,
}

impl Exception {
    pub fn new() -> Self {
        Exception {
            name: ExceptionNames::UncategorizedException.to_string(),
            file: String::new(),
            line: 0,
            column: 0,
            context: None,
            inner: None,
        }
    }

    pub fn file(mut self, file: &str) -> Self {
        self.file = file.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = match name {
            "" => self::ExceptionNames::UncategorizedException.to_string(),
            __ => name.to_string(),
        };
        self
    }

    pub fn position(mut self, line: u32, column: u32) -> Self {
        self.line = line;
        self.column = column;
        self
    }

    pub fn context(mut self, ctx: &str) -> Self {
        self.context = Some(ctx.to_string());
        self
    }

    // Keyword `dyn` means: parameter `err` accepts any argument that implements `StdError`.
    pub fn wrap(mut self, err: impl StdError + 'static) -> Self {
        self.inner = Some(Box::new(err));
        self
    }

    pub fn to_string(&self) -> String {
        let mut msg: String = format!("Exception \"{}\" occurs at \"{}\"", self.name, self.file);
        if self.line > 0 {
            msg += &format!(":{}", &self.line);
        }
        if self.column > 0 {
            msg += &format!(":{}", &self.column);
        }
        if self.context.is_some() {
            msg += &format!("\nContext: {}", &self.context.as_ref().unwrap());
        }
        if self.inner.is_some() {
            msg += &format!("\nCaused by:\n{:?}", &self.inner.as_ref().unwrap());
        }
        msg += "\n";
        return msg;
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

#[macro_export]
macro_rules! exception {
    () => {
        {
            let loc = std::panic::Location::caller();
            crate::Exception::new().file(loc.file()).position(loc.line(), loc.column())
        }
    };
    ($inner:expr) => {
        {
            let loc = std::panic::Location::caller();
            crate::Exception::new().file(loc.file()).position(loc.line(), loc.column()).wrap($inner)
        }
    };
    ($inner:expr, $ctx:expr) => {
        {
            let loc = std::panic::Location::caller();
            crate::Exception::new().file(loc.file()).position(loc.line(), loc.column())
                .wrap($inner)
                .context($ctx)
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_exception() {
        use std::io::Write;
        let mut cout = std::io::stdout();
        use crate::exception;
        use std::fs::File;
        match File::open("!!$%!$>TXT") { // deliberately generate an error.
            Ok(fd) => { },
            Err(e) => { 
                let ex = exception!(e, "Trying to read file from path \"!!$%!$>TXT\"");
                writeln!(cout, "{:#?}", &ex).unwrap();
            },
        }
    }
}