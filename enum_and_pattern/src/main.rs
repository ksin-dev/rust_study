
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {
    let v4 = IpAddr::V4(String::from("192.168.2.1"));
    let v6 = IpAddr::V6(String::from("::1"));


    println!("v4: {:?} , v6: {:?}", v4,v6);
}
