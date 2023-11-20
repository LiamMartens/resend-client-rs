pub mod domains;
pub mod emails;
pub mod reqlib;

use domains::DomainService;
use emails::EmailService;
use reqlib::ReqClient;

pub struct Client {
    pub raw_client: ReqClient,
    pub email_service: EmailService,
    pub domain_service: DomainService,
}

impl Client {
    pub fn new<T: Into<String> + Clone>(api_key: T) -> Client {
        let raw_client = ReqClient::new(api_key.clone().into());
        let email_service = EmailService::new(ReqClient::new(api_key.clone().into()));
        let domain_service = DomainService::new(ReqClient::new(api_key.clone().into()));
        Client {
            raw_client,
            email_service,
            domain_service,
        }
    }
}
