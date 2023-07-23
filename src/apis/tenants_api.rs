
use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for typed errors of method [`create_tenant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTenantError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tenant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTenantError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tenant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTenantError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tenants`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTenantsError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_tenant`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTenantError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

pub struct TenantsApi {
    pub configuration: configuration::Configuration
}

impl TenantsApi {
    /// Specify tenant identity details for tenant tracking, separation, and user management. Tenant identifiers are persisted to access tokens created by Authress.
    pub async fn create_tenant(&self, tenant: crate::models::Tenant) -> Result<crate::models::Tenant, Error<CreateTenantError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/tenants", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&tenant);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateTenantError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete a tenant. If a connection was created for the tenant then it is deleted as well.
    pub async fn delete_tenant(&self, tenant_id: String) -> Result<(), Error<DeleteTenantError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/tenants/{tenantId}", "", tenantId=crate::apis::urlencode(tenant_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteTenantError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Get the tenant details for an Authress tenant.
    pub async fn get_tenant(&self, tenant_id: String) -> Result<crate::models::Tenant, Error<GetTenantError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/tenants/{tenantId}", "", tenantId=crate::apis::urlencode(tenant_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetTenantError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns a paginated tenants list for the account. Only tenants the user has access to are returned.
    pub async fn get_tenants(&self) -> Result<crate::models::TenantCollection, Error<GetTenantsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/tenants", "");
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetTenantsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates the tenants information and linked tenant if it exists.
    pub async fn update_tenant(&self, tenant_id: String, tenant: crate::models::Tenant) -> Result<crate::models::Tenant, Error<UpdateTenantError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/tenants/{tenantId}", "", tenantId=crate::apis::urlencode(tenant_id));
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::PUT, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&tenant);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateTenantError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
