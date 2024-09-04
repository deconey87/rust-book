use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // ストリームをバッファでラップすることで、読み取り操作が効率的になる
    // また、lines() が実装されているので、行単位で読み取れるので便利
    // BufReader::new()に可変参照を渡すことで、ストリームの状態を変更しながら読み取ることができる（現状では読み取りしかしていないので、&streamを渡しても動作に違いはない）
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     // BufReaderはBufReadトレイトを実装しており、それがlines()を提供する。行ごとに取得できるイテレータを返す
    //     .lines()
    //     .map(|result| result.unwrap())
    //     // 空行が出てくるまでとる
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    // write_all() は &[u8]（バイトデータのスライスの参照）を引数にとる
    stream.write_all(response.as_bytes()).unwrap();
}
