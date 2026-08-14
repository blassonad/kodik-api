//! Запуск: KODIK_API_TOKEN=... cargo run --example list_all

use kodik_api::{ContentType, Filters, KodikClient, ListQuery, ListResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("KODIK_API_TOKEN")
        .map_err(|_| "set KODIK_API_TOKEN before running this example")?;
    let client = KodikClient::new(token)?;

    let mut query = ListQuery {
        limit: Some(100),
        filters: Filters::default().with_types([ContentType::AnimeSerial]),
        ..Default::default()
    };

    let mut page_number = 1_u32;
    loop {
        let page: ListResponse = client.list(query.clone()).await?;
        println!("page {page_number}: {} material(s)", page.results.len());
        for material in &page.results {
            println!("  {} — {}", material.id, material.title);
        }

        let Some(next_page_url) = page.next_page else {
            break;
        };

        // Не выполняем URL напрямую: он может содержать query-параметр token.
        query.next = Some(KodikClient::next_cursor(&next_page_url)?);
        page_number += 1;
    }

    Ok(())
}
