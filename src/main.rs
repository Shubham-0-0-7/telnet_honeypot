use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::net::SocketAddr;
use std::time::SystemTime;

#[derive(Debug)]
enum SessionState{
    NewConnection,
    SendBanner,
    WaitUsername,
    WaitPassword,
    ShellActive,
    SessionEnd,
}

#[derive(Debug)]
struct Session{
    id:u32,
    client_addr:SocketAddr,
    state:SessionState,
    input_buff:String,
    username:Option<String>,
    password:Option<String>,

    cmd_his:Vec<String>,
    start_time:SystemTime,
}

fn handle_client(mut stream: TcpStream, session_id: u32){
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => addr,
        Err(_) => return,
    };

    let mut session = Session{
        id:session_id,
        client_addr:peer_addr,
        state:SessionState::NewConnection,
        input_buff:String::new(),
        username:None,
        password:None,
        cmd_his:Vec::new(),
        start_time:SystemTime::now(),
    };
    println!("[+] new attacker connected: session #{} from {}", session.id, session.client_addr);
    let banner = b"Welcome to Ubuntu 18.04 LTS\r\n\r\n";
    if let Err(_e) = stream.write_all(banner) {
        return;
    }
    if let Err(_e) = stream.write_all(b"ubuntu login: ") {
        return;
    }
    session.state = SessionState::WaitUsername;
    let mut buffer = [0; 512];
    loop{
        match stream.read(&mut buffer){
            Ok(0) => {
                println!("client disconnected.");
                break;
            }
            
            Ok(n) => {
                let received_text = String::from_utf8_lossy(&buffer[..n]).trim().to_string();

                match session.state {
                    SessionState::NewConnection => {
                        stream.write_all(b"Username: ").unwrap();
                        session.state = SessionState::WaitUsername;
                    }
                    SessionState::WaitUsername => {
                        session.username = Some(received_text);
                        stream.write_all(b"Password: ").unwrap();
                        session.state = SessionState::WaitPassword;
                    }
                    SessionState::WaitPassword => {
                        session.password = Some(received_text);
                        println!("[!] captured credentials - session :{} user: {:?}, pass: {:?}", session_id, session.username, session.password);
                        stream.write_all(b"root@ubuntu:~# ").unwrap();
                        session.state = SessionState::ShellActive;
                    }
                    SessionState::ShellActive => {
                        session.cmd_his.push(received_text.clone());
                        println!("[!] Command from {}: {}", session.id, received_text);

                        match received_text.as_str() {
                            "ls" => stream.write_all(b"bin  etc  home  lib  proc  root  sys  usr  var\n").unwrap(),
                            "whoami" => stream.write_all(b"root\n").unwrap(),
                            "exit" => {
                                stream.write_all(b"logout\n").unwrap();
                                break;
                            }
                            _ => stream.write_all(format!("{}: command not found\n", received_text).as_bytes()).unwrap(),
                        }
                        stream.write_all(b"root@ubuntu:~# ").unwrap();
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("error reading stream: {}", e);
                break;
            }
        }
    }
}
fn main() {
    let listener = TcpListener::bind("0.0.0.0:2323").unwrap();
    println!("listening on port 2323");
    let mut global_id_counter = 0;

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                global_id_counter += 1;
                let session_id = global_id_counter;
                thread::spawn(move || {handle_client(stream, session_id);});
            }
            Err(e) => { 
                println!("connection failed: {}", e);
            }
        }
    }
    
    // let id = 7;
    // let ca = ClientAddr{
    //     ip:String::from("127.0.0.1"),
    //     port:1337,
    // };
    // let state = String::from("SHELL_ACTIVE");
    // let inbuff = String::from("");
    // let usern = String::from("root");
    // let passwd =  String::from("123456");
    // let s = Session{
    //     id:id,
    //     clientadd:ca,
    //     state: state,
    //     input_buff:inbuff,
    //     username:usern,
    //     password:passwd,
    // };
    // println!("Hello, world!");


}
