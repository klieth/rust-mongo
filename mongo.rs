
use std::io::{IoResult,IoError};
use std::io::net::tcp::TcpStream;
use bson::{BSON,BSONString};

mod bson;

struct MsgHeader {
	messageLength: uint,
	requestID: i32,
	responseTo: Option<i32>,
	opCode: i32
}

enum OP {
	OpReply = 1,
	OpQuery = 2004,
}

struct Reply {
	header: MsgHeader,
	responseFlags: i32,
	cursorID: i64,
	startingFrom: i32,
	numberReturned: i32,
	documents: Vec<BSON>
}

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
            header: MsgHeader {messageLength: 0, requestID: 1, responseTo: None, opCode: OpQuery as i32},
            flags: 0x00000000,
            fullCollectionName: "test.coll".to_string(),
            numberToSkip: 0,
            numberToReturn: 0,
            query: q.serialize(),
            returnFieldsSelector: None
        };
        q.header.messageLength = 16 + 4 + q.fullCollectionName.len() + 1 + 4 + 4 + q.query.len();
        let push_le_uint = |vec: &mut Vec<u8>, i: uint| {
            vec.push(  (i & 0x000000FF) as u8 );
            vec.push( ((i & 0x0000FF00) >> 8) as u8 );
            vec.push( ((i & 0x00FF0000) >> 16) as u8 );
            vec.push( ((i & 0xFF000000) >> 24) as u8 );
        };
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
        for u in q.query.iter() {
            msg.push(*u);
        }
        self.socket.write(msg.as_slice());
        self.socket.flush();
        let mut r = Reply {
            header: MsgHeader {
                        messageLength: try!(self.socket.read_le_i32()) as uint,
                        requestID: try!(self.socket.read_le_i32()),
                        responseTo: Some(try!(self.socket.read_le_i32())),
                        opCode: try!(self.socket.read_le_i32())
                    },
            responseFlags: try!(self.socket.read_le_i32()),
            cursorID: try!(self.socket.read_le_i64()),
            startingFrom: try!(self.socket.read_le_i32()),
            numberReturned: try!(self.socket.read_le_i32()),
            documents: Vec::new(),
        };
        let mut i = 36;
        println!("Message length: {:x}", r.header.messageLength);
        while i < r.header.messageLength {
            let mut doc = Vec::new();
            let len = try!(self.socket.read_le_i32()) as uint;
            push_le_uint(&mut doc, len);
            println!("Trying to get: {:x}", len);
            doc.push_all(try!(self.socket.read_exact(len-4)).as_slice());
            i += len;
            r.documents.push(BSON::deserialize(doc));
        }
        println!("Docs: {}", r.documents);
		Ok(DBResult { docs: Vec::new() })
	}
}

fn main() {
    let mut client = MongoClient::default_connect();
    let mut q = BSON::new();
    client.query(q);
}
