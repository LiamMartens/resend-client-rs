use std::collections::HashMap;

use reqwest::{Error, Method};
use serde::{Deserialize, Serialize};

use crate::reqlib::ReqClient;

#[derive(Debug, Serialize, Clone)]
pub struct Tag {
    name: String,
    value: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Attachment {
    content: Vec<u8>,
    filename: String,
    path: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SendEmailRequest<'a> {
    pub subject: String,
    pub from: String,
    pub to: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<&'a Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendEmailResponse {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Email {
    pub id: String,
    pub object: String,
    pub from: String,
    pub to: Vec<String>,
    pub created_at: String,
    pub subject: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub bcc: Option<Vec<Option<String>>>,
    pub cc: Option<Vec<Option<String>>>,
    pub reply_to: Option<Vec<Option<String>>>,
    pub last_event: String,
}

pub struct EmailService {
    pub req_client: ReqClient,
}

impl EmailService {
    pub fn new(req_client: ReqClient) -> EmailService {
        EmailService { req_client }
    }

    pub async fn send<'a>(
        &self,
        params: &'a SendEmailRequest<'a>,
    ) -> Result<SendEmailResponse, Error> {
        let req = self
            .req_client
            .new_body_request(Method::POST, "emails", Some(params));
        let result = req.send().await;
        match result {
            Ok(response) => match response.error_for_status() {
                Ok(success_response) => Ok(success_response.json().await.unwrap()),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }

    pub async fn get<'a>(&self, email_id: &'a str) -> Result<Email, Error> {
        let path = format!("emails/{}", email_id);
        let req = self.req_client.new_request(Method::GET, &path[..]);
        let result = req.send().await;
        match result {
            Ok(response) => match response.error_for_status() {
                Ok(success_response) => Ok(success_response.json().await.unwrap()),
                Err(err) => Err(err),
            },
            Err(err) => Err(err),
        }
    }
}
