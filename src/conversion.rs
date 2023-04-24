use crate::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{str, string::String, vec::Vec};

// #region conversion between JSON and object
/// Convert an object to json string.
pub fn obj_to_json<T>(obj: &T) -> Outcome<String>
where
    T: Serialize,
{
    let json: String = serde_json::to_string(obj).catch(
        EXN::SerializationException,
        &format!(
            "Failed to convert object of type `{}` to String",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(json)
}

/// Convert an object to formatted json string.
pub fn obj_to_json_pretty<T>(obj: &T) -> Outcome<String>
where
    T: Serialize,
{
    let json: String = serde_json::to_string_pretty(obj).catch(
        EXN::SerializationException,
        &format!(
            "Failed to convert object of type `{}` to String",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(json)
}

/// Convert a json string to an object.
pub fn json_to_obj<'a, T>(text: &'a str) -> Outcome<T>
where
    T: Deserialize<'a>,
{
    let obj: T = serde_json::from_str(text).catch(
        EXN::DeserializationException,
        &format!(
            "Failed to convert string to object of type `{}`",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(obj)
}
// #endregion

/// Convert a `serde_json::Value` to an object.
pub fn jval_to_obj<T>(val: serde_json::value::Value) -> Outcome<T>
where
    T: DeserializeOwned,
{
    let obj: T = serde_json::from_value(val).catch(
        EXN::DeserializationException,
        &format!(
            "jval_to_obj failed to convert serde_json::Value to object of type `{}`",
            std::any::type_name::<T>()
        ),
    )?;
    Ok(obj)
}

// #region conversion between string and bytes
/// Convert a string to a fat-pointer of utf-8 bytes.
pub fn str_to_u8p(text: &str) -> (*const u8, usize) {
    let bytes: &[u8] = text.as_bytes(); // a slice of u8
    let ptr: *const u8 = bytes.as_ptr();
    let len: usize = bytes.len();
    return (ptr, len);
}

/// Convert a fat-pointer of utf8-bytes to an owned String.
pub fn u8p_to_str(u8p: *const u8, len: usize) -> Outcome<String> {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(u8p, len) };
    match String::from_utf8(bytes.to_vec()) {
        Ok(txt) => {
            return Ok(txt);
        }
        Err(e) => {
            return Err(exception!(name = EXN::DeserializationException, src = e));
        }
    }
}

/// Convert a fat-pointer of bytes to a byte slice.
pub fn u8p_to_bslice(u8p: *const u8, len: usize) -> &'static [u8] {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(u8p, len) };
    return bytes;
}

/// Extract a string from a byte vector.
/// Note that the byte vector may contain one or more strings.
/// Each string is formatted as its length in bytes, appended by its utf-8 bytes.
pub fn str_from_partof_vecu8(buf: &Vec<u8>, beg: &mut usize, end: &mut usize) -> Outcome<String> {
    *beg = end.clone();
    *end += PTRLEN;
    let mut len: usize = 0;
    if let Some(item) = buf.get(*beg..*end) {
        len = usize::from_be_bytes(item.try_into().catch(
            EXN::DeserializationException,
            &format!("Bytes {:?} cannot be decoded as Big-Endian usize", &item),
        )?);
    } else {
        return Err(exception!(
            name = EXN::IndexOutOfBoundException,
            ctx = &format!(
                "When reading the length, index {:?} out of bound for bufsize {}",
                *beg..*end,
                buf.len()
            )
        ));
    }
    *beg = *end;
    *end += len;
    if let Some(item) = buf.get(*beg..*end) {
        let ret: String =
            String::from_utf8(item.to_vec()).catch(EXN::InvalidUTF8BytesException, "")?;
        return Ok(ret);
    } else {
        return Err(exception!(
            name = EXN::IndexOutOfBoundException,
            ctx = &format!(
                "When reading the bytes, index {:?} out of bound for bufsize {}",
                *beg..*end,
                buf.len()
            )
        ));
    }
}

pub fn bslice_into_partof_vecu8(
    bslice: &[u8],
    buf: &mut Vec<u8>,
    beg: &mut usize,
    end: &mut usize,
) -> Outcome<()> {
    *beg = end.clone();
    *end += PTRLEN;
    if let Some(item) = buf.get_mut(*beg..*end) {
        item.copy_from_slice(&bslice.len().to_be_bytes());
    } else {
        return Err(exception!(
            name = EXN::IndexOutOfBoundException,
            ctx = &format!(
                "When writing the length, index {:?} out of bound for bufsize {}",
                *beg..*end,
                buf.len()
            )
        ));
    }
    *beg = end.clone();
    *end += bslice.len();
    if let Some(item) = buf.get_mut(*beg..*end) {
        item.copy_from_slice(bslice);
        return Ok(());
    } else {
        return Err(exception!(
            name = EXN::IndexOutOfBoundException,
            ctx = &format!(
                "When writing the bytes, index {:?} out of bound for bufsize {}",
                *beg..*end,
                buf.len()
            )
        ));
    }
}

pub fn str_into_partof_vecu8(
    text: &str,
    buf: &mut Vec<u8>,
    beg: &mut usize,
    end: &mut usize,
) -> Outcome<()> {
    let bslice = text.as_bytes();
    bslice_into_partof_vecu8(bslice, buf, beg, end)
}
// #endregion

pub trait JsonDictGet<V>
where
    V: DeserializeOwned,
{
    fn get_must_provide(&self, field: &str) -> Outcome<V>;
    fn get_with_default(&self, field: &str, default: V) -> Outcome<V>;
}

impl<V> JsonDictGet<V> for serde_json::Map<String, serde_json::Value>
where
    V: DeserializeOwned,
{
    fn get_must_provide(&self, field: &str) -> Outcome<V> {
        match self.get(field) {
            Some(jval) => {
                let val: V = jval_to_obj(jval.clone()).catch(
                    "JsonInvalidFieldException",
                    &format!(
                        "The provided JSON field \"{}\" cannot be parsed into type `{}`",
                        field,
                        std::any::type_name::<V>()
                    ),
                )?;
                Ok(val)
            }
            None => {
                throw!(
                    name = "JsonNoRequiredFieldException",
                    ctx = &format!("The required JSON field \"{}\" is absent", field)
                );
            }
        }
    }

    fn get_with_default(&self, field: &str, default: V) -> Outcome<V> {
        match self.get(field) {
            Some(jval) => {
                let val: V = jval_to_obj(jval.clone()).catch(
                    "JsonInvalidFieldException",
                    &format!(
                        "The provided JSON field \"{}\" cannot be parsed into type `{}`",
                        field,
                        std::any::type_name::<V>()
                    ),
                )?;
                Ok(val)
            }
            None => Ok(default),
        }
    }
}
