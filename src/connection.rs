// use bytes::Bytes;
// use mini_redis::{Command, Connection, Frame};
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

use crate::db::Db;

pub async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection handling parsed frames by socket
    let mut connection = Connection::new(socket);

    // Reading command from frame
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Response to client
        connection.write_frame(&response).await.unwrap();
    }
}
