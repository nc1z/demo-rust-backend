# Rust REST backend built with Rocket

Rust with Rocket framework

## Testing

Refer to `src/tests/mod.rs` for examples

## Endpoints

### `/date/now`

Returns a Date object converted to JSON.

##### Example Request

```bash
curl -X GET http://127.0.0.1:8000/date/now
```

##### Example Response

```json
{
    "day": 26,
    "month": 12,
    "year": 2023
}
```

### `/date/add-month`

Adds a month to the submitted date.

##### Example Request

```bash
curl -X POST -H "Content-Type: application/json" -d '{"day": 3, "month": 6, "year": 2022}' http://127.0.0.1:8000/date/add-month
```

##### Example Response

```json
{
    "day": 3,
    "month": 7,
    "year": 2022
}
```
