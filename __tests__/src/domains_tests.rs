#[cfg(test)]
mod domains_tests {
    use std::vec;

    use resend_client_rs::{
        domains::{
            CreateDomainRequest, CreateDomainResponse, DnsRecord, DnsRecordType, DomainDetails,
            DomainStatus, DomainSummary, EmailDnsRecord, ListDomainsResponse, DeleteDomainResponse,
        },
        Client,
    };

    #[tokio::test]
    async fn create_should_work() {
        let mut server = mockito::Server::new();
        let base_url = server.url();

        let create_domain_response = CreateDomainResponse {
            id: "mock-id".into(),
            created_at: "2023-11-19T10:00:00.000Z".into(),
            name: "domain.com".into(),
            region: "us-east-1".into(),
            status: DomainStatus::Pending,
            dnsProvider: "unknown".into(),
        };

        let domain_post_mock = server
            .mock("POST", "/domains")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(serde_json::to_string(&create_domain_response).unwrap())
            .create();

        let mut client = Client::new("api-key");
        client.domain_service.req_client.base_url = url::Url::parse(&base_url[..]).unwrap();
        let result = client
            .domain_service
            .create(&CreateDomainRequest {
                name: "domain.com".into(),
                region: None,
            })
            .await;
        let data = match result.unwrap() {
            resend_client_rs::reqlib::APIResponse::Success(data) => Some(data),
            _ => None,
        };
        assert_eq!(
            serde_json::to_string(&create_domain_response).unwrap(),
            serde_json::to_string(&data.unwrap()).unwrap(),
        );

        domain_post_mock.assert();
    }

    #[tokio::test]
    async fn list_should_work() {
        let mut server = mockito::Server::new();
        let base_url = server.url();

        let list_domains_response = ListDomainsResponse {
            data: vec![DomainSummary {
                id: "d91cd9bd-1176-453e-8fc1-35364d380206".into(),
                name: "example.com".into(),
                status: DomainStatus::NotStarted,
                created_at: "2023-04-26T20:21:26.347412+00:00".into(),
                region: "us-east-1".into(),
            }],
        };

        let domains_mock = server
            .mock("GET", "/domains")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(serde_json::to_string(&list_domains_response).unwrap())
            .create();

        let mut client = Client::new("api-key");
        client.domain_service.req_client.base_url = url::Url::parse(&base_url[..]).unwrap();
        let result = client.domain_service.list().await;
        let data = match result.unwrap() {
            resend_client_rs::reqlib::APIResponse::Success(data) => Some(data),
            _ => None,
        };
        assert_eq!(
            serde_json::to_string(&list_domains_response).unwrap(),
            serde_json::to_string(&data.unwrap()).unwrap(),
        );

        domains_mock.assert();
    }

    #[tokio::test]
    async fn get_should_work() {
        let mut server = mockito::Server::new();
        let base_url = server.url();

        let get_domain_response = DomainDetails {
            object: "domain".into(),
            id: "d91cd9bd-1176-453e-8fc1-35364d380206".into(),
            name: "example.com".into(),
            status: DomainStatus::NotStarted,
            created_at: "2023-04-26T20:21:26.347412+00:00".into(),
            region: "us-east-1".into(),
            records: vec![DnsRecord {
                record: EmailDnsRecord::Spf,
                name: "send".into(),
                r#type: DnsRecordType::Mx,
                ttl: "Auto".into(),
                status: DomainStatus::NotStarted,
                value: "feedback-smtp.us-east-1.amazonses.com".into(),
                priority: Some(10),
            }],
        };

        let domains_mock = server
            .mock("GET", "/domains/mock-id")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(serde_json::to_string(&get_domain_response).unwrap())
            .create();

        let mut client = Client::new("api-key");
        client.domain_service.req_client.base_url = url::Url::parse(&base_url[..]).unwrap();
        let result = client.domain_service.get("mock-id").await;
        let data = match result.unwrap() {
            resend_client_rs::reqlib::APIResponse::Success(data) => Some(data),
            _ => None,
        };
        assert_eq!(
            serde_json::to_string(&get_domain_response).unwrap(),
            serde_json::to_string(&data.unwrap()).unwrap(),
        );

        domains_mock.assert();
    }

    #[tokio::test]
    async fn delete_should_work() {
        let mut server = mockito::Server::new();
        let base_url = server.url();

        let delete_response = DeleteDomainResponse {
            object: "domain".into(),
            id: "d91cd9bd-1176-453e-8fc1-35364d380206".into(),
            deleted: false,
        };

        let domains_mock = server
            .mock("DELETE", "/domains/mock-id")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(serde_json::to_string(&delete_response).unwrap())
            .create();

        let mut client = Client::new("api-key");
        client.domain_service.req_client.base_url = url::Url::parse(&base_url[..]).unwrap();
        let result = client.domain_service.delete("mock-id").await;
        let data = match result.unwrap() {
            resend_client_rs::reqlib::APIResponse::Success(data) => Some(data),
            _ => None,
        };
        assert_eq!(
            serde_json::to_string(&delete_response).unwrap(),
            serde_json::to_string(&data.unwrap()).unwrap(),
        );

        domains_mock.assert();
    }
}
