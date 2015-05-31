extern crate serde;

use std::fmt;
use std::io;

use std::string::FromUtf8Error;

use serde::ser;

/// A structure for implementing serialization to RON.
pub struct Encoder<W, F = CompactFormatter> {
    writer: W,
    formatter: F,
    first_line: bool,
}

impl<W> Encoder<W>
    where W: io::Write,
{
    pub fn new(writer: W) -> Self {
        Encoder::with_formatter(writer, CompactFormatter)
    }
}

impl<'a, W> Encoder<W, PrettyFormatter<'a>>
    where W: io::Write,
{
    pub fn pretty(writer: W) -> Self {
        Encoder::with_formatter(writer, PrettyFormatter::new())
    }
}

impl<W, F> Encoder<W, F>
    where W: io::Write,
          F: Formatter,
{
    /// Creates a new encoder whose output will be written in compact
    /// RON to the specified writer
    pub fn with_formatter(writer: W, formatter: F) -> Encoder<W, F> {
        Encoder {
            writer: writer,
            formatter: formatter,
            first_line: false,
        }
    }

    fn emit_constant<T: fmt::Display>(&mut self, v: T) -> io::Result<()> {
        write!(self.writer, "{}", v)
    }

    fn emit_escape<T: fmt::Display>(&mut self, v: T, escape: char) -> io::Result<()> {
        write!(self.writer, "{}{}{}", escape, v, escape)
    }

}

impl<W, F> ser::Serializer for Encoder<W, F>
    where W: io::Write,
          F: Formatter,
{
    type Error = io::Error;

    fn visit_bool(&mut self, v: bool)    -> io::Result<()> { self.emit_constant(v) }

    fn visit_usize(&mut self, v: usize)  -> io::Result<()> { self.emit_constant(v) }
    fn visit_u64(&mut self, v: u64)      -> io::Result<()> { self.emit_constant(v) }
    fn visit_u32(&mut self, v: u32)      -> io::Result<()> { self.emit_constant(v) }
    fn visit_u16(&mut self, v: u16)      -> io::Result<()> { self.emit_constant(v) }
    fn visit_u8(&mut self, v: u8)        -> io::Result<()> { self.emit_constant(v) }

    fn visit_isize(&mut self, v: isize)  -> io::Result<()> { self.emit_constant(v) }
    fn visit_i64(&mut self, v: i64)      -> io::Result<()> { self.emit_constant(v) }
    fn visit_i32(&mut self, v: i32)      -> io::Result<()> { self.emit_constant(v) }
    fn visit_i16(&mut self, v: i16)      -> io::Result<()> { self.emit_constant(v) }
    fn visit_i8(&mut self, v: i8)        -> io::Result<()> { self.emit_constant(v) }

    fn visit_f64(&mut self, v: f64)      -> io::Result<()> { self.emit_constant(v) }
    fn visit_f32(&mut self, v: f32)      -> io::Result<()> { self.emit_constant(v) }

    fn visit_char(&mut self, v: char)    -> io::Result<()> { self.emit_escape(v, '\'') }
    fn visit_str(&mut self, v: &str)     -> io::Result<()> { self.emit_escape(v, '\"') }

    fn visit_unit(&mut self) -> io::Result<()> {
        self.writer.write_all(b"()")
    }

    fn visit_none(&mut self) -> io::Result<()> {
        self.visit_unit()
    }

    fn visit_some<V>(&mut self, value: V) -> io::Result<()> 
        where V: ser::Serialize
    {
        value.serialize(self)
    }

    fn visit_enum_unit(&mut self, name: &str, variant: &str) -> io::Result<()> {
        self.emit_escape(name, '¶')
    }

    fn visit_seq<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor,
    {   
        match visitor.len() {
            Some(len) if len == 0 => {
                self.writer.write_all(b"[]")
            },
            _ => {
                try!(self.formatter.open(&mut self.writer, b'['));
                self.first_line = true;

                // TODO: maybe fix
                try!(visitor.visit(self));
 
                self.formatter.close(&mut self.writer, b']')
            }
        }
    }

    fn visit_enum_seq<V>(&mut self, name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::SeqVisitor
    {
        try!(self.formatter.open(&mut self.writer, b'{'));
        try!(self.formatter.comma(&mut self.writer, true));
        try!(self.visit_str(variant));
        try!(self.formatter.colon(&mut self.writer));
        try!(self.visit_seq(visitor));
        self.formatter.close(&mut self.writer, b'}')
    }

    fn visit_seq_elt<T>(&mut self, value: T) -> io::Result<()>
        where T: ser::Serialize
    {
        try!(self.formatter.comma(&mut self.writer, self.first_line));
        self.first_line = true;

        value.serialize(self)
    }

    fn visit_map<V>(&mut self, mut visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        match visitor.len() {
            Some(len) if len == 0 => {
                self.writer.write_all(b"{}")
            }
            _ => {
                try!(self.formatter.open(&mut self.writer, b'{'));
                self.first_line = true;

                /*while let Some(()) = */try!(visitor.visit(self)); //{ }

                self.formatter.close(&mut self.writer, b'}')
            }
        }
    }

    fn visit_enum_map<V>(&mut self, name: &str, variant: &str, visitor: V) -> io::Result<()>
        where V: ser::MapVisitor,
    {
        try!(self.formatter.open(&mut self.writer, b'{'));
        try!(self.formatter.comma(&mut self.writer, true));
        try!(self.visit_str(variant));
        try!(self.formatter.colon(&mut self.writer));
        try!(self.visit_map(visitor));

        self.formatter.close(&mut self.writer, b'}')
    }

    fn visit_map_elt<K, V>(&mut self, key: K, value: V) -> io::Result<()>
        where K: ser::Serialize,
              V: ser::Serialize,
    {
        try!(self.formatter.comma(&mut self.writer, self.first_line));
        self.first_line = false;

        try!(key.serialize(self));
        try!(self.formatter.colon(&mut self.writer));
        value.serialize(self)
    }

    fn format() -> &'static str {
        "ron"
    }

}

