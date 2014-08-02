
extern crate serialize;

use std::io;
use std::mem;
use std::collections::TreeMap;

#[deriving(PartialEq)]
pub enum Bson {
    Float(f64),
    String(String),
    Boolean(bool),
    List(List),
    Object(Object),
    Null,
}

pub type List = Vec<Bson>;
pub type Object = TreeMap<String, Bson>;

pub enum ErrorCode {
    InvalidSyntax,
    // TODO - add the rest
}

pub enum ParserError {
    /// msg, line, col
    SyntaxError(ErrorCode, uint, uint),
    IoError(io::IoErrorKind, &'static str)
}

pub type BuilderError = ParserError;

pub enum DecoderError {
    ParseError(ParserError),
    // TODO - add the rest
}

pub type EncodeResult = io::IoResult<()>;
pub type DecodeResult<T> = Result<T, DecoderError>;

pub fn decode<T: serialize::Decodable<Decoder, DecoderError>>(v: Vec<u8>) -> DecodeResult<T> {
    let bson = match from_vec(v) {
        Ok(b) => b,
        Err(e) => unimplemented!(),
    };
    let mut decoder = Decoder::new(bson);
    serialize::Decodable::decode(&mut decoder)
}

pub fn encode<'a, T: serialize::Encodable<Encoder<'a>, io::IoError>>(object: &T) -> Vec<u8> {
    let buff = Encoder::buffer_encode(object);
    buff
}

pub struct Encoder<'a> {
    stack: Vec<Vec<u8>>,
    writer: &'a mut io::Writer,
}
impl<'a> Encoder<'a> {
    pub fn new(writer: &'a mut io::Writer) -> Encoder {
        Encoder { writer: writer, stack: Vec::new() }
    }
    pub fn buffer_encode<T: serialize::Encodable<Encoder<'a>, io::IoError>>(object: &T) -> Vec<u8> {
        let mut m = io::MemWriter::new();
        unsafe {
            object.encode(mem::transmute(&mut Encoder::new(&mut m)));
        }
        m.unwrap()
    }
}
impl<'a> serialize::Encoder<io::IoError> for Encoder<'a> {
    fn emit_nil(&mut self) -> EncodeResult {
        println!("emit nil");
        unimplemented!();
    }
    fn emit_u64 (&mut self, v: u64) -> EncodeResult {
        println!("emit u64");
        unimplemented!();
    }
    fn emit_u32 (&mut self, v: u32) -> EncodeResult {
        println!("emit u32");
        unimplemented!();
    }
    fn emit_u16 (&mut self, v: u16) -> EncodeResult {
        println!("emit u16");
        unimplemented!();
    }
    fn emit_u8  (&mut self, v: u8) -> EncodeResult {
        println!("emit u8");
        unimplemented!();
    }
    fn emit_uint(&mut self, v: uint) -> EncodeResult {
        println!("emit uint");
        unimplemented!();
    }

    fn emit_i64 (&mut self, v: i64) -> EncodeResult {
        println!("emit i64");
        unimplemented!();
    }
    fn emit_i32 (&mut self, v: i32) -> EncodeResult {
        println!("emit i32");
        unimplemented!();
    }
    fn emit_i16 (&mut self, v: i16) -> EncodeResult {
        println!("emit i16");
        unimplemented!();
    }
    fn emit_i8  (&mut self, v: i8) -> EncodeResult {
        println!("emit i8");
        unimplemented!();
    }
    fn emit_int (&mut self, v: int) -> EncodeResult {
        println!("emit int");
        unimplemented!();
    }
    
    fn emit_bool(&mut self, v: bool) -> EncodeResult {
        println!("emit bool");
        unimplemented!();
    }

    fn emit_f64(&mut self, v: f64) -> EncodeResult {
        println!("emit f64");
        let mut t = io::MemWriter::new();
        try!(t.write_u8(0x01));
        self.stack.push(t.unwrap());
        // push an empty vec to trick the 'len'
        self.stack.push(Vec::new());
        let mut w = io::MemWriter::new();
        try!(w.write_le_f64(v));
        self.stack.push(w.unwrap());
        Ok(())
    }
    fn emit_f32(&mut self, v: f32) -> EncodeResult {
        println!("emit f32");
        unimplemented!();
    }

    fn emit_char(&mut self, v: char) -> EncodeResult {
        println!("emit char");
        unimplemented!();
    }
    fn emit_str(&mut self, v: &str) -> EncodeResult {
        println!("emit str");
        let mut t = io::MemWriter::new();
        try!(t.write_u8(0x02));
        self.stack.push(t.unwrap());
        // push the length
        let mut l = io::MemWriter::new();
        l.write_le_u32(v.len() as u32 + 1);
        self.stack.push(l.unwrap());
        let mut w = io::MemWriter::new();
        try!(w.write_str(v));
        try!(w.write_u8(0x00));
        self.stack.push(w.unwrap());
        Ok(())
    }

    fn emit_enum(&mut self, name: &str, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit enum");
        unimplemented!();
    }
    fn emit_enum_variant(&mut self, name: &str, id: uint, cnt: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit enum variant");
        unimplemented!();
    }
    fn emit_enum_variant_arg(&mut self, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit enum variant arg");
        unimplemented!();
    }
    fn emit_enum_struct_variant(&mut self, name: &str, id: uint, cnt: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit enum struct variant");
        unimplemented!();
    }
    fn emit_enum_struct_variant_field(&mut self, name: &str, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit enum struct variant field");
        unimplemented!();
    }

