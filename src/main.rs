use::std::net::SocketAddr;
use::std::time::SystemTime;

#[derive(Debug)]
enum SessionState{
    NewConnection,
    SendBanner,
    WaitUsername,
    WaitPassword,
    ShellActive,
    SessionEnd,
}

struct Session{
    id:u32,
    client_addr:SocketAddr,
    state:SessionState,
    input_buff:String,
    username:Option<String>,
    password:Option<String>,

    command_his:Vec<String>,
    start_time:SystemTime,
}
fn main() {
    let dummy_addr:SocketAddr = "127.0.0.1:1337".parse().unwrap();
    let s = Session{
        id:1,
        client_addr: dummy_addr,
        state:SessionState::NewConnection,
        input_buff: String::new(),
        username:None,
        password:None,
        command_his: Vec::new(),
        start_time: SystemTime::now(),
    };

    println!("Session {} started from {}", s.id, s.client_addr);
    println!("Current State: {:?}", s.state);
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
