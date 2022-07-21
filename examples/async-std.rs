#[async_std::main]
async fn main() {
    let _mortem = mortem::hard();

    async_std::task::spawn(async {
        println!("Hello!");
    });
}
