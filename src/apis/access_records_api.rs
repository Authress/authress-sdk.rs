
use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_claim`]
#[derive(Clone, Debug)]
pub struct CreateClaimParams {
    pub claim_request: crate::models::ClaimRequest
}

/// struct for passing parameters to the method [`create_invite`]
#[derive(Clone, Debug)]
pub struct CreateInviteParams {
    pub invite: crate::models::Invite
}

/// struct for passing parameters to the method [`create_record`]
#[derive(Clone, Debug)]
pub struct CreateRecordParams {
    pub access_record: crate::models::AccessRecord
}

/// struct for passing parameters to the method [`create_request`]
#[derive(Clone, Debug)]
pub struct CreateRequestParams {
    pub access_request: crate::models::AccessRequest
}

/// struct for passing parameters to the method [`delete_invite`]
#[derive(Clone, Debug)]
pub struct DeleteInviteParams {
    /// The identifier of the invite.
    pub invite_id: String
}

/// struct for passing parameters to the method [`delete_record`]
#[derive(Clone, Debug)]
pub struct DeleteRecordParams {
    /// The identifier of the access record.
    pub record_id: String
}

/// struct for passing parameters to the method [`delete_request`]
#[derive(Clone, Debug)]
pub struct DeleteRequestParams {
    /// The identifier of the access request.
    pub request_id: String
}

/// struct for passing parameters to the method [`get_record`]
#[derive(Clone, Debug)]
pub struct GetRecordParams {
    /// The identifier of the access record.
    pub record_id: String
}

/// struct for passing parameters to the method [`get_records`]
#[derive(Clone, Debug)]
pub struct GetRecordsParams {
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging
    pub cursor: Option<String>,
    /// Filter to search records by. This is a case insensitive search through every text field.
    pub filter: Option<String>,
    /// Filter records by their current status.
    pub status: Option<String>
}

/// struct for passing parameters to the method [`get_request`]
#[derive(Clone, Debug)]
pub struct GetRequestParams {
    /// The identifier of the access request.
    pub request_id: String
}

/// struct for passing parameters to the method [`get_requests`]
#[derive(Clone, Debug)]
pub struct GetRequestsParams {
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging
    pub cursor: Option<String>,
    /// Filter requests by their current status.
    pub status: Option<String>
}

/// struct for passing parameters to the method [`respond_to_access_request`]
#[derive(Clone, Debug)]
pub struct RespondToAccessRequestParams {
    /// The identifier of the access request.
    pub request_id: String,
    pub access_request_response: crate::models::AccessRequestResponse
}

/// struct for passing parameters to the method [`respond_to_invite`]
#[derive(Clone, Debug)]
pub struct RespondToInviteParams {
    /// The identifier of the invite.
    pub invite_id: String
}

/// struct for passing parameters to the method [`update_record`]
#[derive(Clone, Debug)]
pub struct UpdateRecordParams {
    /// The identifier of the access record.
    pub record_id: String,
    pub access_record: crate::models::AccessRecord,
    /// The expected last time the record was modified. (<a href=\"https://tools.ietf.org/html/rfc7231#section-7.1.1.1\" target=\"_blank\">format</a>)
    pub if_unmodified_since: Option<String>
}


/// struct for typed errors of method [`create_claim`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClaimError {
    Status401(),
    Status403(),
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateInviteError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_record`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRecordError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRequestError {
    Status401(),
    Status403(),
    Status422(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteInviteError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_record`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRecordError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRequestError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_record`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRecordError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_records`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRecordsError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRequestError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_requests`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRequestsError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`respond_to_access_request`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RespondToAccessRequestError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`respond_to_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RespondToInviteError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_record`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRecordError {
    Status401(),
    Status403(),
    Status404(),
    Status412(),
    Status413(),
    UnknownValue(serde_json::Value),
}


/// Claim a resource by allowing a user to pick an identifier and receive admin access to that resource if it hasn't already been claimed. This only works for resources specifically marked as <strong>CLAIM</strong>. The result will be a new access record listing that user as the admin for this resource. The resourceUri will be appended to the collection resource uri using a '/' (forward slash) automatically.
pub async fn create_claim(configuration: &configuration::Configuration, params: CreateClaimParams) -> Result<serde_json::Value, Error<CreateClaimError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let claim_request = params.claim_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/claims", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&claim_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateClaimError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Invites are used to easily assign permissions to users that have not been created in your identity provider yet. 1. This generates an invite record. 2. Send the invite ID to the user. 3. Log the user in. 4. As the user PATCH the /invite/{inviteId} endpoint 5. This accepts the invite.         When the user accepts the invite they will automatically receive the permissions assigned in the Invite. Invites automatically expire after 7 days.
pub async fn create_invite(configuration: &configuration::Configuration, params: CreateInviteParams) -> Result<crate::models::Invite, Error<CreateInviteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let invite = params.invite;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/invites", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&invite);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateInviteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Specify user roles for specific resources. (Records have a maximum size of ~100KB)
pub async fn create_record(configuration: &configuration::Configuration, params: CreateRecordParams) -> Result<crate::models::AccessRecord, Error<CreateRecordError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let access_record = params.access_record;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/records", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&access_record);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRecordError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Specify a request in the form of an access record that an admin can approve.
pub async fn create_request(configuration: &configuration::Configuration, params: CreateRequestParams) -> Result<crate::models::AccessRequest, Error<CreateRequestError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let access_request = params.access_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/requests", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&access_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes an invite.
pub async fn delete_invite(configuration: &configuration::Configuration, params: DeleteInviteParams) -> Result<(), Error<DeleteInviteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let invite_id = params.invite_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/invites/{inviteId}", local_var_configuration.base_path, inviteId=crate::apis::urlencode(invite_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteInviteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove an access record, removing associated permissions from all users in record. If a user has a permission from another record, that permission will not be removed. (Note: This disables the record by changing the status to <strong>DELETED</strong> but not completely remove the record for tracking purposes.
pub async fn delete_record(configuration: &configuration::Configuration, params: DeleteRecordParams) -> Result<(), Error<DeleteRecordError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let record_id = params.record_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/records/{recordId}", local_var_configuration.base_path, recordId=crate::apis::urlencode(record_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteRecordError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove an access request.
pub async fn delete_request(configuration: &configuration::Configuration, params: DeleteRequestParams) -> Result<(), Error<DeleteRequestError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let request_id = params.request_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/requests/{requestId}", local_var_configuration.base_path, requestId=crate::apis::urlencode(request_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Access records contain information assigning permissions to users for resources.
pub async fn get_record(configuration: &configuration::Configuration, params: GetRecordParams) -> Result<crate::models::AccessRecord, Error<GetRecordError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let record_id = params.record_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/records/{recordId}", local_var_configuration.base_path, recordId=crate::apis::urlencode(record_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRecordError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a paginated records list for the account. Only records the user has access to are returned. This query resource is meant for administrative actions only, therefore has a lower rate limit tier than user permissions related resources. Additionally, the results from a query to Access Records may be delayed up to 5 minutes.
pub async fn get_records(configuration: &configuration::Configuration, params: GetRecordsParams) -> Result<crate::models::AccessRecordCollection, Error<GetRecordsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let cursor = params.cursor;
    let filter = params.filter;
    let status = params.status;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/records", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRecordsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Access request contain information requesting permissions for users to resources.
pub async fn get_request(configuration: &configuration::Configuration, params: GetRequestParams) -> Result<crate::models::AccessRequest, Error<GetRequestError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let request_id = params.request_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/requests/{requestId}", local_var_configuration.base_path, requestId=crate::apis::urlencode(request_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a paginated request list. Only requests the user has access to are returned. This query resource is meant for administrative actions only, therefore has a lower rate limit tier than user permissions related resources.
pub async fn get_requests(configuration: &configuration::Configuration, params: GetRequestsParams) -> Result<crate::models::AccessRequestCollection, Error<GetRequestsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let limit = params.limit;
    let cursor = params.cursor;
    let status = params.status;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/requests", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetRequestsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an access request, approving it or denying it.
pub async fn respond_to_access_request(configuration: &configuration::Configuration, params: RespondToAccessRequestParams) -> Result<crate::models::AccessRequest, Error<RespondToAccessRequestError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let request_id = params.request_id;
    let access_request_response = params.access_request_response;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/requests/{requestId}", local_var_configuration.base_path, requestId=crate::apis::urlencode(request_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&access_request_response);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RespondToAccessRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Accepts an invite by claiming this invite by this user. The user access token used for this request will gain the permissions associated with the invite.
pub async fn respond_to_invite(configuration: &configuration::Configuration, params: RespondToInviteParams) -> Result<crate::models::Account, Error<RespondToInviteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let invite_id = params.invite_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/invites/{inviteId}", local_var_configuration.base_path, inviteId=crate::apis::urlencode(invite_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RespondToInviteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates an access record adding or removing user permissions to resources. (Records have a maximum size of ~100KB)
pub async fn update_record(configuration: &configuration::Configuration, params: UpdateRecordParams) -> Result<(), Error<UpdateRecordError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let record_id = params.record_id;
    let access_record = params.access_record;
    let if_unmodified_since = params.if_unmodified_since;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/records/{recordId}", local_var_configuration.base_path, recordId=crate::apis::urlencode(record_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = if_unmodified_since {
        local_var_req_builder = local_var_req_builder.header("If-Unmodified-Since", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&access_record);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdateRecordError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

