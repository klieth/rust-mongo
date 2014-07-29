
use std::io::{IoResult,IoError};
use std::io::net::tcp::TcpStream;
use bson::{BSON,BSONString};

mod bson;

struct MsgHeader {
	messageLength: uint,
	requestID: i32,
	responseTo: Option<i32>,
	opCode: OP
}

enum OP {
	OpReply = 1,
	OpQuery = 2004,
}

/*
struct Reply {
	header: MsgHeader,
	responseFlags: i32,
	cursorID: i64,
	startingFrom: i32,
	numberReturned: i32,
	documents: Vec<u8>
}
*/

struct Query {
	header: MsgHeader,
	flags: i32,
	fullCollectionName: String,
	numberToSkip: i32,
	numberToReturn: i32,
	query: Vec<u8>,
	returnFieldsSelector: Option<Vec<u8>>,
}

struct DBResult {
	docs: Vec<BSON>
}

struct MongoClient {
	socket: TcpStream,
}
impl MongoClient {
	pub fn default_connect() -> MongoClient {
		MongoClient { socket: TcpStream::connect("127.0.0.1",27017).unwrap() }
	}
	pub fn query(&mut self, q: BSON) -> IoResult<DBResult> {
		println!("Doing query");
        let mut q = Query {
            header: MsgHeader {messageLength: 0, requestID: 1, responseTo: None, opCode: OpQuery},
            flags: 0x00000000,
            fullCollectionName: "test.coll".to_string(),
            numberToSkip: 0,
            numberToReturn: 0,
            query: q.serialize(),
            returnFieldsSelector: None
        };
        q.header.messageLength = 16 + 4 + q.fullCollectionName.len() + 1 + 4 + 4 + 5;
        println!("Message length: {:x}", q.header.messageLength);
        let push_le_i32 = |vec: &mut Vec<u8>, i: i32| {
            vec.push(  (i & 0x000000FF) as u8 );
            vec.push( ((i & 0x0000FF00) >> 8) as u8 );
            vec.push( ((i & 0x00FF0000) >> 16) as u8 );
            vec.push( ((i & 0xFF000000) >> 24) as u8 );
        };
        let mut msg: Vec<u8> = Vec::new();
        push_le_i32(&mut msg, q.header.messageLength as i32);
        push_le_i32(&mut msg, q.header.requestID);
        push_le_i32(&mut msg, 0);
        push_le_i32(&mut msg, q.header.opCode as i32);
        push_le_i32(&mut msg, 0);
        for c in q.fullCollectionName.as_slice().chars() {
            msg.push(c as u8);
        }
        msg.push(0x00);
        push_le_i32(&mut msg, 0);
        push_le_i32(&mut msg, 0);
        msg.push(0x05);
        msg.push(0x00);
        msg.push(0x00);
        msg.push(0x00);
        msg.push(0x00);
        self.socket.write(msg.as_slice());
        self.socket.flush();
        // Send the header
        /*
        println!("{:x}", q.header.requestID);
        try!(self.socket.write_le_i32(q.header.requestID));
        println!("0");
        try!(self.socket.write_le_i32(0));        // TODO - fix this. hard coded the None value for now
        println!("{:x}",q.header.opCode as i32);
        try!(self.socket.write_le_i32(q.header.opCode as i32));
        // Send the body
        println!("{:x}",q.flags);
        try!(self.socket.write_le_i32(q.flags));
        for c in q.fullCollectionName.as_slice().chars() {
            println!("c: {}", c);
            try!(self.socket.write_char(c));
        }
        println!("c: 0");
        try!(self.socket.write_u8(0x00));
        println!("{:x}", q.numberToSkip);
        try!(self.socket.write_le_i32(q.numberToSkip));
        println!("{:x}", q.numberToReturn);
        try!(self.socket.write_le_i32(q.numberToReturn));
        try!(self.socket.write_le_i32(0x05));
        try!(self.socket.write_le_i32(0x00));
        try!(self.socket.write_le_i32(0x00));
        try!(self.socket.write_le_i32(0x00));
        try!(self.socket.write_le_i32(0x00));
        self.socket.flush();
        */
        let size = try!(self.socket.read_le_uint());
        println!("Got size: {}", size);
        for i in range(0u, size-4) {
            println!("{}",try!(self.socket.read_u8()) as char);
        }
        /*
        match self.socket.read_to_end() {
            Ok(res) => {
                println!("Done: {}", res);
            },
            Err(e) => {
                fail!("Reading failed");
            }
        }
        */
		Ok(DBResult { docs: Vec::new() })
	}
}

fn main() {
    let mut client = MongoClient::default_connect();
    let mut q = BSON::new();
    q.insert("test".to_string(), BSONString("item".to_string()));
    client.query(q);
}
