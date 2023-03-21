#![feature(track_caller)]
use std::{
    error::Error as StdError,
    result::Result as StdResult,
    fmt,
};

/// Make every fail-able function return StdResult<T, Box<dyn StdError>>.
pub type Outcome<T> = StdResult<T, Box<Exception>>;
// pub type Result<T> = StdResult<T, Box<Exception>>; // avoid name collision with std Result
use crate::EXN;

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
            name: EXN::UncategorizedException.to_string(),
            file: String::new(),
            line: 0,
            column: 0,
            context: None,
            inner: None,
        }
    }

    #[inline]
    pub fn file(&mut self, file: &str) -> &mut Self {
        self.file = file.to_string();
        self
    }

    #[inline]
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = match name {
            "" => self::EXN::UncategorizedException.to_string(),
            __ => name.to_string(),
        };
        self
    }

    #[inline]
    pub fn position(&mut self, line: u32, column: u32) -> &mut Self {
        self.line = line;
        self.column = column;
        self
    }

    #[inline]
    pub fn context(&mut self, ctx: &str) -> &mut Self {
        self.context = Some(ctx.to_string());
        self
    }

    #[inline]
    pub fn ctx(&mut self, ctx: &str) -> &mut Self {
        self.context = Some(ctx.to_string());
        self
    }

    // Keyword `impl` means: parameter `err` accepts any argument that implements `StdError`.
    #[inline]
    pub fn src(&mut self, err: impl StdError + 'static) -> &mut Self {
        self.inner = Some(Box::new(err));
        self
    }

    #[inline]
    pub fn cause(&mut self, err: impl StdError + 'static) -> &mut Self {
        self.inner = Some(Box::new(err));
        self
    }

    pub fn to_string(&self) -> String {
        let mut msg: String = format!("Exception \"{}\" occurs at \"{}", self.name, self.file);
        if self.line > 0 {
            msg += &format!(":{}", &self.line);
        }
        if self.column > 0 {
            msg += &format!(":{}", &self.column);
        }
        msg += "\"";
        if self.context.is_some() {
            let ctx = self.context.as_ref().unwrap().trim();
            if ctx != "" {
                msg += &format!("\nContext: {}", &self.context.as_ref().unwrap());
            }
        }
        if self.inner.is_some() {
            msg += &format!("\nCaused by:\n{:?}", &self.inner.as_ref().unwrap());
        }
        msg += "\n";
        return msg;
    }
}

pub trait TraitStdResultToOutcome<T, E> {
    fn handle(self, name: &str, ctx: &str) -> Outcome<T>;
}

impl<T, E: StdError + 'static> TraitStdResultToOutcome<T, E> for StdResult<T, E> {
    #[track_caller]
    fn handle(self, name: &str, ctx: &str) -> Outcome<T> {
        match self {
            Ok(v) => { return Ok(v); },
            Err(e) => { 
                let mut ex = std::boxed::Box::new(crate::Exception::new());
                let loc = std::panic::Location::caller();
                ex.file(loc.file()).position(loc.line(), loc.column()).name(name).ctx(ctx).src(e);
                return Err(ex);
            }
        }
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

impl std::error::Error for Box<Exception> {}

#[macro_export]
macro_rules! exception {
    ($($key:ident = $value:expr),*) => {
        {
            let mut ex = std::boxed::Box::new(crate::Exception::new());
            let loc = std::panic::Location::caller();
            ex.file(loc.file()).position(loc.line(), loc.column());
            $(
                ex.$key($value);
            )*
            ex
        }
    };
}

#[cfg(test)]
mod tests {
    #[macro_use] use crate::exception::*;

    #[test]
    fn test_exception() {
        use std::io::Write;
        let mut cout = std::io::stdout();
        use std::fs::File;
        match File::open("!!$%!$>TXT") { // deliberately generate an error for testing.
            Ok(fd) => { },
            Err(e) => { 
                let ex = exception!(src=e, ctx="Trying to read file from path \"!!$%!$>TXT\"");
                let ex2 = exception!(
                    name="IntendedException",
                    src=ex,
                    ctx="Deliberately wrap another Exception"
                );
                writeln!(cout, "{:#?}", &ex2).unwrap();
            },
        }
    }

    #[test]
    fn test_handle() {
        fn actual_test() -> Outcome<()> {
            use std::fs::File;
            let path = "!!$%!$>TXT";
            let f = File::open("!!$%!$>TXT").handle("IntendedException", &format!("Path \"{}\" has no file.", path))?;
            Ok(())
        }
        fn actual_test2() -> Outcome<()> {
            let x = actual_test().handle("AnotherIntendedException", "")?;
            Ok(())
        }
        use std::io::Write;
        let mut cout = std::io::stdout();
        let x = actual_test2();
        writeln!(cout, "{:#?}", x).unwrap();
    }
}