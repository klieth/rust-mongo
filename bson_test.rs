
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

fn main() {
    let mut embed = BSON::new();
    embed.insert("this".to_string(), BSONString("is embedded".to_string()));
    embed.insert("negative".to_string(), BSONInt(-5));
    let mut bson = BSON::new();
    bson.insert("hello".to_string(), BSONString("world".to_string()));
    bson.insert("another".to_string(), BSONString("thing".to_string()));
    bson.insert("number".to_string(), BSONInt(10));
    bson.insert("embedded".to_string(), BSONObject(embed));
    for v in bson.get_iter() {
        println!("{}",v);
    }
    let done = bson.serialize();
    show_bson(&done);
    write_bson(&done, "test2.bson");
}
