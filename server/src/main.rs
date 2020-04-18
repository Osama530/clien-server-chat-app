use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_micros(100));
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking"); //read bytes in nonblocking mode

    let mut clients = vec![]; //empty vector to hold all the clients
    let (tx, rx) = mpsc::channel::<String>(); //instantiatind a channel for type string

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            //Accept a new incoming connection from listener
            println!("Clinen {} connected...", addr); //print the address of a client connected

            let tx = tx.clone(); //the the tx
            clients.push(socket.try_clone().expect("failed to clone the client")); //cloning so we cant use it in our thread and puch it into vector 'clients'

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE]; //crating vector to hold msg

                match socket.read_exact(&mut buff) {
                    //read msg from specific client
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>(); // converting buff into itrating value then dropping white spaces and then collecting into vector
                        let msg = String::from_utf8(msg).expect("Invalid utf msg"); //converting the slices of string into actual string

                        println!("{} from address: {}", msg, addr);
                        tx.send(msg).expect("failed to send msg to rx"); //transmitimg msg
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (), // if non-bloking would block some how
                    Err(_) => {
                        println!("Closing connection with {}", addr);
                        break;
                    } // for other kinds of errors
                }
                sleep(); //goto sleep for 100ms
            });
        }
        //if server recives a msg
        if let Ok(msg) = rx.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    //itrating over clients and filter
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }
    }
}
