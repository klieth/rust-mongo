
use std::fmt::{Show,Formatter,FormatError};
use std::collections::treemap::{TreeMap, Entries};
use std::slice::Items;

pub enum BSONData {
    BSONObject(BSON),
    BSONInt(i32),
    BSONString(String),
    BSONId(Vec<u8>),
}
impl BSONData {
    fn get_name(it: &mut Items<u8>) -> String {
        let mut name = String::new();
        loop {
            let c = *it.next().unwrap();
            match c {
                0x00 => break,
                _ => name.push_char(c as char),
            }
        }
        name
    }
    fn deserialize_int(it: &mut Items<u8>) -> (String, BSONData) {
        let mut name = BSONData::get_name(it);
        let num: i32 = *it.next().unwrap() as i32 
            | (*it.next().unwrap() as i32 << 8) 
            | (*it.next().unwrap() as i32 << 16) 
            | (*it.next().unwrap() as i32 << 24);
        (name, BSONInt(num))
    }
    fn deserialize_string(it: &mut Items<u8>) -> (String, BSONData) {
        let mut name = BSONData::get_name(it);
        let len: i32 = *it.next().unwrap() as i32 
            | (*it.next().unwrap() as i32 << 8) 
            | (*it.next().unwrap() as i32 << 16) 
            | (*it.next().unwrap() as i32 << 24);
        let mut s = String::new();
        for _ in range(0,len-1) {
            s.push_char((*it.next().unwrap()) as char);
        }
        it.next().unwrap();
        (name, BSONString(s))
    }
    fn deserialize_object(it: &mut Items<u8>) -> (String, BSONData) {
        let mut name = BSONData::get_name(it);
        let mut vec: Vec<u8> = Vec::new();
        vec.push(*it.next().unwrap());
        vec.push(*it.next().unwrap());
        vec.push(*it.next().unwrap());
        vec.push(*it.next().unwrap());
        let len: i32 = vec[0] as i32 
            | (vec[1] as i32 << 8) 
            | (vec[2] as i32 << 16) 
            | (vec[3] as i32 << 24);
        for _ in range(4,len) {
            vec.push(*it.next().unwrap());
        }
        (name, BSONObject(BSON::deserialize(vec)))
    }
    fn deserialize_id(it: &mut Items<u8>) -> (String, BSONData) {
        let mut name = BSONData::get_name(it);
        let mut id = Vec::new();
        for _ in range(0u,12) {
            id.push(*it.next().unwrap());
        }
        (name, BSONId(id))
    }
}
impl Show for BSONData {
    fn fmt(&self, f: &mut Formatter) -> Result<(),FormatError> {
        match *self {
            BSONObject(ref o) => write!(f, "Object: {}", o),
            BSONInt(ref i) => write!(f, "Number: {}", i),
            BSONString(ref s) => write!(f, "String: {}", s),
            BSONId(ref i) => write!(f, "ObjectId: {}", i),
        }
    }
}
impl PartialEq for BSONData {
    fn eq(&self, other: &BSONData) -> bool {
        match *self {
            BSONObject(ref o1) => {
                match *other {
                    BSONObject(ref o2) => o1.eq(o2),
                    _ => false
                }
            },
            BSONInt(ref i1) => {
                match *other {
                    BSONInt(ref i2) => i1.eq(i2),
                    _ => false,
                }
            },
            BSONString(ref s1) => {
                match *other {
                    BSONString(ref s2) => s1.eq(s2),
                    _ => false,
                }
            },
            BSONId(ref i1) => {
                match *other {
                    BSONId(ref i2) => i1.eq(i2),
                    _ => false
                }
            }
        }
    }
    fn ne(&self, other: &BSONData) -> bool {
        unimplemented!();
        false
    }
}

