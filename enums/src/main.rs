fn main() {
    let _four = IP::V4;
    let _six = IP::V6;

    let _home = IPaddr::V4("127.0.0.1".to_string());
    let _loopback = IPaddr::V6("::1".to_string());

    let m = Message::Write("hello world".to_string());
    m.call()
}

// Enum with 2 variants
// Instances can be either IP::V4 or IP::V6 but not both
enum IP {
    V4,
    V6,
}

// Enum with associated String values
enum IPaddr {
    V4(String),
    V6(String),
}

impl Message {
    fn call(&self) {
        println!("Instance called!");
    }
}

enum Message {
    Quit,                   // Not associated data
    Move{x: i32, y: i32},   // Anonymous struct
    Write(String),          // String instance
    RGB(u8, u8, u8),        // Three u8 values
}

// Above same as:
struct QuitMessage; // unit struct
struct MoveMessage { // regular struct
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
