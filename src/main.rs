

use std::{fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread, time::Duration};

extern crate learning_rust;
use learning_rust::ThreadPool;






fn handle_stream(mut stream : TcpStream){

    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();

    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let mut content = String::new();
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line , filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n" , "index.html")
    }
    else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(200));
        ("HTTP/1.1 200 NOT FOUND\r\n\r\n" , "index.html")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n" , "404.html")
    };

        let mut file  = File::open(filename).unwrap();
    
        file.read_to_string(&mut content).unwrap();
    
        let response = format!("{}{}",status_line ,  content);
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
 
}


fn main(){
    let tcp_listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in tcp_listener.incoming().take(100){
        let stream = stream.unwrap();
        
        pool.execute(||{
            handle_stream(stream);
        })
    }
}