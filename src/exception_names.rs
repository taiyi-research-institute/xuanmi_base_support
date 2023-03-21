macro_rules! register_exception_names {
    ($($arg:ident),+) => {
        $(
            pub const $arg: &'static str = stringify!($arg);
        )+
    };
}

register_exception_names!(
    UncategorizedException,
    HttpPostException,
    DeserializationException
);
