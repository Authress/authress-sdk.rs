
use reqwest;

use crate::{apis::ResponseContent, AuthressSettings};
use super::{Error};

/// struct for passing parameters to the method [`get_groups`]
#[derive(Clone, Debug)]
pub struct GetGroupsParams {
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging
    pub cursor: Option<String>,
    /// Filter to search groups by. This is a case insensitive search through every text field.
    pub filter: Option<String>
}

/// struct for typed errors of method [`create_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateGroupError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGroupError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_groups`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGroupsError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_group`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGroupError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


pub struct GroupsApi {
    pub configuration: AuthressSettings
}

impl GroupsApi {
    /// Specify users to be included in a new group. (Groups have a maximum size of ~100KB)
    pub async fn create_group(&self, group: crate::models::Group) -> Result<crate::models::Group, Error<CreateGroupError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/groups", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&group);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateGroupError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Remove a group, users will lose any role that was assigned through membership of this group. This action cannot be undone.
    pub async fn delete_group(&self, group_id: String) -> Result<(), Error<DeleteGroupError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/groups/{groupId}", "", groupId=crate::apis::urlencode(group_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteGroupError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// A group contains multiple users which can be added to an access record, and should be assigned the same roles at the same time.
    pub async fn get_group(&self, group_id: String) -> Result<crate::models::Group, Error<GetGroupError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/groups/{groupId}", "", groupId=crate::apis::urlencode(group_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetGroupError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns a paginated groups list for the account. Only groups the user has access to are returned. This query resource is meant for administrative actions only, therefore has a lower rate limit tier than user permissions related resources.
    pub async fn get_groups(&self, params: Option<GetGroupsParams>) -> Result<crate::models::GroupCollection, Error<GetGroupsError>> {
        let local_var_configuration = &self.configuration;
        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/groups", "");
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
        }
        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetGroupsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates a group adding or removing user. Change a group updates the permissions and roles the users have access to. (Groups have a maximum size of ~100KB)
    pub async fn update_group(&self, group_id: String, group: crate::models::Group) -> Result<crate::models::Group, Error<UpdateGroupError>> {
        let local_var_configuration = &self.configuration;
        
        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/groups/{groupId}", "", groupId=crate::apis::urlencode(group_id));
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::PUT, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&group);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateGroupError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
