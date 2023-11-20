# resend-client-rs

[![crates.io](https://img.shields.io/crates/v/resend-client-rs.svg)](https://crates.io/crates/resend-client-rs)
[![Released API docs](https://docs.rs/resend-client-rs/badge.svg)](https://docs.rs/resend-client-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Client for sending emails with Resend. Main logic ported from official [Go library](https://github.com/resendlabs/resend-go).

This library is more complete than other available options since it includes the domain endpoints.

## Installation

`cargo add resend-client-rs`

## Usage

```rust
use resend_client_rs::Client;

let client = Client::new("API_KEY");
let result = client.email_service.send(&SendEmailRequest {
    subject: "My subject".to_string(),
    from: "from@domain.com".to_string(),
    to: vec!["to@domain.com".to_string()],
    cc: None,
    bcc: None,
    reply_to: None,
    html: None,
    text: None,
    tags: None,
    attachments: None,
    headers: None,
}).await;
```
