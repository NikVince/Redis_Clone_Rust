use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Debug message for testing auto-merge
    println!("Redis Clone Server starting up - testing automated Jenkins build/merge from test to main branch feature");

    // Binding listener to address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        // cloning handle to hashmap
        let db = db.clone();

        println!("Accepted Connection");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // creating hashmap
    let mut db = HashMap::new();

    // connection handling parsed frames by socket
    let mut connection = Connection::new(socket);

    // reading command from frame
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let _response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // value stored as 'Vec<u8>'
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("Ok".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // response to client
        connection.write_frame(&_response).await.unwrap();
    }
}
