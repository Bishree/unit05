fn main() {
    println!("Hello, world!");
    let mut user1 = User{
        username: String::from("ahmed1"),
        email: "ahmed@gmail.com".to_string(),
        sign_in_count: 1,
        active: true,
    };
    user1.email = ("ahmed2").to_string();
    let user2= User{
        username: "moh".to_string(),
        email: "moh@gmail.com".to_string(),
        ..user1
    };
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user (email: String, username:String) -> User{
    User{
        username,
        email,
        sign_in_count: 1,
        active: true,
    }

}