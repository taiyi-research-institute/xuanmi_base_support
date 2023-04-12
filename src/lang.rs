#[macro_export]
macro_rules! stack {
    () => {{
        let var: i32 = 0;
        let sp = &var as *const _ as u64;
        sp
    }};
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}

pub fn typename<T>(_obj: T) -> &'static str {
    return std::any::type_name::<T>();
}

pub const PTRLEN: usize = std::mem::size_of::<usize>();
