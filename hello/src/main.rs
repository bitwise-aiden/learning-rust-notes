use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;


fn main() -> () {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) -> () {
    let buf_reader = BufReader::new(&mut stream);

    let request = buf_reader.lines().next().unwrap().unwrap();

    let (status, file_name) = match &request[..] {
        "GET / HTTP/1.1" => ("200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));

            ("200 OK", "hello.html")
        }
        _ => ("404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();

    let response = format!("HTTP/1.1 {status}\nContent-Length: {length}\n\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
