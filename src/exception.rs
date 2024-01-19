use std::{fmt, result::Result as StdResult};

pub struct Exception {
    name: String,
    file: String,
    line: u32,
    column: u32,
    context: Option<String>,
    inner: Option<Box<dyn std::string::ToString + Send + Sync>>,
}

unsafe impl Send for Exception {}
unsafe impl Sync for Exception {}

/// Make every fail-able function return StdResult<T, Box<dyn StdError>>.
pub type Outcome<T> = StdResult<T, Box<Exception>>;

impl Exception {
    pub fn new() -> Box<Self> {
        Box::new(Exception {
            name: "UnknownException".to_string(),
            file: String::new(),
            line: 0,
            column: 0,
            context: None,
            inner: None,
        })
    }

    pub fn dummy() -> Box<Self> {
        Box::new(Exception {
            name: String::new(),
            file: String::new(),
            line: 0,
            column: 0,
            context: None,
            inner: None,
        })
    }

    #[inline]
    pub fn set_file(&mut self, file: &str) -> &mut Self {
        self.file = file.to_string();
        self
    }

    #[inline]
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    #[inline]
    pub fn set_line(&mut self, line: u32) -> &mut Self {
        self.line = line;
        self
    }

    #[inline]
    pub fn set_column(&mut self, column: u32) -> &mut Self {
        self.column = column;
        self
    }

    #[inline]
    pub fn set_context(&mut self, ctx: &str) -> &mut Self {
        self.context = Some(ctx.to_string());
        self
    }

    #[inline]
    pub fn set_caused_by(
        &mut self,
        err: impl std::string::ToString + Send + Sync + 'static,
    ) -> &mut Self {
        self.inner = Some(Box::new(err));
        self
    }

    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn get_context(&self) -> Option<&str> {
        match &self.context {
            Some(ctx) => Some(ctx),
            None => None,
        }
    }
}

/// std::string::ToString has a default to_string() implementation
/// for any type satisfying std::fmt::Display, and the type needn't be Sized.
impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
                msg += &format!("\nContext: {}", ctx);
            }
        }
        if self.inner.is_some() {
            // if self.inner is Some(Exception)
            msg += &format!(
                "\nCaused by:\n{}",
                &self.inner.as_ref().unwrap().to_string()
            );
        }
        msg += "\n";
        write!(f, "{}", msg)
    }
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::error::Error for Exception {}

pub trait TraitStdResultToOutcome<T, E> {
    /// Add detailed information to an error.
    /// `name` and `context` is provided by the caller.
    /// source path, line and column is automatically injected by the compiler.
    fn catch(self, name: &str, ctx: &str) -> Outcome<T>;

    /// equivalent to `catch("", "")`.
    /// Note that the exception's name will be shown as "DummyException".
    fn catch_(self) -> Outcome<T>;
}

impl<T, E> TraitStdResultToOutcome<T, E> for StdResult<T, E>
where
    E: fmt::Display + Send + Sync + 'static,
{
    #[track_caller]
    fn catch(self, name: &str, ctx: &str) -> Outcome<T> {
        match self {
            Ok(v) => {
                return Ok(v);
            }
            Err(e) => {
                let mut ex = Exception::new();
                let loc = std::panic::Location::caller();
                let (file, line, column) = (loc.file(), loc.line(), loc.column());
                ex.set_name(name)
                    .set_file(file)
                    .set_line(line)
                    .set_column(column)
                    .set_context(ctx)
                    .set_caused_by(e);
                return Err(ex);
            }
        }
    }

    #[track_caller]
    fn catch_(self) -> Outcome<T> {
        match self {
            Ok(v) => {
                return Ok(v);
            }
            Err(e) => {
                let mut ex = Exception::new();
                let loc = std::panic::Location::caller();
                let (file, line, column) = (loc.file(), loc.line(), loc.column());
                ex.set_name("")
                    .set_file(file)
                    .set_line(line)
                    .set_column(column)
                    .set_context("")
                    .set_caused_by(e);
                return Err(ex);
            }
        }
    }
}

#[macro_export]
macro_rules! exception {
    ($name:expr, $ctx:expr) => {{
        let mut ex = Exception::new();
        let loc = std::panic::Location::caller();
        ex.set_name($name)
            .set_file(loc.file())
            .set_line(loc.line())
            .set_column(loc.column())
            .set_context($ctx);
        ex
    }};
}

#[macro_export]
macro_rules! throw {
    ($name:expr, $ctx:expr) => {{
        use crate::exception::Exception;
        let mut ex = Exception::new();
        let loc = std::panic::Location::caller();
        ex.set_name($name)
            .set_file(loc.file())
            .set_line(loc.line())
            .set_column(loc.column())
            .set_context($ctx);
        return Err(ex);
    }};
}

#[macro_export]
macro_rules! assert_throw {
    ($cond:expr, $name:expr, $ctx:expr) => {
        if !($cond) {
            use crate::exception::Exception;
            let mut ex = Exception::new();
            let loc = std::panic::Location::caller();
            let ctx = format!("Condition: {}\nExplanation: {}", stringify!($cond), $ctx);
            ex.set_name($name)
                .set_file(loc.file())
                .set_line(loc.line())
                .set_column(loc.column())
                .set_context(&ctx);
            return Err(ex);
        }
    };
    ($cond:expr, $ctx:expr) => {
        if !($cond) {
            use crate::exception::Exception;
            let mut ex = Exception::new();
            let loc = std::panic::Location::caller();
            let ctx = format!("Condition: {}\nExplanation: {}", stringify!($cond), $ctx);
            ex.set_name("AssertionFailedException")
                .set_file(loc.file())
                .set_line(loc.line())
                .set_column(loc.column())
                .set_context(&ctx);
            return Err(ex);
        }
    };
    ($cond:expr) => {
        if !($cond) {
            use crate::exception::Exception;
            let mut ex = Exception::new();
            let loc = std::panic::Location::caller();
            let ctx = format!("Condition: {}", stringify!($cond));
            ex.set_name("AssertionFailedException")
                .set_file(loc.file())
                .set_line(loc.line())
                .set_column(loc.column())
                .set_context(&ctx);
            return Err(ex);
        }
    };
}

pub trait TraitStdOptionToOutcome<T> {
    fn ifnone(self, name: &str, ctx: &str) -> Outcome<T>;
    fn ifnone_(self) -> Outcome<T>;
}

impl<T> TraitStdOptionToOutcome<T> for std::option::Option<T> {
    #[track_caller]
    fn ifnone(self, name: &str, ctx: &str) -> Outcome<T> {
        match self {
            Some(t) => Ok(t),
            None => throw!(name, ctx),
        }
    }

    #[track_caller]
    fn ifnone_(self) -> Outcome<T> {
        match self {
            Some(t) => Ok(t),
            None => throw!("", ""),
        }
    }
}
