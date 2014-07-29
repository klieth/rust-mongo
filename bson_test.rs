
use std::io::File;

use bson::{BSON,BSONString,BSONInt,BSONObject};


mod bson;

fn show_bson(v: &Vec<u8>) {
    print!("[");
    let mut is_first = true;
    for b in v.iter() {
        if is_first {
            is_first = false;
        } else {
            print!(",");
        }
        print!("{:x}",*b);
    }
    println!("]");
}
fn write_bson(v: &Vec<u8>, name: &str) {
    let mut file = File::create(&Path::new(name));
    for b in v.iter() {
        file.write_u8(*b).unwrap();
    }
}

#[test]
fn empty() {
    let empty = BSON::new();
    let serialized = empty.serialize();
    show_bson(&serialized);
    assert_eq!(serialized, vec!(0x05,0x00,0x00,0x00,0x00));
}

#[test]
fn int32() {
    let mut bson = BSON::new();
    bson.insert("int32".to_string(), BSONInt(10));
    let serialized = bson.serialize();
    show_bson(&serialized);
    assert_eq!(serialized, vec!(0x10,0x00,0x00,0x00,
                                0x10,
                                0x69,0x6e,0x74,0x33,0x32,0x00,
                                0x0A,0x00,0x00,0x00,
                                0x00));
}

#[test]
fn string() {
    let mut bson = BSON::new();
    bson.insert("string".to_string(), BSONString("sample".to_string()));
    let serialized = bson.serialize();
    show_bson(&serialized);
    assert_eq!(serialized, vec!(0x18,0x00,0x00,0x00,
                                0x02,
                                0x73,0x74,0x72,0x69,0x6e,0x67,0x00,
                                0x07,0x00,0x00,0x00,
                                0x73,0x61,0x6d,0x70,0x6c,0x65,0x00,
                                0x00));
}

#[test]
fn embedded() {
    let mut embed = BSON::new();
    embed.insert("this".to_string(), BSONString("is embedded".to_string()));
    embed.insert("negative".to_string(), BSONInt(-5));
    let mut bson = BSON::new();
    bson.insert("hello".to_string(), BSONString("world".to_string()));
    bson.insert("another".to_string(), BSONString("thing".to_string()));
    bson.insert("number".to_string(), BSONInt(10));
    bson.insert("embedded".to_string(), BSONObject(embed));
    let done = bson.serialize();
    show_bson(&done);
    assert_eq!(done, vec!(0x68,0x00,0x00,0x00,
                          0x02,
                          0x61,0x6e,0x6f,0x74,0x68,0x65,0x72,0x00,
                          0x06,0x00,0x00,0x00,
                          0x74,0x68,0x69,0x6e,0x67,0x00,
                          0x03,
                          0x65,0x6d,0x62,0x65,0x64,0x64,0x65,0x64,0x00,
                          0x29,0x00,0x00,0x00,
                          0x10,
                          0x6e,0x65,0x67,0x61,0x74,0x69,0x76,0x65,0x00,
                          0xFB,0xFF,0xFF,0xFF,
                          0x02,
                          0x74,0x68,0x69,0x73,0x00,
                          0x0c,0x00,0x00,0x00,
                          0x69,0x73,0x20,0x65,0x6d,0x62,0x65,0x64,0x64,0x65,0x64,0x00,
                          0x00,
                          0x02,
                          0x68,0x65,0x6c,0x6c,0x6f,0x00,
                          0x06,0x00,0x00,0x00,
                          0x77,0x6f,0x72,0x6c,0x64,0x00,
                          0x10,
                          0x6e,0x75,0x6d,0x62,0x65,0x72,0x00,
                          0x0A,0x00,0x00,0x00,
                          0x00));
}
