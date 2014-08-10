
use std::{io,mem,fmt};
use std::collections::TreeMap;

mod serialize;

#[deriving(PartialEq)]
pub enum Bson {
    ObjectId(Id),
    Float(f64),
    String(String),
    Boolean(bool),
    List(List),
    Object(Object),
    Null,
}

pub type List = Vec<Bson>;
pub type Object = TreeMap<String, Bson>;
pub type Id = Vec<u8>;

pub enum ErrorCode {
    InvalidSyntax,
    // TODO - add the rest
}
impl fmt::Show for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            InvalidSyntax => "Invalid syntax",
        })
    }
}

#[deriving(Show)]
pub enum ParserError {
    /// msg, line, col
    SyntaxError(ErrorCode, uint, uint),
    IoError(io::IoErrorKind, &'static str)
}

pub type BuilderError = ParserError;

#[deriving(Show)]
pub enum DecoderError {
    ParseError(ParserError),
    // TODO - add the rest
}

pub type EncodeResult = io::IoResult<Vec<u8>>;
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
    Encoder::buffer_encode(object)
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
            let mut buf = object.encode(mem::transmute(&mut Encoder::new(&mut m))).unwrap();
            let _ = buf.shift(); // Take of the leading 'object' type
            m.write(buf.as_slice());
        }
        m.unwrap()
    }
}
impl<'a> serialize::Encoder<io::IoError> for Encoder<'a> {
    fn build_nil(&mut self) -> EncodeResult {
        unimplemented!();
    }
    fn build_bool(&mut self, v: bool) -> EncodeResult {
        unimplemented!();
    }
    fn build_f64(&mut self, v: f64) -> EncodeResult {
        let mut w = io::MemWriter::new();
        try!(w.write_u8(0x01));
        try!(w.write_le_f64(v));
        Ok(w.unwrap())
    }
    fn build_str(&mut self, v: &str) -> EncodeResult {
        let mut w = io::MemWriter::new();
        try!(w.write_u8(0x02));
        for c in v.chars() {
            try!(w.write_char(c));
        }
        try!(w.write_u8(0x00));
        Ok(w.unwrap())
    }
    fn build_seq(&mut self, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        unimplemented!();
    }
    fn build_seq_elt(&mut self, idx: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        unimplemented!();
    }
    fn build_map(&mut self, len: uint, f: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        let mut w = io::MemWriter::new();
        try!(w.write_u8(0x03));
        let buf = try!(f(self));
        try!(w.write_le_u32(buf.len() as u32 + 5));
        try!(w.write(buf.as_slice()));
        try!(w.write_u8(0x00));
        Ok(w.unwrap())
    }
    fn build_map_item(&mut self, idx: uint, key: |&mut Encoder<'a>| -> EncodeResult, val: |&mut Encoder<'a>| -> EncodeResult) -> EncodeResult {
        let mut w = io::MemWriter::new();
        let mut k = try!(key(self));
        let _ = k.shift();
        let mut v = try!(val(self));
        let t = v.shift().unwrap();
        try!(w.write_u8(t));
        try!(w.write(k.as_slice()));
        match t {
            0x02 => try!(w.write_le_u32(v.len() as u32)),
            _ => ()
        }
        try!(w.write(v.as_slice()));
        println!("{} {} {}", k, v, t);
        Ok(w.unwrap())
    }
}

impl<E: serialize::Encoder<S>, S> serialize::Encodable<E, S> for Bson {
    fn encode(&self, e: &mut E) -> Result<Vec<u8>, S> {
        match *self {
            ObjectId(ref v) => {
                e.build_custom(|e| {
                    let mut w = io::MemWriter::new();
                    w.write_u8(0x07).unwrap();
                    w.write(v.as_slice()).unwrap();
                    Ok(w.unwrap())
                })
            },
            Float(v)      => v.encode(e),
            String(ref v) => v.encode(e),
            Boolean(v)    => v.encode(e),
            List(ref v)   => v.encode(e),
            Object(ref v) => v.encode(e),
            Null => { e.build_nil() }
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
    fn read_bool(&mut self) -> DecodeResult<bool> {
        unimplemented!();
    }
    fn read_f64(&mut self) -> DecodeResult<f64> {
        unimplemented!();
    }
    fn read_str(&mut self) -> DecodeResult<String> {
        unimplemented!();
    }
    fn read_map<T>(&mut self, f: |&mut Decoder, uint| -> DecodeResult<T>) -> DecodeResult<T> {
        unimplemented!();
    }
    fn read_map_item<T1,T2>(&mut self, idx: uint, key: |&mut Decoder| -> DecodeResult<T1>, val: |&mut Decoder| -> DecodeResult<T2>) -> DecodeResult<(T1,T2)> {
        unimplemented!();
    }
}
