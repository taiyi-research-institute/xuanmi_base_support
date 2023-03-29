use std::{
    error::Error as StdError,
    result::Result as StdResult,
    fmt,
};

/// Make every fail-able function return StdResult<T, Box<dyn StdError>>.
pub type Outcome<T> = StdResult<T, Box<Exception>>;
// pub type Result<T> = StdResult<T, Box<Exception>>; // avoid name collision with std Result
use crate::EXN;
use anyhow::Error as AnyhowError;

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
    pub fn new() -> Box<Self> {
        std::boxed::Box::new(Exception {
            name: EXN::UncategorizedException.to_string(),
            file: String::new(),
            line: 0,
            column: 0,
            context: None,
            inner: None,
        })
    }

    pub fn dummy() -> Box<Self> {
        Box::new(
            Exception {
                name: EXN::DummyException.to_string(),
                file: String::new(),
                line: 0,
                column: 0,
                context: None,
                inner: None,
            }
        )
    }

    #[inline]
    pub fn file(&mut self, file: &str) -> &mut Self {
        self.file = file.to_string();
        self
    }

    #[inline]
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = match name {
            "" => self::EXN::DummyException.to_string(),
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
    pub fn line(&mut self, line: u32) -> &mut Self {
        self.line = line;
        self
    }

    #[inline]
    pub fn column(&mut self, column: u32) -> &mut Self {
        self.column = column;
        self
    }

    #[inline]
    pub fn col(&mut self, column: u32) -> &mut Self {
        self.column(column)
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
    fn catch(self, name: &str, ctx: &str) -> Outcome<T>;
    fn catch_replace(self, name: &str, ctx: &str, src: impl StdError + 'static) -> Outcome<T>;
}

impl<T, E: StdError + 'static> TraitStdResultToOutcome<T, E> for StdResult<T, E> {
    #[track_caller]
    fn catch(self, name: &str, ctx: &str) -> Outcome<T> {
        match self {
            Ok(v) => { return Ok(v); },
            Err(e) => { 
                let mut ex = crate::Exception::new();
                let loc = std::panic::Location::caller();
                ex.file(loc.file()).position(loc.line(), loc.column()).name(name).ctx(ctx).src(e);
                return Err(ex);
            }
        }
    }

    #[track_caller]
    fn catch_replace(self, 
        name: &str,
        ctx: &str, 
        src: impl StdError + 'static
    ) -> Outcome<T> {
        match self {
            Ok(v) => { return Ok(v); },
            Err(_) => {
                let mut ex = crate::Exception::new();
                let loc = std::panic::Location::caller();
                ex.file(loc.file()).position(loc.line(), loc.column()).name(name).ctx(ctx).src(src);
                return Err(ex);
            }
        }
    }
}

pub trait TraitAnyhowResultToOutcome<T> {
    fn catch_anyhow(self, name: &str, ctx: &str) -> Outcome<T>;
}

impl<T> TraitAnyhowResultToOutcome<T> for StdResult<T, AnyhowError> {
    #[track_caller]
    fn catch_anyhow(self, name: &str, ctx: &str) -> Outcome<T> {
        match self {
            Ok(v) => { return Ok(v); },
            Err(e) => { 
                let mut ex = crate::Exception::new();
                let loc = std::panic::Location::caller();
                ex.file(loc.file()).position(loc.line(), loc.column()).name(name).ctx(
                    &format!("{}\nCaused by:{:?}", ctx, &e)
                );
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
            let mut ex = Exception::new();
            let loc = std::panic::Location::caller();
            ex.file(loc.file()).position(loc.line(), loc.column());
            $(
                ex.$key($value);
            )*
            ex
        }
    };
}

#[macro_export]
macro_rules! throw {
    ($($key:ident = $value:expr),*) => {
        {
            let mut ex = Exception::new();
            let loc = std::panic::Location::caller();
            ex.file(loc.file()).position(loc.line(), loc.column());
            $(
                ex.$key($value);
            )*
            return Err(ex);
        }
    };
}


pub trait TraitStdOptionToOutcome<T> {
    fn if_none(self, name: &str, ctx: &str) -> Outcome<T>;
    fn if_none_wrap(self, name: &str, ctx: &str, src: impl StdError + 'static) -> Outcome<T>;
}

impl<T> TraitStdOptionToOutcome<T> for std::option::Option<T> {
    #[track_caller]
    fn if_none(self, name: &str, ctx: &str) -> Outcome<T> {
        match self {
            Some(t) => Ok(t),
            None => throw!(name=name, ctx=ctx),
        }
    }

    #[track_caller]
    fn if_none_wrap(self, name: &str, ctx: &str, src: impl StdError + 'static) -> Outcome<T> {
        match self {
            Some(t) => Ok(t),
            None => throw!(name=name, ctx=ctx, src=src),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::EXN;

    #[test]
    fn test_exception() {
        use std::fs::File;
        match File::open("!!$%!$>TXT") { // deliberately generate an error for testing.
            Ok(__) => { },
            Err(e) => { 
                let ex = exception!(src=e, ctx="Trying to read file from path \"!!$%!$>TXT\"");
                let ex2 = exception!(
                    name="IntendedException",
                    src=ex,
                    ctx="Deliberately wrap another Exception"
                );
                println!("{:#?}", &ex2);
            },
        }
    }

    #[test]
    fn test_catch() {
        fn actual_test() -> Outcome<()> {
            use std::fs::File;
            let path = "!!$%!$>TXT";
            let _f = File::open("!!$%!$>TXT").catch("IntendedException", &format!("Path \"{}\" has no file.", path))?;
            Ok(())
        }
        fn actual_test2() -> Outcome<()> {
            let _x = actual_test().catch("AnotherIntendedException", "")?;
            Ok(())
        }
        let x = actual_test2();
        println!("{:#?}", x);
    }

    #[test]
    fn test_throw() {
        fn div() -> Outcome<f64> {
            let (a, b, eps): (f64, f64, f64) = (1.0, 0.0, 1.0 / 4096 as f64);
            if b.abs() < eps {
                throw!(
                    name=EXN::ArithmeticException,
                    ctx=&format!("Cannot divide a={:.4} by b={:.4}", a, b)
                );
            } else {
                return Ok(a/b);
            }
        }
        println!("{:?}", div());
    }
    
    #[test]
    fn test_option() {
        let x: Option<i32> = None;
        let x_out = x.if_none("IntendedException", "x has no value");
        println!("{:#?}", x_out);
    }
}