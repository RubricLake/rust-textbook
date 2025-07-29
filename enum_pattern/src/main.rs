#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum UsState {
    Texas,
    New_California,
    // ...
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: i32) -> bool {
        match self {
            UsState::New_California => year >= 2020,
            UsState::Texas => year >= 1845,
        }
    }

}

fn describe_state_quarter(coin : Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    }
    else {
        Some(format!("{state:?} is relatively new."))
    }
}


fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    
    check(&home);
    check(&loopback);

    let config_max = Some(3u8);
    
    if let Some(max) = config_max {
         println!("The maximum is configured to be {max}")
    }

    let coin1 = Coin::Quarter(UsState::Texas);
    let coin2 = Coin::Quarter(UsState::New_California);

    if let Some(string) = describe_state_quarter(coin1) {
        println!("{}", string);
    }

    if let Some(string) = describe_state_quarter(coin2) {
        println!("{}", string);
    }
    
    
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
