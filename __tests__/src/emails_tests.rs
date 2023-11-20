#[cfg(test)]

mod emails_tests {
    use resend_client_rs::{
        emails::{Email, SendEmailRequest, SendEmailResponse},
        Client,
    };

    #[tokio::test]
    async fn send_should_work() {
        let mut server = mockito::Server::new();
        let base_url = server.url();

        let send_email_response = SendEmailResponse {
            id: "mock-id".to_string(),
        };

        let email_post_mock = server
            .mock("POST", "/emails")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(serde_json::to_string(&send_email_response).unwrap())
            .create();

        let mut client = Client::new("api-key");
        client.email_service.req_client.base_url = url::Url::parse(&base_url[..]).unwrap();
        let result = client
            .email_service
            .send(&SendEmailRequest {
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
            })
            .await;
        let data = match result.unwrap() {
            resend_client_rs::reqlib::APIResponse::Success(data) => Some(data),
            _ => None,
        };
        assert_eq!(data.unwrap().id, "mock-id");

        email_post_mock.assert();
    }

    #[tokio::test]
    async fn get_should_work() {
        let mut server = mockito::Server::new();
        let base_url = server.url();

        let email_details = Email {
            id: "id".to_string(),
            object: "email".to_string(),
            from: "from@domain.com".to_string(),
            to: vec!["to@domain.com".to_string()],
            created_at: "2023-11-19T10:00:00.000Z".to_string(),
            subject: "My subject".to_string(),
            html: None,
            text: None,
            bcc: None,
            cc: None,
            reply_to: None,
            last_event: "delivered".to_string(),
        };

        let email_post_mock = server
            .mock("GET", "/emails/mock-id")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(serde_json::to_string(&email_details).unwrap())
            .create();

        let mut client = Client::new("api-key");
        client.email_service.req_client.base_url = url::Url::parse(&base_url[..]).unwrap();
        let result = client.email_service.get("mock-id").await;
        let data = match result.unwrap() {
            resend_client_rs::reqlib::APIResponse::Success(data) => Some(data),
            _ => None,
        };

        assert_eq!(
            serde_json::to_string(&email_details).unwrap(),
            serde_json::to_string(&data.unwrap()).unwrap()
        );

        email_post_mock.assert();
    }

    #[tokio::test]
    async fn integration_test() {
        // this test will only run if an resend api key is provided
        let api_key = std::env::var("RESEND_API_KEY");
        let from_value = std::env::var("RESEND_FROM");
        let to_value = std::env::var("RESEND_TO");

        if api_key.is_ok() && from_value.is_ok() && to_value.is_ok() {
            let mut client = Client::new(api_key.unwrap());
            // client.email_service.req_client.base_url = url::Url::parse("http://localhost:3000").unwrap();
            let result = client
                .email_service
                .send(&SendEmailRequest {
                    subject: "My subject".to_string(),
                    from: from_value.unwrap(),
                    to: vec![to_value.unwrap()],
                    cc: None,
                    bcc: None,
                    reply_to: None,
                    html: None,
                    text: Some("Hello World".to_string()),
                    tags: None,
                    attachments: None,
                    headers: None,
                })
                .await;
            let data = match result.unwrap() {
                resend_client_rs::reqlib::APIResponse::Success(data) => Some(data),
                _ => None,
            };
            assert!(data.unwrap().id != "");
        } else {
            assert!(true);
        }
    }
}
