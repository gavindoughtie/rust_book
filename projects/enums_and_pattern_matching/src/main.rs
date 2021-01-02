#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    route(&home);
    route(&loopback);
}

fn route(addr: &IpAddr) {
    println!("v4 address: {:#?}", addr);
}

fn _route(addr: &IpAddr) {
    println!("addr: {:#?}", addr);
}
