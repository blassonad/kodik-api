use std::num::NonZeroU32;

use kodik_api::{
    BlockedSeason, BlockedSeasons, ContentType, Episode, Filters, KodikClient, ListQuery,
    RequestMethod, SearchQuery, TranslationType,
};

#[test]
fn client_builder_preserves_public_configuration() {
    let client = KodikClient::builder("test-token")
        .requests_per_second(NonZeroU32::new(5).unwrap())
        .base_url("https://api.example.test/")
        .build()
        .expect("a valid configuration builds");

    assert_eq!(client.base_url(), "https://api.example.test");
}

#[test]
fn list_query_encodes_all_detail_flags_and_filters() {
    let query = ListQuery {
        filters: Filters {
            translation_type: Some(TranslationType::Subtitles),
            camrip: Some(false),
            lgbt: Some(false),
            ..Filters::default().with_types([ContentType::AnimeSerial])
        },
        limit: Some(100),
        with_seasons: true,
        with_episodes_data: true,
        with_page_links: true,
        not_blocked_in: vec!["RU".into(), "UA".into()],
        not_blocked_for_me: true,
        with_material_data: true,
        next: Some("cursor==".into()),
        ..Default::default()
    };

    let encoded = query.query_string().expect("valid list query");
    for expected in [
        "types=anime-serial",
        "translation_type=subtitles",
        "camrip=false",
        "lgbt=false",
        "limit=100",
        "with_seasons=true",
        "with_episodes_data=true",
        "with_page_links=true",
        "not_blocked_in=RU%2CUA",
        "not_blocked_for_me=true",
        "with_material_data=true",
        "next=cursor%3D%3D",
    ] {
        assert!(
            encoded.contains(expected),
            "missing {expected} in {encoded}"
        );
    }
}

#[test]
fn search_query_accepts_an_external_identifier_as_the_only_criterion() {
    let query = SearchQuery {
        imdb_ids: vec!["tt0499549".into(), "tt4154796".into()],
        prioritize_translations: vec!["704".into(), "voice".into()],
        unprioritize_translations: vec!["subtitles".into()],
        ..Default::default()
    };

    let encoded = query
        .query_string()
        .expect("external ID is a search criterion");
    assert!(encoded.contains("imdb_id=tt0499549%2Ctt4154796"));
    assert!(encoded.contains("prioritize_translations=704%2Cvoice"));
    assert!(encoded.contains("unprioritize_translations=subtitles"));
}

#[test]
fn material_deserializes_detailed_episode_and_season_block_shapes() {
    let mut json = br#"
        {
          "id":"serial-1",
          "type":"foreign-serial",
          "link":"https://example.test/serial",
          "title":"Example",
          "translation":{"id":1,"title":"Voice","type":"voice"},
          "seasons":{"1":{"link":"https://example.test/season/1","episodes":{"1":{"link":"https://example.test/episode/1","title":"Pilot","screenshots":["https://example.test/shot.jpg"]}}}},
          "blocked_seasons":{"2":"all","3":["1","4"]}
        }
    "#
    .to_vec();

    let material: kodik_api::Material = simd_json::serde::from_slice(&mut json).unwrap();
    let episode = material
        .seasons
        .as_ref()
        .and_then(|seasons| seasons.get("1"))
        .and_then(|season| season.episodes.as_ref())
        .and_then(|episodes| episodes.get("1"))
        .expect("episode is present");
    assert!(matches!(episode, Episode::Data(data) if data.title.as_deref() == Some("Pilot")));

    let blocks = material.blocked_seasons.expect("blocks are present");
    assert!(matches!(
        blocks,
        BlockedSeasons::BySeason(ref by_season)
            if matches!(by_season.get("2"), Some(BlockedSeason::All(value)) if value == "all")
                && matches!(by_season.get("3"), Some(BlockedSeason::Episodes(values)) if values == &["1", "4"])
    ));
}

#[test]
fn request_method_default_is_get() {
    assert_eq!(RequestMethod::default(), RequestMethod::Get);
}
