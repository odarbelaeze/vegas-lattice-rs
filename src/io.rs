//! Defines the `to_writer_lattice` function for serializing a type to a writer

use serde::ser;
use serde_json::error::Result;
use serde_json::ser::{Formatter, Serializer};
use std::io;

/// A formatter for serializing to a writer with a lattice style
///
/// Essentially formats the json as a lattice, with a max indent of 2.
/// This way the json is still readable, but not too verbose.
pub struct LatticeFormatter {
    current_indent: usize,
    has_value: bool,
    max_indent: usize,
    indent: &'static [u8],
}

impl LatticeFormatter {
    /// Create a new formatter with a max indent of 2
    pub fn new() -> Self {
        LatticeFormatter::with_max_indent(2)
    }

    /// Create a new formatter with a custom max indent
    pub fn with_max_indent(max_indent: usize) -> Self {
        LatticeFormatter {
            current_indent: 0,
            has_value: false,
            max_indent,
            indent: b"  ",
        }
    }

    #[inline]
    fn above_max_indent(&self) -> bool {
        self.current_indent > self.max_indent
    }

    #[inline]
    fn begin_colletion_item<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        match (first, self.above_max_indent()) {
            (true, false) => {
                writer.write_all(b"\n")?;
                indent(writer, self.current_indent, self.indent)
            }
            (false, false) => {
                writer.write_all(b",\n")?;
                indent(writer, self.current_indent, self.indent)
            }
            (false, true) => {
                writer.write_all(b", ")?;
                Ok(())
            }
            (true, true) => Ok(()),
        }
    }
}

impl Default for LatticeFormatter {
    fn default() -> Self {
        LatticeFormatter::new()
    }
}

impl Formatter for LatticeFormatter {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if !self.above_max_indent() && self.has_value {
            writer.write_all(b"\n")?;
            indent(writer, self.current_indent - 1, self.indent)?;
        }
        self.current_indent -= 1;
        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.begin_colletion_item(writer, first)
    }

    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if !self.above_max_indent() && self.has_value {
            writer.write_all(b"\n")?;
            indent(writer, self.current_indent - 1, self.indent)?;
        }
        self.current_indent -= 1;
        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.begin_colletion_item(writer, first)
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }
}

/// Serializes a type to a writer with a lattice style
#[inline]
pub fn to_writer_lattice<W, T>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + ser::Serialize,
{
    let mut ser = Serializer::with_formatter(writer, LatticeFormatter::new());
    value.serialize(&mut ser)?;
    Ok(())
}

/// Serialises a type to a vector with a lattice style
#[inline]
pub fn to_vec_lattice<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + ser::Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer_lattice(&mut writer, value)?;
    Ok(writer)
}

/// Serializes a type to a string with a lattice style
#[inline]
pub fn to_string_lattice<T>(value: &T) -> Result<String>
where
    T: ?Sized + ser::Serialize,
{
    let vec = to_vec_lattice(value)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        wr.write_all(s)?;
    }

    Ok(())
}