    fn emit_struct(&mut self, name: &str, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit struct");
        unimplemented!();
    }
    fn emit_struct_field(&mut self, name: &str, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit struct field");
        unimplemented!();
    }

    fn emit_tuple(&mut self, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit tuple");
        unimplemented!();
    }
    fn emit_tuple_arg(&mut self, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit tuple arg");
        unimplemented!();
    }
    fn emit_tuple_struct(&mut self, name: &str, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit tuple struct");
        unimplemented!();
    }
    fn emit_tuple_struct_arg(&mut self, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit tuple struct arg");
        unimplemented!();
    }

    fn emit_option(&mut self, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit option");
        unimplemented!();
    }
    fn emit_option_none(&mut self) -> EncodeResult {
        println!("emit option none");
        unimplemented!();
    }
    fn emit_option_some(&mut self, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit option some");
        unimplemented!();
    }

    fn emit_seq(&mut self, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit seq");
        unimplemented!();
    }
    fn emit_seq_elt(&mut self, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit seq elt");
        unimplemented!();
    }

    fn emit_map(&mut self, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit map");
        try!(f(self));
        println!("items: {} ", self.stack.len());
        let mut w = io::MemWriter::new();
        for item in self.stack.iter() {
            w.write(item.as_slice());
        }
        let v = w.unwrap();
        println!("Final buffer: {}", v);
        try!(self.writer.write_le_u32(v.len() as u32 + 5));
        try!(self.writer.write(v.as_slice()));
        self.writer.write_u8(0x00)
    }
    fn emit_map_elt_key(&mut self, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit map elt key");
        f(self)
    }
    fn emit_map_elt_val(&mut self, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        println!("emit map elt val");
        try!(f(self));
        let val = self.stack.pop().unwrap();
        println!("val: {}", val);
        let len = self.stack.pop().unwrap();
        println!("len: {}", len);
        let t = self.stack.pop().unwrap();
        println!("t: {}", t);
        let key = self.stack.pop().unwrap();
        println!("key: {}", key);
        let _ = self.stack.pop();
        let _ = self.stack.pop();
        let mut w = io::MemWriter::new();
        try!(w.write(t.as_slice()));
        try!(w.write(key.as_slice()));
        try!(w.write(len.as_slice()));
        try!(w.write(val.as_slice()));
        self.stack.push(w.unwrap());
        Ok(())
    }
}

// line 835
impl<E: serialize::Encoder<S>, S> serialize::Encodable<E, S> for Bson {
    fn encode(&self, e: &mut E) -> Result<(), S> {
        match *self {
            Float(v)      => v.encode(e),
            String(ref v) => v.encode(e),
            Boolean(v)    => v.encode(e),
            List(ref v)   => v.encode(e),
            Object(ref v) => v.encode(e),
            Null => { e.emit_nil() }
        }
    }
}

// line 1796
pub fn from_vec(v: Vec<u8>) -> Result<Bson, BuilderError> {
    unimplemented!();
}

// line 1802
pub struct Decoder {
    stack: Vec<Bson>
}
impl Decoder {
    pub fn new(bson: Bson) -> Decoder {
        Decoder { stack: vec![bson] }
    }
}
impl serialize::Decoder<DecoderError> for Decoder {
    fn read_nil(&mut self) -> DecodeResult<()> {
        unimplemented!();
    }
    fn read_u64 (&mut self) -> DecodeResult<u64> { unimplemented!(); }
    fn read_u32 (&mut self) -> DecodeResult<u32> { unimplemented!(); }
    fn read_u16 (&mut self) -> DecodeResult<u16> { unimplemented!(); }
    fn read_u8  (&mut self) -> DecodeResult<u8> { unimplemented!(); }
    fn read_uint(&mut self) -> DecodeResult<uint> { unimplemented!(); }

    fn read_i64 (&mut self) -> DecodeResult<i64> { unimplemented!(); }
    fn read_i32 (&mut self) -> DecodeResult<i32> { unimplemented!(); }
    fn read_i16 (&mut self) -> DecodeResult<i16> { unimplemented!(); }
    fn read_i8  (&mut self) -> DecodeResult<i8> { unimplemented!(); }
    fn read_int (&mut self) -> DecodeResult<int> { unimplemented!(); }
    
    fn read_bool(&mut self) -> DecodeResult<bool> { unimplemented!(); }

    fn read_f64(&mut self) -> DecodeResult<f64> { unimplemented!(); }
    fn read_f32(&mut self) -> DecodeResult<f32> { unimplemented!(); }

    fn read_char(&mut self) -> DecodeResult<char> { unimplemented!(); }
    fn read_str(&mut self) -> DecodeResult<String> { unimplemented!(); }

    fn read_enum<T>(&mut self, name: &str, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_enum_variant<T>(&mut self, names: &[&str], f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_enum_variant_arg<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_enum_struct_variant<T>(&mut self, names: &[&str], f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_enum_struct_variant_field<T>(&mut self, name: &str, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }

    fn read_struct<T>(&mut self, name: &str, len: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_struct_field<T>(&mut self, name: &str, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }

    fn read_tuple<T>(&mut self, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_tuple_arg<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_tuple_struct<T>(&mut self, name: &str, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_tuple_struct_arg<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }

    fn read_option<T>(&mut self, f: |&mut Decoder, bool| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }

    fn read_seq<T>(&mut self, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_seq_elt<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }

    fn read_map<T>(&mut self, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_map_elt_key<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_map_elt_val<T>(&mut self, idx: uint, f: |&mut Decoder| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
}
