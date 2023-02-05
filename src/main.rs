use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let body = reqwest::get("https://lukesmith.xyz").await?.text().await?;
    println!("Result: {body}");
    Ok(())
}
