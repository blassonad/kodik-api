# Live Kodik API Tests

`tests/live_api.rs` проверяет настоящие read-only запросы к `https://kodik-api.com`. В отличие от обычных unit- и integration-тестов, этот набор требует сети, действующего токена и расходует лимит Kodik API. Поэтому оба теста помечены `#[ignore]` и никогда не запускаются обычной командой `cargo test`.

## Что именно проверяется

| Тест | Ресурсы и инварианты |
|---|---|
| `live_catalog_resources_decode_successfully` | `/years`, `/genres`, `/countries`, `/qualities/v2` и `/translations/v2`; успешный HTTP/JSON-разбор, непустой `time` и базовая корректность строк справочника. |
| `live_search_list_and_post_requests_decode_successfully` | `/search` через GET и POST, `/list`; десериализация реального ответа, лимит `1`, согласованность `total` и количества результатов. |

Тесты выполняются последовательно (`--test-threads=1`), используют только GET/POST-запросы к read-only API и не печатают токен. Они не проверяют наличие конкретного фильма или перевода, потому что содержимое базы Kodik меняется.

## Локальный запуск

Передайте токен только в переменной окружения текущего процесса:

```bash
KODIK_API_TOKEN='ваш_токен' \
  cargo test --test live_api -- --ignored --test-threads=1
```

Не добавляйте токен в `.env`, `Cargo.toml`, исходный код или командную историю, которая сохраняется в общем окружении.

## GitHub Actions

Workflow [`.github/workflows/live-api.yml`](../.github/workflows/live-api.yml) запускается после **каждого push в любую ветку** и вручную. Он берёт значение исключительно из `KODIK_API_TOKEN` защищённого GitHub Environment **`kodik-api`**.

Создать secret может maintainer с соответствующим GitHub-доступом:

```bash
printf '%s' 'ваш_токен' | \
  gh secret set KODIK_API_TOKEN --repo blassonad/kodik-api --env kodik-api
```

GitHub не возвращает значение secret после сохранения и маскирует его в логах Actions. Workflow завершится ошибкой, а не выполнит фиктивную проверку, если secret отсутствует.
