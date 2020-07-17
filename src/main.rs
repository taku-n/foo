use async_std::println;

use std::io::Error;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let mut app = tide::new();

    app.at("/").get(|req| async {Ok(req)});

    app.listen("127.0.0.1:8888").await?;

    Ok(())
}
