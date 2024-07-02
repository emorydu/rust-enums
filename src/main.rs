#[derive(Debug)]
enum IpAddrKind {
    V4, 
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind: {:?}", ip_kind);
}

#[derive(Debug)]
enum IpAddrWithValue {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,   // struct QuitMessage;
    Move { x: i32, y: i32 },    // struct QuitMove { x: i32, y: i32 }
    Write(String),  // struct QuitWrite(String);
    ChangeColor(i32, i32, i32), // struct QuitChangeColor(i32, i32, i32);
}

impl Message {
    fn call(&self) {
        // something
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?}", four);
    println!("six: {:?}", six);

    route(four);
    route(six);

    let home = IpAddr { 
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home: {:?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback: {:?}", loopback);


    let home = IpAddrWithValue::V4(127, 0, 0, 1);
    println!("home: {:?}", home);
    let loopback = IpAddrWithValue::V6(String::from("::1"));
    println!("loopback: {:?}", loopback);


    let m = Message::Write(String::from("hello, world"));
    println!("m: {:?}", m);
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;

}
