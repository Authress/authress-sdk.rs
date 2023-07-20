
use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_role`]
#[derive(Clone, Debug)]
pub struct CreateRoleParams {
    pub role: crate::models::Role
}

/// struct for passing parameters to the method [`delete_role`]
#[derive(Clone, Debug)]
pub struct DeleteRoleParams {
    /// The identifier of the role.
    pub role_id: String
}

/// struct for passing parameters to the method [`get_role`]
#[derive(Clone, Debug)]
pub struct GetRoleParams {
    /// The identifier of the role.
    pub role_id: String
}

/// struct for passing parameters to the method [`update_role`]
#[derive(Clone, Debug)]
pub struct UpdateRoleParams {
    /// The identifier of the role.
    pub role_id: String,
    pub role: crate::models::Role
}


/// struct for typed errors of method [`create_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRoleError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRoleError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRoleError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_roles`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRolesError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_role`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRoleError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}


pub struct RolesApi {
    pub configuration: configuration::Configuration
}

impl RolesApi {
    /// Creates a role with permissions.
    pub async fn create_role(&self, params: CreateRoleParams) -> Result<crate::models::Role, Error<CreateRoleError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let role = params.role;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/roles", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&role);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Remove a role. If a record references the role, that record will not be modified.
    pub async fn delete_role(&self, params: DeleteRoleParams) -> Result<(), Error<DeleteRoleError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let role_id = params.role_id;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/roles/{roleId}", "", roleId=crate::apis::urlencode(role_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Roles contain a list of permissions that will be applied to any user or resource
    pub async fn get_role(&self, params: GetRoleParams) -> Result<crate::models::Role, Error<GetRoleError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let role_id = params.role_id;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/roles/{roleId}", "", roleId=crate::apis::urlencode(role_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get all the account roles. Roles contain a list of permissions that will be applied to any user or resource
    pub async fn get_roles(&self) -> Result<crate::models::RoleCollection, Error<GetRolesError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/roles", "");
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetRolesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates a role adding or removing permissions.
    pub async fn update_role(&self, params: UpdateRoleParams) -> Result<crate::models::Role, Error<UpdateRoleError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let role_id = params.role_id;
        let role = params.role;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/roles/{roleId}", "", roleId=crate::apis::urlencode(role_id));
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::PUT, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&role);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateRoleError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
