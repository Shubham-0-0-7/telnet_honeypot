struct ClientAddr{
    ip:String,
    port:i32,
}
struct Session{
    id:i32,
    clientadd:ClientAddr,
    state:String,
    input_buff:String,
    username:String,
    password:String,
}
fn main() {
    let id = 7;
    let ca = ClientAddr{
        ip:String::from("127.0.0.1"),
        port:1337,
    };
    let state = String::from("SHELL_ACTIVE");
    let inbuff = String::from("");
    let usern = String::from("root");
    let passwd =  String::from("123456");
    let s = Session{
        id:id,
        clientadd:ca,
        state: state,
        input_buff:inbuff,
        username:usern,
        password:passwd,
    };
    

    println!("Hello, world!");
}
