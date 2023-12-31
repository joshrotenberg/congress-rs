use congress::{bill::actions::ActionsResponse, prelude::*, ClientBuilder, Result};

#[tokio::test]
async fn bill() -> Result<()> {
    let api_key = std::env::var("CONGRESS_API_KEY").unwrap();
    let client = ClientBuilder::new(api_key).build()?;
    let bills = client.bills().limit(2).send().await?;

    for b in bills.into_iter() {
        let _bill = client
            .bill(b.congress, b.bill_type, b.number.parse().unwrap())
            .send()
            .await?;
    }

    Ok(())
}

#[tokio::test]
async fn actions() -> Result<()> {
    let api_key = std::env::var("CONGRESS_API_KEY").unwrap();
    let client = ClientBuilder::new(api_key).build()?;
    let bills = client.bills().limit(2).send().await?;

    for b in bills.into_iter() {
        let _actions = client
            .bill(b.congress, b.bill_type, b.number.parse().unwrap())
            .actions()
            .limit(2)
            .send()
            .await?;
        // for a: &Action in actions().into_iter() {
            // dbg!(a);
        // }
        let next: ActionsResponse = client.next(&_actions).await?.unwrap();
        dbg!(next);
        // dbg!(cl & _actions);
    }

    Ok(())
}

#[tokio::test]
async fn amendments() -> Result<()> {
    let api_key = std::env::var("CONGRESS_API_KEY").unwrap();
    let client = ClientBuilder::new(api_key).build()?;
    let bills = client.bills().limit(2).send().await?;

    for b in bills.into_iter() {
        let _actions = client
            .bill(b.congress, b.bill_type, b.number.parse().unwrap())
            .amendments()
            .send()
            .await?;
        dbg!(&_actions);
    }

    Ok(())
}
