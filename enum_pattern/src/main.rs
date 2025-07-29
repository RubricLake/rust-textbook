use std::net::IpAddr;

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}


fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    
    check(&home);
    check(&loopback);
    
}

fn check(addr : &IpAddrKind) {
    match addr {
        IpAddrKind::V4(a, b, c ,d) => {
            println!("ipv4: {a}.{b}.{c}.{d}");
        }
        IpAddrKind::V6(x) => {
            println!("ipv6: {x}");
        }
    }
}
