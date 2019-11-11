use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("crash and burn");
    //let v = vec![1, 2, 3];
    //println!("{}", v[99]);
    // Env: RUST_BACKTRACE=1

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error: {:?}", error)
            },
            other_error => panic!("Error: {:?}", error)
            
        }
    };
    
    //use std::net::IpAddr;
    //let home: IpAddr = "127.0.0.1".parse().unwrap();
}
