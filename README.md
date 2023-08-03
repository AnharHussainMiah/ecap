```
███████  ██████  █████  ██████
██      ██      ██   ██ ██   ██
█████   ██      ███████ ██████
██      ██      ██   ██ ██
███████  ██████ ██   ██ ██
```

> super simple email capture service

![alt Daemon](https://img.shields.io/badge/Type-Web_service-red.svg)
![alt Rust](https://img.shields.io/badge/Language-Rust-orange.svg)
![alt Binary](https://img.shields.io/badge/Architecture-binary-green.svg)
![alt Failed](https://img.shields.io/badge/Failed-👎_0-red.svg)
![alt Passed](https://img.shields.io/badge/Passed-👍_0-green.svg)
![alt Version](https://img.shields.io/badge/version-0.1.0_ALPHA-blue.svg)

# eCap

eCap is a super simple email capture web service. It is designed to validate an email and then simply insert into a `postgresql` database.

It's main intention is those looking to capture emails for newsletter subcribers and that are also using some kind of `static` site that does not have a backed API.

## Features

- Full email validation
- API authentication via `x-api-key` header
- Email de-duplication

## Building

Make sure you have `sqlx-cli` installed so that you can manually run migrations to make sure that the database schema is in place. Otherwise compilations will fail :|

```
$ cargo install sqlx-cli
$ sqlx migrate run
$ cargo build
```

## Environment Variables

| Variable     | Description             | Example                              |
| ------------ | ----------------------- | ------------------------------------ |
| DATABASE_URL | Pg connection string    | postgres://dev:pass@localhost/ecap   |
| API_KEY      | API Key for the service | 1f726148-499f-4e5b-b164-1e76ff223af1 |

## Calling the API

```shell
$ curl \
        -X POST \
        -H "Content-Type: application/json" \
        -H "X-API-KEY: 63cad126-7373-45dd-a075-8687b148aeeb" \
        -d '{"email": "bob.smith@gmail.com"}' \
        localhost:8080/submit
```

## API Responses

| Status Code | Body   | Description                   | Example            |
| ----------- | ------ | ----------------------------- | ------------------ |
| 200         | string | operation was succcessful     | "success"          |
| 400         | string | some validation error message | "email is invalid" |
| 500         | string | some server side issue        | "server error"     |

## Version

0.1.0-ALPHA

## Contributing

1. Fork it (<https://github.com/anharhussainmiah/ecap/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## Contributors

- [anharmiah](https://github.com/anharhussainmiah) Anhar Miah - creator
