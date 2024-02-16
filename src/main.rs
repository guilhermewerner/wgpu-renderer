use anyhow::Result;
use renderer::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await;
    Ok(())
}
