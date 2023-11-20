use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::reqlib::{APIResponse, ReqClient};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum DnsRecordType {
    Mx,
    Cname,
    Txt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum EmailDnsRecord {
    Spf,
    Dkim,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DomainStatus {
    Pending,
    Verified,
    Failed,
    TemporaryFailure,
    NotStarted,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateDomainRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DnsRecord {
    pub record: EmailDnsRecord,
    pub r#type: DnsRecordType,
    pub name: String,
    pub ttl: String,
    pub status: DomainStatus,
    pub value: String,
    pub priority: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDomainResponse {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub status: DomainStatus,
    pub region: String,
    // this property is not snake case in the Resend API for some reason
    pub dnsProvider: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DomainDetails {
    pub id: String,
    pub object: String,
    pub name: String,
    pub created_at: String,
    pub status: DomainStatus,
    pub region: String,
    pub records: Vec<DnsRecord>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DomainSummary {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub status: DomainStatus,
    pub region: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListDomainsResponse {
    pub data: Vec<DomainSummary>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VerifyDomainResponse {
    pub id: String,
    pub object: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeleteDomainResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

pub struct DomainService {
    pub req_client: ReqClient,
}

impl DomainService {
    pub fn new(req_client: ReqClient) -> DomainService {
        DomainService { req_client }
    }

    pub async fn create<'a>(
        &self,
        params: &'a CreateDomainRequest,
    ) -> Result<APIResponse<CreateDomainResponse>, APIResponse<CreateDomainResponse>> {
        let req = self
            .req_client
            .new_body_request(Method::POST, "domains", Some(params));
        let result = self.req_client.exec::<CreateDomainResponse>(req).await;
        result
    }

    pub async fn verify<'a, T: Into<String>>(
        &self,
        domain_id: T,
    ) -> Result<APIResponse<VerifyDomainResponse>, APIResponse<VerifyDomainResponse>> {
        let path = format!("domains/{}", domain_id.into());
        let req = self.req_client.new_request(Method::POST, &path[..]);
        let result = self.req_client.exec::<VerifyDomainResponse>(req).await;
        result
    }

    pub async fn get<'a, T: Into<String>>(
        &self,
        domain_id: T,
    ) -> Result<APIResponse<DomainDetails>, APIResponse<DomainDetails>> {
        let path = format!("domains/{}", domain_id.into());
        let req = self.req_client.new_request(Method::GET, &path[..]);
        let result = self.req_client.exec::<DomainDetails>(req).await;
        result
    }

    pub async fn list<'a>(
        &self,
    ) -> Result<APIResponse<ListDomainsResponse>, APIResponse<ListDomainsResponse>> {
        let req = self.req_client.new_request(Method::GET, "domains");
        let result = self.req_client.exec::<ListDomainsResponse>(req).await;
        result
    }

    pub async fn delete<'a, T: Into<String>>(
        &self,
        domain_id: T,
    ) -> Result<APIResponse<DeleteDomainResponse>, APIResponse<DeleteDomainResponse>> {
        let path = format!("domains/{}", domain_id.into());
        let req = self.req_client.new_request(Method::DELETE, &path[..]);
        let result = self.req_client.exec::<DeleteDomainResponse>(req).await;
        result
    }
}
