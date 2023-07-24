
use reqwest;

use crate::{apis::ResponseContent, AuthressSettings};
use super::{Error};
/// struct for passing parameters to the method [`get_users`]
#[derive(Clone, Debug)]
pub struct GetUsersParams {
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging
    pub cursor: Option<String>,
    /// Filter to search users by. This is a case insensitive search through every text field.
    pub filter: Option<String>,
    /// Return only users that are part of the specified tenant. Users can only be part of one tenant, using this parameter will limit returned users that have logged into this tenant.
    pub tenant_id: Option<String>
}


/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

pub struct UsersApi {
    pub configuration: AuthressSettings
}

impl UsersApi {
    /// Removes the user, all access the user has been granted through Authress access records, and any related user data. This action is completed asynchronously.
    pub async fn delete_user(&self, user_id: String) -> Result<(), Error<DeleteUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/users/{userId}", "", userId=user_id);
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get the user data associated with a user. The data returned by this endpoint is highly variable based on the source OAuth provider. Avoid depending on undocumented properties.
    pub async fn get_user(&self, user_id: String) -> Result<crate::models::UserIdentity, Error<GetUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/users/{userId}", "", userId=user_id);
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns a paginated user list for the account. The data returned by this endpoint is highly variable based on the source OAuth provider. Avoid depending on undocumented properties.
    pub async fn get_users(&self, params: Option<GetUsersParams>) -> Result<crate::models::UserIdentityCollection, Error<GetUsersError>> {
        let local_var_configuration = &self.configuration;
        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/users", "");
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
            if let Some(ref local_var_str) = parsed_params.tenant_id {
                local_var_req_builder = local_var_req_builder.query(&[("tenantId", &local_var_str.to_string())]);
            }
        }
        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUsersError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