pub struct BSON {
    elements: TreeMap<String, BSONData>
}
impl BSON {
    pub fn new() -> BSON {
        BSON { elements: TreeMap::new() }
    }
    pub fn deserialize(data: Vec<u8>) -> BSON {
        let mut result = BSON { elements: TreeMap::new() };
        let mut it = data.iter();
        println!("Vec: {}", data);
        let bytes: uint = *it.next().unwrap() as uint 
            | (*it.next().unwrap() as uint << 8) 
            | (*it.next().unwrap() as uint << 16) 
            | (*it.next().unwrap() as uint << 24);
        println!("Bytes: {}", bytes);
        loop {
            match *it.next().unwrap() {
                0x02 => {
                    match BSONData::deserialize_string(&mut it) {
                        (name, value) => result.insert(name, value),
                    }
                },
                0x03 => {
                    match BSONData::deserialize_object(&mut it) {
                        (name, value) => result.insert(name, value),
                    }
                },
                0x07 => {
                    match BSONData::deserialize_id(&mut it) {
                        (name, value) => result.insert(name, value),
                    }
                },
                0x10 => {
                    match BSONData::deserialize_int(&mut it) {
                        (name, value) => result.insert(name, value),
                    }
                },
                0x00 => break,
                _ => unimplemented!(),
            }
        }
        println!("Result: {}", result);
        result
    }
    pub fn insert(&mut self, key: String, data: BSONData) {
        self.elements.insert(key, data);
    }
    pub fn get_iter(&self) -> Entries<String, BSONData> {
        self.elements.iter()
    }
    pub fn serialize(&self) -> Vec<u8> {
        let push_le_i32 = |vec: &mut Vec<u8>, i: i32| {
            vec.push(  (i & 0x000000FF) as u8);
            vec.push( ((i & 0x0000FF00) >> 8) as u8);
            vec.push( ((i & 0x00FF0000) >> 16) as u8);
            vec.push( ((i & 0xFF000000) >> 24) as u8);
        };
        let push_le_uint = |vec: &mut Vec<u8>, i: uint| {
            vec.push(  (i & 0x000000FF) as u8);
            vec.push( ((i & 0x0000FF00) >> 8) as u8);
            vec.push( ((i & 0x00FF0000) >> 16) as u8);
            vec.push( ((i & 0xFF000000) >> 24) as u8);
        };
        let mut v: Vec<Vec<u8>> = Vec::new();
        for elem in self.elements.iter() {
            match elem {
                (name, data) => {
                    let mut local: Vec<u8> = Vec::new();
                    match *data {
                        BSONObject(ref obj) => {
                            local.push(0x03);
                            for c in name.as_slice().chars() {
                                local.push(c as u8);
                            }
                            local.push(0x00);
                            local.push_all(obj.serialize().as_slice());
                        },
                        BSONInt(ref i) => {
                            local.push(0x10);
                            for c in name.as_slice().chars() {
                                local.push(c as u8);
                            }
                            local.push(0x00);
                            push_le_i32(&mut local, *i);
                        },
                        BSONString(ref s) => {
                            local.push(0x02);
                            for c in name.as_slice().chars() {
                                local.push(c as u8);
                            }
                            local.push(0x00);
                            let s_slice = s.as_slice();
                            push_le_uint(&mut local, s.len() + 1);
                            for c in s_slice.chars() {
                                local.push(c as u8);
                            }
                            local.push(0x00);
                        },
                        BSONId(ref i) => {
                            local.push(0x07);
                            for c in name.as_slice().chars() {
                                local.push(c as u8);
                            }
                            local.push(0x00);
                            assert_eq!(i.len(), 12u);
                            for v in i.iter() {
                                local.push(*v);
                            }
                        },
                    }
                    v.push(local);
                },
            }
        }
        let mut final: Vec<u8> = Vec::new();
        push_le_uint(&mut final, 4 + v.iter().fold(0, |current, next| {current + next.len()}) + 1);
        for vec in v.iter() {
            final.push_all(vec.as_slice());
        }
        final.push(0x00);
        final
    }
}
impl Show for BSON {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        for v in self.elements.iter() {
            try!(write!(f, "{}", v));
        }
        write!(f, "\n")
    }
}
impl PartialEq for BSON {
    fn eq(&self, other: &BSON) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }
        let mut s = self.elements.iter();
        let mut o = other.elements.iter();
        for i in range(0,self.elements.len()) {
            if !s.next().unwrap().eq(&o.next().unwrap()) {
                return false;
            }
        }
        true
    }
    fn ne(&self, other: &BSON) -> bool {
        unimplemented!();
        false
    }
}
