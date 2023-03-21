#[macro_export]
macro_rules! stack {
    () => {{
        let var: i32 = 0;
        let sp = &var as *const _ as u64;
        sp
    }};
}

pub fn typename<T>(_obj: T) -> &'static str {
    return std::any::type_name::<T>();
}

pub const PTRLEN: usize = std::mem::size_of::<usize>();
