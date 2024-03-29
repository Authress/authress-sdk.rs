
use reqwest;

use crate::{apis::ResponseContent, AuthressSettings};
use super::{Error};
/// struct for passing parameters to the method [`get_user_resources`]
#[derive(Default, Clone, Debug)]
pub struct GetUserResourcesParams {
    /// `TOP_LEVEL_ONLY` - returns only directly nested resources under the resourceUri. A query to `resourceUri=Collection` will return `Collection/resource_1`.<br>`INCLUDE_NESTED` - will return all sub-resources as well as deeply nested resources that the user has the specified permission to. A query to `resourceUri=Collection` will return `Collection/namespaces/ns/resources/resource_1`.<br><br>To return matching resources for nested resources, set this parameter to `INCLUDE_NESTED`.
    pub collection_configuration: Option<String>,
    /// Permission to check, '*' and scoped permissions can also be checked here. By default if the user has any permission explicitly to a resource, it will be included in the list.
    pub permissions: Option<String>,
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging
    pub cursor: Option<String>
}

/// struct for typed errors of method [`authorize_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthorizeUserError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_permissions_for_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserPermissionsForResourceError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_resources`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserResourcesError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_roles_for_resource`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserRolesForResourceError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

pub struct UserPermissionsApi {
    pub configuration: AuthressSettings
}

impl UserPermissionsApi {
    /// Performs the user authorization check. Does the user have the specified permission to the resource?
    pub async fn authorize_user(&self, user_id: String, resource_uri: String, permission: String) -> Result<(), Error<AuthorizeUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/users/{userId}/resources/{resourceUri}/permissions/{permission}", "", userId=user_id, resourceUri=crate::apis::urlencode(resource_uri), permission=permission);
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<AuthorizeUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get a summary of the permissions a user has to a particular resource.
    pub async fn get_user_permissions_for_resource(&self, user_id: String, resource_uri: String) -> Result<crate::models::PermissionCollection, Error<GetUserPermissionsForResourceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/users/{userId}/resources/{resourceUri}/permissions", "", userId=user_id, resourceUri=crate::apis::urlencode(resource_uri));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUserPermissionsForResourceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get the users resources. This result is a list of resource uris that a user has an permission to. By default only the top level matching resources are returned. To get a user's list of deeply nested resources, set the `collectionConfiguration` to be `INCLUDE_NESTED`. This collection is paginated.
    pub async fn get_user_resources(&self, user_id: String, resource_uri: Option<String>, params: Option<GetUserResourcesParams>) -> Result<crate::models::UserResourcesCollection, Error<GetUserResourcesError>> {
        let local_var_configuration = &self.configuration;
        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/users/{userId}/resources", "", userId=user_id);
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        if let Some(ref local_var_str) = resource_uri {
            local_var_req_builder = local_var_req_builder.query(&[("resourceUri", &local_var_str.to_string())]);
        }

        if let Some(ref parsed_params) = params {
            if let Some(ref local_var_str) = parsed_params.collection_configuration {
                local_var_req_builder = local_var_req_builder.query(&[("collectionConfiguration", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.permissions {
                local_var_req_builder = local_var_req_builder.query(&[("permissions", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.limit {
                local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
            }
            if let Some(ref local_var_str) = parsed_params.cursor {
                local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
            }
        }
        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUserResourcesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get a summary of the roles a user has to a particular resource. Users can be assigned roles from multiple access records, this may cause the same role to appear in the list more than once.
    pub async fn get_user_roles_for_resource(&self, user_id: String, resource_uri: String) -> Result<crate::models::UserRoleCollection, Error<GetUserRolesForResourceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/users/{userId}/resources/{resourceUri}/roles", "", userId=user_id, resourceUri=crate::apis::urlencode(resource_uri));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUserRolesForResourceError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
