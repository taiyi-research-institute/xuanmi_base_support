use crate::{Outcome, TraitStdResultToOutcome, TraitStdOptionToOutcome, EXN, PTRLEN};

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
    String::from_utf8(bytes.to_vec()).catch_()
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
    let item = buf.get(*beg..*end).if_none(
        EXN::IndexOutOfBoundException,
        &format!(
            "When reading the length, index {:?} out of bound for bufsize {}",
            *beg..*end,
            buf.len()
        ),
    )?;
    len = usize::from_be_bytes(item.try_into().catch(
        EXN::DeserializationException,
        &format!("Bytes {:?} cannot be decoded as Big-Endian usize", &item),
    )?);
    *beg = *end;
    *end += len;
    let item = buf.get(*beg..*end).if_none(
        EXN::IndexOutOfBoundException,
        &format!(
            "When reading the bytes, index {:?} out of bound for bufsize {}",
            *beg..*end,
            buf.len()
        ),
    )?;
    let ret: String = String::from_utf8(item.to_vec()).catch_()?;
    return Ok(ret);
}

pub fn bslice_into_partof_vecu8(
    bslice: &[u8],
    buf: &mut Vec<u8>,
    beg: &mut usize,
    end: &mut usize,
) -> Outcome<()> {
    *beg = end.clone();
    *end += PTRLEN;
    let buflen = buf.len();
    let item = buf.get_mut(*beg..*end).if_none(
        EXN::IndexOutOfBoundException,
        &format!(
            "When writing the length, index {:?} out of bound for bufsize {}",
            *beg..*end,
            buflen
        ),
    )?;
    item.copy_from_slice(&bslice.len().to_be_bytes());
    *beg = end.clone();
    *end += bslice.len();
    let item = buf.get_mut(*beg..*end).if_none(
        EXN::IndexOutOfBoundException,
        &format!(
            "When writing the bytes, index {:?} out of bound for bufsize {}",
            *beg..*end,
            buflen
        ),
    )?;
    item.copy_from_slice(bslice);
    Ok(())
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