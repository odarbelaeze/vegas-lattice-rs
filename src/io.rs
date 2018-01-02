use serde_json::error::Result;
use serde_json::ser::{Serializer, Formatter};
use serde::ser;
use std::io;


pub struct LatticeFormatter {
}


impl Formatter for LatticeFormatter {
}


#[inline]
pub fn to_writer_lattice<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ser::Serialize,
{
    let mut ser = Serializer::with_formatter(writer, LatticeFormatter {});
    try!(value.serialize(&mut ser));
    Ok(())
}


#[inline]
pub fn to_vec_lattice<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: ser::Serialize,
{
    let mut writer = Vec::with_capacity(128);
    try!(to_writer_lattice(&mut writer, value));
    Ok(writer)
}


#[inline]
pub fn to_string_lattice<T: ?Sized>(value: &T) -> Result<String>
where
    T: ser::Serialize,
{
    let vec = try!(to_vec_lattice(value));
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
