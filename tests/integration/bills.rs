use congress::prelude::*;
use congress::ClientBuilder;

#[tokio::test]
async fn client() {
    let api_key = std::env::var("CONGRESS_API_KEY").unwrap();
    let client = ClientBuilder::new(api_key).build().unwrap();
    let bills = client
        .bills()
        // .congress(117)
        // .bill_type(doo::bill_type::BillType::House)
        .limit(250)
        // .sort(Sort::UpdateDateAscending)
        // .to_date(chrono::offset::Utc::now())
        // .offset(2)
        .send()
        .await
        .unwrap();
    dbg!(bills.into_iter().count());
    // for b in bills.into_iter() {
    //     dbg!(b);
    // }
}
