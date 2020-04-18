use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking"); //read bytes in nonblocking mode

    let (tx, rx) = mpsc::channel::<String>(); //instantiatind a channel for type string


    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE]; //crating vector to hold msg

        match client.read_exact(&mut buff) {
            //read msg from specific client
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(); // converting buff into itrating value then dropping white spaces and then collecting into vector
                println!("message recived: {:?} ", msg );
            },
            //error handling
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (), // if non-bloking would block some how
            Err(_) => {
                println!("connection with server was severed");
                break;
            } // for other kinds of errors
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent: {:?}", msg);    },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("write a message :");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if msg == "quit" || tx.send(msg).is_err() {break}
    }
    println!("bye bye");
}
