use crate::EXN::*;
use crate::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{self, Path, PathBuf};
use std::{str, string::String};

pub fn read_str_from_file(path: &str) -> Outcome<String> {
    let mut fd = File::open(path).catch(
        IOException,
        &format!("File::open failed to access \"{}\"", path),
    )?;
    let len = fd.seek(SeekFrom::End(0)).catch(
        IOException,
        &format!("File::seek failed to access \"{}\"", path),
    )? as usize;
    let _0 = fd.seek(SeekFrom::Start(0)).catch(
        IOException,
        &format!("File::seek failed to access \"{}\"", path),
    )?;
    let mut buf = vec![0_u8; len];
    let _len = fd.read(buf.as_mut_slice()).catch(
        IOException,
        &format!("File::seek failed to access \"{}\"", path),
    )?;
    let ret = String::from_utf8(buf).catch(InvalidUTF8BytesException, "")?;
    Ok(ret)
}

pub fn write_str_to_file(path: &str, text: &str) -> Outcome<usize> {
    let mut fd = File::create(path).catch(
        IOException,
        &format!("File::create failed to access \"{}\"", path),
    )?;
    let data = text.as_bytes();
    let len_to_write = data.len();
    let len_written = fd.write(data).catch(
        IOException,
        &format!(
            "File::write failed to write {} bytes to path \"{}\"",
            len_to_write, path
        ),
    )?;
    if len_to_write != len_written {
        throw!(
            name = IOException,
            ctx = &format!(
                "To path \"{}\", expected to write {} bytes, actually wrote {} bytes",
                path, len_to_write, len_written
            )
        );
    }
    Ok(len_to_write)
}

pub trait LexicalAbspath {
    fn to_lexical_abspath(&self) -> Outcome<String>;
}

impl<STR> LexicalAbspath for STR
where
    STR: AsRef<str> + core::fmt::Display,
{
    fn to_lexical_abspath(&self) -> Outcome<String> {
        let expanded = shellexpand::full(&self).catch(
            PathResolutionException,
            &format!("Cannot expand `~` or/and env-vars from path: {}", &self),
        )?;
        let abspath = {
            let path: &Path = Path::new(expanded.as_ref());
            let mut abspath = if path.is_absolute() {
                PathBuf::new()
            } else {
                std::env::current_dir().catch(PathResolutionException, "Invalid CWD.")?
            };
            for component in path.components() {
                match component {
                    path::Component::CurDir => {}
                    path::Component::ParentDir => {
                        abspath.pop();
                    }
                    component @ _ => {
                        abspath.push(component.as_os_str());
                    }
                }
            }
            abspath.to_string_lossy().as_ref().to_string()
        };
        Ok(abspath)
    }
}

#[cfg(test)]
mod tests {
    use crate::{LexicalAbspath, Outcome};

    #[test]
    pub fn test_abspath() -> Outcome<()> {
        let x = "~/test".to_lexical_abspath()?;
        let y = String::from("~/../../$LANG").to_lexical_abspath()?;
        println!("x={}", x);
        println!("y={}", y);
        Ok(())
    }
}
