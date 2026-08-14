//! Запуск: KODIK_API_TOKEN=... cargo run --example search

use kodik_api::{ContentType, Filters, KodikClient, SearchQuery, TranslationType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("KODIK_API_TOKEN")
        .map_err(|_| "set KODIK_API_TOKEN before running this example")?;
    let client = KodikClient::new(token)?;

    let query = SearchQuery {
        title: Some("Avatar".into()),
        full_match: false,
        limit: Some(10),
        filters: Filters::default().with_types([ContentType::ForeignMovie]),
        prioritize_translation_type: Some(TranslationType::Voice),
        with_material_data: true,
        ..Default::default()
    };

    let response = client.search(query).await?;
    println!("{} result(s), API time: {}", response.total, response.time);

    for material in response.results {
        let rating = material
            .material_data
            .as_ref()
            .and_then(|data| data.kinopoisk_rating)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "n/a".into());
        println!(
            "{} | {} | {} | Kinopoisk rating: {rating}",
            material.id,
            material
                .year
                .map(|year| year.to_string())
                .unwrap_or_else(|| "n/a".into()),
            material.title,
        );
    }

    Ok(())
}
