#[tokio::main]
async fn main() {
    let _mortem = mortem::hard();

    tokio::spawn(async {
        println!("Hello!");
    });
}
