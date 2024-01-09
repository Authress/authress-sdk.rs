
use reqwest;

use crate::{apis::ResponseContent, AuthressSettings};
use super::Error;

/// struct for passing parameters to the method [`get_records`]
#[derive(Default, Clone, Debug)]
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

/// struct for passing parameters to the method [`get_requests`]
#[derive(Default, Clone, Debug)]
pub struct GetRequestsParams {
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging
    pub cursor: Option<String>,
    /// Filter requests by their current status.
    pub status: Option<String>
}

/// struct for passing parameters to the method [`respond_to_invite`]
#[derive(Default, Clone, Debug)]
pub struct RespondToInviteParams {
}

/// struct for passing parameters to the method [`update_record`]
#[derive(Default, Clone, Debug)]
pub struct UpdateRecordParams {
    /// The expected last time the record was modified.
    pub expected_last_modified_time: Option<String>
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

pub struct AccessRecordApi {
    pub configuration: AuthressSettings
}

impl AccessRecordApi {
    /// Claim a resource by allowing a user to pick an identifier and receive admin access to that resource if it hasn't already been claimed. This only works for resources specifically marked as <strong>CLAIM</strong>. The result will be a new access record listing that user as the admin for this resource. The resourceUri will be appended to the collection resource uri using a '/' (forward slash) automatically.
    pub async fn create_claim(&self, claim_request: crate::models::ClaimRequest) -> Result<serde_json::Value, Error<CreateClaimError>> {
        let local_var_configuration = &self.configuration;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/claims", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
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
    pub async fn create_invite(&self, invite: crate::models::Invite) -> Result<crate::models::Invite, Error<CreateInviteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/invites", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
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
    pub async fn create_record(&self, access_record: crate::models::AccessRecord) -> Result<crate::models::AccessRecord, Error<CreateRecordError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/records", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
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
    pub async fn create_request(&self, access_request: crate::models::AccessRequest) -> Result<crate::models::AccessRequest, Error<CreateRequestError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/requests", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
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
    pub async fn delete_invite(&self, invite_id: String) -> Result<(), Error<DeleteInviteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/invites/{inviteId}", "", inviteId=crate::apis::urlencode(invite_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

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
    pub async fn delete_record(&self, record_id: String) -> Result<(), Error<DeleteRecordError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/records/{recordId}", "", recordId=crate::apis::urlencode(record_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

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
    pub async fn delete_request(&self, request_id: String) -> Result<(), Error<DeleteRequestError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/requests/{requestId}", "", requestId=crate::apis::urlencode(request_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

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
    pub async fn get_record(&self, record_id: String) -> Result<crate::models::AccessRecord, Error<GetRecordError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/records/{recordId}", "", recordId=crate::apis::urlencode(record_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

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
    pub async fn get_records(&self, params: Option<GetRecordsParams>) -> Result<crate::models::AccessRecordCollection, Error<GetRecordsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/records", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        if let Some(ref parsed_params) = params {
            if let Some(ref local_var_str) = parsed_params.limit {
                local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.cursor {
                local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.filter {
                local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.status {
                local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
            }
        }
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
    pub async fn get_request(&self, request_id: String) -> Result<crate::models::AccessRequest, Error<GetRequestError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/requests/{requestId}", "", requestId=crate::apis::urlencode(request_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

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
    pub async fn get_requests(&self, params: Option<GetRequestsParams>) -> Result<crate::models::AccessRequestCollection, Error<GetRequestsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/requests", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        if let Some(ref parsed_params) = params {
            if let Some(ref local_var_str) = parsed_params.limit {
                local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.cursor {
                local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.status {
                local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
            }
        }
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
    pub async fn respond_to_access_request(&self, request_id: String, access_request_response: crate::models::AccessRequestResponse) -> Result<crate::models::AccessRequest, Error<RespondToAccessRequestError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/requests/{requestId}", "", requestId=crate::apis::urlencode(request_id));
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::PATCH, local_var_uri_str);
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
    pub async fn respond_to_invite(&self, invite_id: String) -> Result<crate::models::Account, Error<RespondToInviteError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/invites/{inviteId}", "", inviteId=crate::apis::urlencode(invite_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::PATCH, local_var_uri_str);

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
    pub async fn update_record(&self, record_id: String, access_record: crate::models::AccessRecord, params: UpdateRecordParams) -> Result<(), Error<UpdateRecordError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let expected_last_modified_time = params.expected_last_modified_time;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/records/{recordId}", "", recordId=crate::apis::urlencode(record_id));
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::PUT, local_var_uri_str);

        if let Some(local_var_param_value) = expected_last_modified_time {
            local_var_req_builder = local_var_req_builder.header("If-Unmodified-Since", local_var_param_value.to_string());
        }
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
}