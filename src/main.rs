use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, error::Error,
    //result,
};

fn handler_conexion(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents:String = fs::read_to_string(filename).unwrap();
    let length:usize = contents.len();
 
    let response:String =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    // Creamos una variable la cual alamacena la escucha
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Creamos un bucle que itere en la conexion y imprima si se realizo la conexion o no
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler_conexion(stream);
    }
}
