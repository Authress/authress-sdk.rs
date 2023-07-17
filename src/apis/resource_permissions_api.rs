
use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_permissioned_resource`]
#[derive(Clone, Debug)]
pub struct GetPermissionedResourceParams {
    /// The uri path of a resource to validate, must be URL encoded, uri segments are allowed.
    pub resource_uri: String
}

/// struct for passing parameters to the method [`get_resource_users`]
#[derive(Clone, Debug)]
pub struct GetResourceUsersParams {
    /// The uri path of a resource to validate, must be URL encoded, uri segments are allowed.
    pub resource_uri: String,
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`update_permissioned_resource`]
#[derive(Clone, Debug)]
pub struct UpdatePermissionedResourceParams {
    /// The uri path of a resource to validate, must be URL encoded, uri segments are allowed.
    pub resource_uri: String,
    /// The contents of the permission to set on the resource. Overwrites existing data.
    pub permissioned_resource: crate::models::PermissionedResource
}


/// struct for typed errors of method [`get_permissioned_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionedResourceError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_permissioned_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPermissionedResourcesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_resource_users`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetResourceUsersError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_permissioned_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePermissionedResourceError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


/// Permissions can be set globally at a resource level. This will apply to all users in an account.
pub async fn get_permissioned_resource(configuration: &configuration::Configuration, params: GetPermissionedResourceParams) -> Result<crate::models::PermissionedResource, Error<GetPermissionedResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let resource_uri = params.resource_uri;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/resources/{resourceUri}", local_var_configuration.base_path, resourceUri=crate::apis::urlencode(resource_uri));
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
        let local_var_entity: Option<GetPermissionedResourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Permissions can be set globally at a resource level. Lists any resources with a globally set resource policy.
pub async fn get_permissioned_resources(configuration: &configuration::Configuration) -> Result<crate::models::PermissionedResourceCollection, Error<GetPermissionedResourcesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/resources", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetPermissionedResourcesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the resource users with explicit access to the resource. This result is a list of users that have some permission to the resource. Users with access to parent resources and users with access only to a sub-resource will not be returned in this result. In the case that the resource has multiple users, the list will be paginated.
pub async fn get_resource_users(configuration: &configuration::Configuration, params: GetResourceUsersParams) -> Result<crate::models::ResourceUsersCollection, Error<GetResourceUsersError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let resource_uri = params.resource_uri;
    let limit = params.limit;
    let cursor = params.cursor;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/resources/{resourceUri}/users", local_var_configuration.base_path, resourceUri=crate::apis::urlencode(resource_uri));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetResourceUsersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the global permissions on a resource. This applies to all users in an account.
pub async fn update_permissioned_resource(configuration: &configuration::Configuration, params: UpdatePermissionedResourceParams) -> Result<(), Error<UpdatePermissionedResourceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let resource_uri = params.resource_uri;
    let permissioned_resource = params.permissioned_resource;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/resources/{resourceUri}", local_var_configuration.base_path, resourceUri=crate::apis::urlencode(resource_uri));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&permissioned_resource);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UpdatePermissionedResourceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
