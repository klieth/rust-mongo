
use std::fmt::{Show,Formatter,FormatError};
use std::collections::treemap::{TreeMap, Entries};

pub enum BSONData {
    BSONObject(BSON),
    BSONInt(i32),
    BSONString(String),
}
impl Show for BSONData {
    fn fmt(&self, f: &mut Formatter) -> Result<(),FormatError> {
        match *self {
            BSONObject(ref o) => write!(f, "Object: {}", o),
            BSONInt(ref i) => write!(f, "Number: {}", i),
            BSONString(ref s) => write!(f, "String: {}", s),
        }
    }
}

pub struct BSON {
    elements: TreeMap<String, BSONData>
}
impl BSON {
    pub fn new() -> BSON {
        BSON { elements: TreeMap::new() }
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
                        }
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
