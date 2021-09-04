// Get your API key at www.interzoid.com/register

// rename to main.rs in src directory for simple "Cargo run" within project directory

// Cargo.html:
//
// [dependencies]
// reqwest = { version = "0.11", features = ["json"] }
// tokio = { version = "1", features = ["full"] }

#[tokio::main]

async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://api.interzoid.com/getweatherzipcode?license=YOURAPIKEY&zip=94111").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body: {}", body);

    Ok(())
}
