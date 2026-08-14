//! Реальные integration-тесты Kodik API.
//!
//! Они намеренно помечены `#[ignore]`: для запуска требуются сеть и токен.
//! Запуск вручную:
//! `KODIK_API_TOKEN=... cargo test --test live_api -- --ignored --test-threads=1`.

use kodik_api::{
    ContentType, CountriesQuery, Filters, GenresQuery, KodikClient, ListQuery, QualitiesQuery,
    RequestMethod, SearchQuery, TranslationsQuery, YearsQuery,
};

fn live_client() -> KodikClient {
    let token = std::env::var("KODIK_API_TOKEN")
        .expect("KODIK_API_TOKEN must be set to run live Kodik API tests");
    assert!(
        !token.trim().is_empty(),
        "KODIK_API_TOKEN must not be an empty string"
    );
    KodikClient::new(token).expect("a non-empty Kodik API token is a valid client configuration")
}

fn assert_api_time(value: &str, endpoint: &str) {
    assert!(
        !value.trim().is_empty(),
        "{endpoint} returned an empty API processing time"
    );
}

/// Проверяет, что все пять справочных read-only ресурсов принимают токен,
/// возвращают успешный JSON и десериализуются в публичные модели.
#[tokio::test]
#[ignore = "requires KODIK_API_TOKEN and access to kodik-api.com"]
async fn live_catalog_resources_decode_successfully() {
    let client = live_client();

    let years = client.years(YearsQuery::default()).await.unwrap();
    assert_api_time(&years.time, "/years");
    assert!(years.results.iter().all(|item| item.count > 0));

    let genres = client.genres(GenresQuery::default()).await.unwrap();
    assert_api_time(&genres.time, "/genres");
    assert!(genres
        .results
        .iter()
        .all(|item| !item.title.trim().is_empty()));

    let countries = client.countries(CountriesQuery::default()).await.unwrap();
    assert_api_time(&countries.time, "/countries");
    assert!(countries
        .results
        .iter()
        .all(|item| !item.title.trim().is_empty()));

    let qualities = client.qualities(QualitiesQuery::default()).await.unwrap();
    assert_api_time(&qualities.time, "/qualities/v2");
    assert!(qualities
        .results
        .iter()
        .all(|item| !item.title.trim().is_empty()));

    let translations = client
        .translations(TranslationsQuery::default())
        .await
        .unwrap();
    assert_api_time(&translations.time, "/translations/v2");
    assert!(translations
        .results
        .iter()
        .all(|item| !item.title.trim().is_empty()));
}

/// Проверяет поисковые и list-ресурсы, включая POST-путь передачи параметров.
/// Запросы ограничены одной записью, не изменяют данные и не требуют player URL.
#[tokio::test]
#[ignore = "requires KODIK_API_TOKEN and access to kodik-api.com"]
async fn live_search_list_and_post_requests_decode_successfully() {
    let client = live_client();
    let search = SearchQuery {
        title: Some("Avatar".into()),
        limit: Some(1),
        ..Default::default()
    };

    let get_result = client.search(search.clone()).await.unwrap();
    assert_api_time(&get_result.time, "/search GET");
    assert!(get_result.results.len() <= 1);
    assert!(get_result.total >= get_result.results.len() as u64);

    let post_result = client
        .search_with_method(search, RequestMethod::Post)
        .await
        .unwrap();
    assert_api_time(&post_result.time, "/search POST");
    assert!(post_result.results.len() <= 1);
    assert!(post_result.total >= post_result.results.len() as u64);

    let list_result = client
        .list(ListQuery {
            filters: Filters::default().with_types([ContentType::ForeignMovie]),
            limit: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    assert_api_time(&list_result.time, "/list");
    assert!(list_result.results.len() <= 1);
    assert!(list_result.total >= list_result.results.len() as u64);
}
