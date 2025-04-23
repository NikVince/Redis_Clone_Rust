use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Binding listener to address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        // each new inbound socket get's assigned to a new task (aka working thread)
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // creating hashmap
    let mut db = HashMap::new();

    // connection handling parsed frames by socket
    let mut connection = Connection::new(socket);

    // reading command from frame
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {

            }
            Get(cmd) => {

            } else {
                Frame::Null
            }
    }
    cmd => panic!("unimplemented {:?}", cmd),
};
