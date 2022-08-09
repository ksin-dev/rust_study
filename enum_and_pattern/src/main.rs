
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {

    // enum
    let v4 = IpAddr::V4(String::from("192.168.2.1"));
    let v6 = IpAddr::V6(String::from("::1"));


    println!("v4: {:?} , v6: {:?}", v4,v6);


    //match
    match v4 {
        IpAddr::V4(_) => println!("true"),
        IpAddr::V6(_) => println!("false"),
    }

    let i = Some(1);

    let new_i = if let Some(1) = i {
        1
    } else {
        0
    };

    println!("{}",new_i);

    
}
