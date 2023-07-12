use tokio::{
    io::{self,AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    //sync::broadcast,
};

#[tokio::main]
async fn main() -> io::Result<()>{
    let listener = TcpListener::bind("localhost:8080").await?;
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("Accepting incoming connection from {:?}", addr);

        tokio::spawn(async move {
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello World!\r\n";
            socket.write_all(response.as_bytes()).await
        });
    }
}
