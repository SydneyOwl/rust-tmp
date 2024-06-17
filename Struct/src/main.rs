fn main() {
    let mut owl = User{
        active: false,
        username: String::from("Owl"),
        email: String::from("Owl"),
        signin_count: 0,
    };
    //总体上说我们在创建 user2 后不能就再使用 user1 了，因为 user1 的 username 字段中的 String 被移到 user2 中。
    // 如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。
    let pig = User{
        active:true,
        ..owl
    };
    owl.signin_count = 1;
    // println!("{:?}",owl)
}
#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    signin_count: u64
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        signin_count: 1,
    }
}