
use std::io::File;
use std::collections::TreeMap;

use bson::Bson;
use bson::serialize::Decodable;

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
fn serialize_empty() {
    let map = TreeMap::new();
    let serialized = bson::encode(&bson::Object(map));
    show_bson(&serialized);
    assert_eq!(serialized, vec!(0x05,0x00,0x00,0x00,0x00));
}
/*
#[test]
fn deserialize_empty() {
    /*
    let empty: TreeMap<String, bson::Bson> = TreeMap::new();
    let built = bson::decode(vec!(0x05,0x00,0x00,0x00,0x00)).unwrap();
    assert_eq!(bson::Object(empty), built);
    */
}
*/

#[test]
fn serialize_f64() {
    let mut map = TreeMap::new();
    map.insert("f".to_string(), bson::Float(10f64));
    let serialized = bson::encode(&bson::Object(map));
    show_bson(&serialized);
    assert_eq!(serialized, vec!(0x10,0x00,0x00,0x00,
                                0x01,
                                0x66,0x00,
                                0x00,0x00,0x00,0x00,0x00,0x00,0x24,0x40,
                                0x00));
}
struct FloatStruct {
    f: f64
}
impl bson::serialize::Decodable<bson::Decoder, bson::DecoderError> for FloatStruct {
    fn decode(d: &mut Decoder) -> Result<FloatStruct, io::IoError> {
        let map = try!(d.read_map());
        // TODO - FloatStruct { f: map.get("f") }
    }
}
#[test]
fn deserialize_f64() {
    let v = vec!(0x10,0x00,0x00,0x00,
                 0x01,
                 0x66,0x00,
                 0x00,0x00,0x00,0x00,0x00,0x00,0x24,0x40,
                 0x00);
    let obj: FloatStruct = bson::decode(v).unwrap();
}
/*
#[test]
fn deserialize_int32() {
    let mut correct = BSON::new();
    correct.insert("int32".to_string(), BSONInt(10));
    let built = BSON::deserialize(vec!(0x10,0x00,0x00,0x00,
                                       0x10,
                                       0x69,0x6e,0x74,0x33,0x32,0x00,
                                       0x0A,0x00,0x00,0x00,
                                       0x00));
    assert_eq!(correct, built);
}
*/

#[test]
fn serialize_string() {
    let mut map = TreeMap::new();
    map.insert("string".to_string(), bson::String("sample".to_string()));
    let serialized = bson::encode(&bson::Object(map));
    show_bson(&serialized);
    assert_eq!(serialized, vec!(0x18,0x00,0x00,0x00,
                                0x02,
                                0x73,0x74,0x72,0x69,0x6e,0x67,0x00,
                                0x07,0x00,0x00,0x00,
                                0x73,0x61,0x6d,0x70,0x6c,0x65,0x00,
                                0x00));
}
/*
#[test]
fn deserialize_string() {
    let mut correct = BSON::new();
    correct.insert("string".to_string(), BSONString("sample".to_string()));
    let built = BSON::deserialize(vec!(0x18,0x00,0x00,0x00,
                                       0x02,
                                       0x73,0x74,0x72,0x69,0x6e,0x67,0x00,
                                       0x07,0x00,0x00,0x00,
                                       0x73,0x61,0x6d,0x70,0x6c,0x65,0x00,
                                       0x00));
    assert_eq!(correct, built);
}
*/

#[test]
fn serialize_basic_mongo_document() {
    let mut doc = TreeMap::new();
    doc.insert("_id".to_string(), bson::ObjectId(vec!(0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0A,0x0B)));
    doc.insert("data".to_string(), bson::String("something".to_string()));
    let serialized = bson::encode(&bson::Object(doc));
    assert_eq!(serialized, vec!(0x2A,0x00,0x00,0x00,
                                0x07,
                                0x5f,0x69,0x64,0x00,
                                0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0A,0x0B,
                                0x02,
                                0x64,0x61,0x74,0x61,0x00,
                                0x0A,0x00,0x00,0x00,
                                0x73,0x6f,0x6d,0x65,0x74,0x68,0x69,0x6e,0x67,0x00,
                                0x00));
}

