use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let s: Vec<&str> = request_line.split(" ").collect();
    let title = String::from(&s[1][1..]);
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    thread::spawn(|| {
        send_request_to_google_script_api(title);
    });
}

fn send_request_to_google_script_api(title: String) {
    let base_url = env::var("APP_SCRIPT_BASE_URL").unwrap();
    let url = format!("{}?title={}", base_url, title);
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let resp = client
        .post(url)
        .header("Content-Length", "0")
        .send()
        .unwrap();
    if resp.status() != 200 {
        println!("{:?}", resp.text().unwrap())
    }
}