pub trait Formatter {
    fn open<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write;

    fn comma<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where W: io::Write;

    fn colon<W>(&mut self, writer: &mut W) -> io::Result<()>
        where W: io::Write;

    fn close<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write;
}

pub struct CompactFormatter;

impl Formatter for CompactFormatter {
    fn open<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write,
    {
        writer.write_all(&[ch])
    }

    fn comma<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where W: io::Write,
    {
        if first {
            Ok(())
        } else {
            writer.write_all(b",")
        }
    }

    fn colon<W>(&mut self, writer: &mut W) -> io::Result<()>
        where W: io::Write,
    {
        writer.write_all(b":")
    }

    fn close<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write,
    {
        writer.write_all(&[ch])
    }
}

pub struct PrettyFormatter<'a> {
    current_indent: usize,
    indent: &'a [u8],
}

impl<'a> PrettyFormatter<'a> {
    fn new() -> Self {
        PrettyFormatter::with_indent(b"  ")
    }

    fn with_indent(indent: &'a [u8]) -> Self {
        PrettyFormatter {
            current_indent: 0,
            indent: indent,
        }
    }
}

impl<'a> Formatter for PrettyFormatter<'a> {
    fn open<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write,
    {
        self.current_indent += 1;
        writer.write_all(&[ch])
    }

    fn comma<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where W: io::Write,
    {
        if first {
            try!(writer.write_all(b"\n"));
        } else {
            try!(writer.write_all(b",\n"));
        }

        indent(writer, self.current_indent, self.indent)
    }

    fn colon<W>(&mut self, writer: &mut W) -> io::Result<()>
        where W: io::Write,
    {
        writer.write_all(b": ")
    }

    fn close<W>(&mut self, writer: &mut W, ch: u8) -> io::Result<()>
        where W: io::Write,
    {
        self.current_indent -= 1;
        try!(writer.write(b"\n"));
        try!(indent(writer, self.current_indent, self.indent));

        writer.write_all(&[ch])
    }
}

fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
    where W: io::Write,
{
    for _ in 0 .. n {
        try!(wr.write_all(s));
    }

    Ok(())
}

#[inline]
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> io::Result<()>
    where W: io::Write,
          T: ser::Serialize,
{
    let mut enc = Encoder::new(writer);
    try!(value.serialize(&mut enc));
    Ok(())
}

#[inline]
pub fn to_writer_pretty<W, T>(writer: &mut W, value: &T) -> io::Result<()>
    where W: io::Write,
          T: ser::Serialize,
{
    let mut enc = Encoder::pretty(writer);
    try!(value.serialize(&mut enc));
    Ok(())
}

/// Encode the specified struct into a json `[u8]` buffer.
#[inline]
pub fn to_vec<T>(value: &T) -> Vec<u8>
    where T: ser::Serialize,
{
    // We are writing to a Vec, which doesn't fail. So we can ignore
    // the error.
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, value).unwrap();
    writer
}

/// Encode the specified struct into a json `[u8]` buffer.
#[inline]
pub fn to_vec_pretty<T>(value: &T) -> Vec<u8>
    where T: ser::Serialize,
{
    // We are writing to a Vec, which doesn't fail. So we can ignore
    // the error.
    let mut writer = Vec::with_capacity(128);
    to_writer_pretty(&mut writer, value).unwrap();
    writer
}

/// Encode the specified struct into a json `String` buffer.
#[inline]
pub fn to_string<T>(value: &T) -> Result<String, FromUtf8Error>
    where T: ser::Serialize
{
    let vec = to_vec(value);
    String::from_utf8(vec)
}

/// Encode the specified struct into a json `String` buffer.
#[inline]
pub fn to_string_pretty<T>(value: &T) -> Result<String, FromUtf8Error>
    where T: ser::Serialize
{
    let vec = to_vec_pretty(value);
    String::from_utf8(vec)
}
