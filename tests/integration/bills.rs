use congress::bills::BillsResponse;
use congress::{error::Result, Client, ClientBuilder};

fn test_client() -> Result<Client> {
    let api_key = std::env::var("CONGRESS_API_KEY").unwrap_or_else(|_| {
        panic!(
            "No value found for {} in environment.",
            "CONGRESS_TEST_API_KEY"
        )
    });
    ClientBuilder::default().api_key(api_key).build() //.unwrap();
}

#[tokio::test]
async fn bills() {
    let resp = test_client().unwrap().bills().bills().await.unwrap();
}

#[tokio::test]
async fn bills_congress() {
    let resp = test_client()
        .unwrap()
        .bills()
        .congress(117)
        .bills()
        .await
        .unwrap();
    // dbg!(resp);
}

#[tokio::test]
async fn bills_congress_bill_type() {
    let resp = test_client()
        .unwrap()
        .bills()
        .congress(117)
        .bill_type(congress::bill_type::BillType::House)
        .bills()
        .await
        .unwrap();
    // dbg!(resp);
}

#[tokio::test]
async fn bills_no_congress_bill_type() {
    let r = test_client()
        .unwrap()
        .bills()
        .bill_type(congress::bill_type::BillType::House)
        .bills()
        .await;
    dbg!(r.unwrap_err().to_string());
    assert!(test_client()
        .unwrap()
        .bills()
        .bill_type(congress::bill_type::BillType::House)
        .bills()
        .await
        .is_err());
}

#[tokio::test]
async fn bills_congress_bill_type_previous_next_iterate() -> Result<()> {
    let client = test_client().unwrap();
    let resp = client.bills().congress(118).bills().await.unwrap();

    let previous = client.previous(&resp).await?;
    assert!(previous.is_none());

    let next = client.next(&resp).await?;
    assert!(next.is_some());

    for b in resp {
        assert_eq!(b.congress, 118);
    }

    Ok(())
}

#[tokio::test]
async fn bills_congress_bill_type_sort() {
    let client = test_client().unwrap();
    let resp = client
        .bills()
        .congress(118)
        .sort(congress::parameters::Sort::UpdateDateDescending)
        .bills()
        .await
        .unwrap();

    // dbg!(&resp);
    let next = client.next(&resp).await;
    // dbg!(next);
}
