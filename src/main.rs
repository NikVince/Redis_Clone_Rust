mod connection;
mod db;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Debug message for testing auto-merge
    println!("Redis Clone Server starting up - testing automated Jenkins build/merge from test to main branch feature");

    // Binding listener to address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db = db::new();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        // Cloning handle to hashmap
        let db = db.clone();

        println!("Accepted Connection");

        tokio::spawn(async move {
            connection::process(socket, db).await;
        });
    }
}
