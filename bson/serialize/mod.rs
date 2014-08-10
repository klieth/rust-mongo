
use std::collections::TreeMap;

pub trait Encoder<E> {
    fn build_nil(&mut self) -> Result< Vec<u8> , E>;
    fn build_bool(&mut self, v: bool) -> Result< Vec<u8>, E>;
    fn build_f64(&mut self, v: f64) -> Result< Vec<u8>, E>;
    fn build_str(&mut self, v: &str) -> Result< Vec<u8>, E>;

    fn build_seq(&mut self, len:uint, f: |&mut Self| -> Result<Vec<u8>,E>) -> Result<Vec<u8>,E>;
    fn build_seq_elt(&mut self, idx: uint, f: |&mut Self| -> Result<Vec<u8>,E>) -> Result<Vec<u8>,E>;

    fn build_map(&mut self, len: uint, f: |&mut Self| -> Result< Vec<u8>, E>) -> Result< Vec<u8>, E>;
    fn build_map_item(&mut self, idx: uint, key: |&mut Self| -> Result< Vec<u8>, E>, val: |&mut Self| -> Result< Vec<u8>, E>) -> Result<Vec<u8>, E>;

    fn build_custom(&mut self, f: |&mut Self| -> Result< Vec<u8>, E>) -> Result<Vec<u8>, E> {
        f(self)
    }
}

pub trait Encodable<S: Encoder<E>, E> {
    fn encode(&self, s: &mut S) -> Result< Vec<u8>, E>;
}

impl<E, S:Encoder<E>> Encodable<S, E> for bool {
    fn encode(&self, s: &mut S) -> Result< Vec<u8>, E> {
        s.build_bool(*self)
    }
}

impl<E, S:Encoder<E>> Encodable<S, E> for f64 {
    fn encode(&self, s: &mut S) -> Result< Vec<u8>, E> {
        s.build_f64(*self)
    }
}

impl<'a, E, S:Encoder<E>> Encodable<S, E> for &'a str {
    fn encode(&self, s: &mut S) -> Result< Vec<u8>, E> {
        s.build_str(*self)
    }
}
impl<E, S:Encoder<E>> Encodable<S, E> for String {
    fn encode(&self, s: &mut S) -> Result< Vec<u8>, E> {
        s.build_str(self.as_slice())
    }
}

impl<E, S:Encoder<E>, T:Encodable<S,E>> Encodable<S,E> for Vec<T> {
    fn encode(&self, s:&mut S) -> Result< Vec<u8>, E> {
        s.build_seq(self.len(), |s| {
            let mut buf = Vec::new();
            for (i,e) in self.iter().enumerate() {
                buf.push_all(try!(s.build_seq_elt(i, |s| e.encode(s))).as_slice());
            }
            Ok(buf)
        })
    }
}

impl<E, S:Encoder<E>, K:Encodable<S,E>+PartialEq+Ord, V:Encodable<S,E>+PartialEq> Encodable<S,E> for TreeMap<K,V> {
    fn encode(&self, e: &mut S) -> Result< Vec<u8>, E> {
        e.build_map(self.len(), |e| {
            let mut i = 0;
            let mut buf: Vec<u8> = Vec::new();
            for (key, val) in self.iter() {
                buf.push_all(try!(e.build_map_item(i, |e| key.encode(e), |e| val.encode(e))).as_slice());
                i += 1;
            }
            Ok(buf)
        })
    }
}

pub trait Decoder<E> {
    fn read_bool(&mut self) -> Result< bool, E>;
    fn read_f64(&mut self) -> Result< f64, E>;
    fn read_str(&mut self) -> Result<String, E>;
    fn read_map<T>(&mut self, f: |&mut Self, uint| -> Result<T, E>) -> Result<T, E>;
    fn read_map_item<T1,T2>(&mut self, idx: uint, key: |&mut Self| -> Result<T1, E>, val: |&mut Self| -> Result<T2, E>) -> Result<(T1,T2), E>;
}

pub trait Decodable<D: Decoder<E>, E> {
    fn decode(d: &mut D) -> Result<Self, E>;
}

impl<E, D: Decoder<E>> Decodable<D, E> for bool {
    fn decode(d: &mut D) -> Result<bool, E> {
        d.read_bool()
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for f64 {
    fn decode(d: &mut D) -> Result<f64, E> {
        d.read_f64()
    }
}

impl<E, D: Decoder<E>> Decodable<D, E> for String {
    fn decode(d: &mut D) -> Result<String, E> {
        d.read_str()
    }
}

impl<E, D:Decoder<E>, K:Decodable<D,E>+PartialEq+Ord, V:Decodable<D,E>+PartialEq> Decodable<D,E> for TreeMap<K,V> {
    fn decode(d: &mut D) -> Result<TreeMap<K,V>, E> {
        d.read_map(|d, len| {
            let mut map = TreeMap::new();
            for i in range(0u, len) {
                let item = try!(d.read_map_item(i, |d| Decodable::decode(d), |d| Decodable::decode(d)));
                match item {
                    (key, val) => {map.insert(key, val);},
                }
            }
            Ok(map)
        })
    }
}
