use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // 1️⃣ LOGIN
    hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    )
    .await?
    .print()
    .await?;

    // 2️⃣ CREATE TICKET ✅
    hc.do_post(
        "/api/tickets",
        json!({
            "title": "My First Ticket"
        }),
    )
    .await?
    .print()
    .await?;

    // 3️⃣ GET ALL TICKETS
    hc.do_get("/api/tickets")
        .await?
        .print()
        .await?;

    Ok(())
}