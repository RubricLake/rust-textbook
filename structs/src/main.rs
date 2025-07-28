struct User {
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64,
}

struct vec3(i32, i32, i32);

struct test_struct;

fn main() {
    let my_user = build_user(String::from("John"), String::from("John@gmail.com"));
    println!("Username: {}\nEmail: {}", my_user.username, my_user.email);

    let v = vec3(1, 2, 3);
    let vec3(x, y, z) = v;
    println!("X = {x}\nY = {y}\nZ = {z}");

    let subject = test_struct;
}

fn build_user(username : String, email : String) -> User {
    User {
        username,
        active: false,
        email,
        sign_in_count : 0
    }
}