use serde_json::error::Result;
use serde_json::ser::{Serializer, Formatter};
use serde::ser;
use std::io;


pub struct LatticeFormatter {
    current_indent: usize,
    has_value: bool,
    max_indent: usize,
    indent: &'static [u8],
}


impl LatticeFormatter {
    pub fn new() -> Self {
        LatticeFormatter::with_max_indent(2)
    }

    pub fn with_max_indent(max_indent: usize) -> Self {
        LatticeFormatter {
            current_indent: 0,
            has_value: false,
            max_indent: max_indent,
            indent: b"  ",
        }
    }

    #[inline]
    fn above_max_indent(&self) -> bool {
        self.current_indent > self.max_indent
    }

    #[inline]
    fn begin_colletion_item<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        match (first, self.above_max_indent()) {
            (true, false) => {
                try!(writer.write_all(b"\n"));
                indent(writer, self.current_indent, self.indent)
            },
            (false, false) => {
                try!(writer.write_all(b",\n"));
                indent(writer, self.current_indent, self.indent)
            },
            (false, true) => {
                try!(writer.write_all(b", "));
                Ok(())
            },
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
    fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {

        self.current_indent -= 1;

        if !self.above_max_indent() && self.has_value {
            try!(writer.write_all(b"\n"));
            try!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        self.begin_colletion_item(writer, first)
    }

    #[inline]
    fn end_array_value<W: ?Sized>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        if !self.above_max_indent() && self.has_value {
            try!(writer.write_all(b"\n"));
            try!(indent(writer, self.current_indent - 1, self.indent));
        }
        self.current_indent -= 1;
        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: io::Write,
    {
        self.begin_colletion_item(writer, first)
    }

    #[inline]
    fn begin_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W: ?Sized>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        self.has_value = true;
        Ok(())
    }
}


#[inline]
pub fn to_writer_lattice<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: ser::Serialize,
{
    let mut ser = Serializer::with_formatter(writer, LatticeFormatter::new());
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


fn indent<W: ?Sized>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: io::Write,
{
    for _ in 0..n {
        try!(wr.write_all(s));
    }

    Ok(())
}