#[test]
fn serialize_multiple() {
    let mut map = TreeMap::new();
    map.insert("n".to_string(), bson::Float(1f64));
    map.insert("s".to_string(), bson::String("t".to_string()));
    let serialized = bson::encode(&bson::Object(map));
    assert_eq!(serialized, vec!(0x19,0x00,0x00,0x00,
                                0x01,
                                0x6e,0x00,
                                0x00,0x00,0x00,0x00,0x00,0x00,0xf0,0x3f,
                                0x02,
                                0x73,0x00,
                                0x02,0x00,0x00,0x00,
                                0x74,0x00,
                                0x00));
}

#[test]
fn serialize_small_embedded() {
    let mut embed = TreeMap::new();
    embed.insert("n".to_string(), bson::Float(1f64));
    let mut map = TreeMap::new();
    map.insert("s".to_string(), bson::String("t".to_string()));
    map.insert("o".to_string(), bson::Object(embed));
    let serialized = bson::encode(&bson::Object(map));
    assert_eq!(serialized, vec!(0x21,0x00,0x00,0x00,
                                0x03,
                                0x6f,0x00,
                                0x10,0x00,0x00,0x00,
                                0x01,
                                0x6e,0x00,
                                0x00,0x00,0x00,0x00,0x00,0x00,0xf0,0x3f,
                                0x00,
                                0x02,
                                0x73,0x00,
                                0x02,0x00,0x00,0x00,
                                0x74,0x00,
                                0x00));
}

#[test]
fn serialize_embedded() {
    let mut embed = TreeMap::new();
    embed.insert("this".to_string(), bson::String("is embedded".to_string()));
    let mut map = TreeMap::new();
    map.insert("hello".to_string(), bson::String("world".to_string()));
    map.insert("another".to_string(), bson::String("thing".to_string()));
    map.insert("number".to_string(), bson::Float(1f64));
    map.insert("embedded".to_string(), bson::Object(embed));
    let done = bson::encode(&bson::Object(map));
    show_bson(&done);
    write_bson(&done, "test.bson");
    assert_eq!(done, vec!(0x5e,0x00,0x00,0x00,
                          0x02,
                          0x61,0x6e,0x6f,0x74,0x68,0x65,0x72,0x00,
                          0x06,0x00,0x00,0x00,
                          0x74,0x68,0x69,0x6e,0x67,0x00,
                          0x03,
                          0x65,0x6d,0x62,0x65,0x64,0x64,0x65,0x64,0x00,
                          0x1b,0x00,0x00,0x00,
                          0x02,
                          0x74,0x68,0x69,0x73,0x00,
                          0x0c,0x00,0x00,0x00,
                          0x69,0x73,0x20,0x65,0x6d,0x62,0x65,0x64,0x64,0x65,0x64,0x00,
                          0x00,
                          0x02,
                          0x68,0x65,0x6c,0x6c,0x6f,0x00,
                          0x06,0x00,0x00,0x00,
                          0x77,0x6f,0x72,0x6c,0x64,0x00,
                          0x01,
                          0x6e,0x75,0x6d,0x62,0x65,0x72,0x00,
                          0x00,0x00,0x00,0x00,0x00,0x00,0xf0,0x3f,
                          0x00));
}

/*
#[test]
fn deserialize_embedded() {
    let mut embed = BSON::new();
    embed.insert("this".to_string(), BSONString("is embedded".to_string()));
    embed.insert("negative".to_string(), BSONInt(-5));
    let mut correct = BSON::new();
    correct.insert("hello".to_string(), BSONString("world".to_string()));
    correct.insert("another".to_string(), BSONString("thing".to_string()));
    correct.insert("number".to_string(), BSONInt(10));
    correct.insert("embedded".to_string(), BSONObject(embed));
    let built = BSON::deserialize(vec!(0x68,0x00,0x00,0x00,
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
    assert_eq!(correct, built);
}

#[test]
fn serialize_id() {
    let mut bson = BSON::new();
    bson.insert("_id".to_string(), BSONId(vec!(0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b)));
    let serialized = bson.serialize();
    show_bson(&serialized);
    assert_eq!(serialized, vec!(0x16,0x00,0x00,0x00,
                          0x07,
                          0x5f,0x69,0x64,0x00,
                          0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,
                          0x00));
}

#[test]
fn deserialize_id() {
    let mut correct = BSON::new();
    correct.insert("_id".to_string(), BSONId(vec!(0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b)));
    // TODO - this works even with an incorrect length specified
    let built = BSON::deserialize(vec!(0x16,0x00,0x00,0x00,
                                       0x07,
                                       0x5f,0x69,0x64,0x00,
                                       0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0a,0x0b,
                                       0x00));
    assert_eq!(correct, built);
}
*/
