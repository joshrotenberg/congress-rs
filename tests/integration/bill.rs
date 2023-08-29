use congress::{prelude::*, ClientBuilder, Result};

#[tokio::test]
async fn bill() -> Result<()> {
    let api_key = std::env::var("CONGRESS_API_KEY").unwrap();
    let client = ClientBuilder::new(api_key).build()?;
    let bills = client.bills().limit(20).send().await?;

    for b in bills.into_iter() {
        let _bill = client
            .bill(b.congress, b.bill_type, b.number.parse().unwrap())
            .send()
            .await?;
    }

    Ok(())
}
