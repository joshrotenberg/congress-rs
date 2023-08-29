use congress::{bill_type::BillType, prelude::*, sort::Sort, ClientBuilder, Result};

#[tokio::test]
async fn bills() -> Result<()> {
    let api_key = std::env::var("CONGRESS_API_KEY").unwrap();
    let client = ClientBuilder::new(api_key).build()?;
    let bills = client
        .bills()
        .congress(117)
        .bill_type(BillType::House)
        .limit(250)
        .sort(Sort::UpdateDateAscending)
        .to_date(chrono::offset::Utc::now())
        .offset(2)
        .send()
        .await?;
    for _b in bills.into_iter() {}

    Ok(())
}
