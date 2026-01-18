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
    let banner = b"Welcome to Ubuntu 18.04 LTS\r\n";
    if let Err(e) = stream.write_all(banner){
        println!("failed to send banner: {}", e);
        return;
    }
    let mut buffer = [0; 512];
    loop{
        match stream.read(&mut buffer){
            Ok(0) => {
                println!("client disconnected.");
                break;
            }
            
            Ok(n) => {
                let recieved_text = String::from_utf8_lossy(&buffer[..n]);
                print!("recieved {} bytes: {}", n, recieved_text);
                use std::io::stdout;
                let _ = stdout().flush();
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
