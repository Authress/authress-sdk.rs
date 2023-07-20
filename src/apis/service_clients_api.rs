
use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_client`]
#[derive(Clone, Debug)]
pub struct CreateClientParams {
    pub client: crate::models::Client
}

/// struct for passing parameters to the method [`delete_access_key`]
#[derive(Clone, Debug)]
pub struct DeleteAccessKeyParams {
    /// The unique identifier of the client.
    pub client_id: String,
    /// The ID of the access key to remove from the client.
    pub key_id: String
}

/// struct for passing parameters to the method [`delete_client`]
#[derive(Clone, Debug)]
pub struct DeleteClientParams {
    /// The unique identifier for the client.
    pub client_id: String
}

/// struct for passing parameters to the method [`get_client`]
#[derive(Clone, Debug)]
pub struct GetClientParams {
    /// The unique identifier for the client.
    pub client_id: String
}

/// struct for passing parameters to the method [`get_clients`]
#[derive(Clone, Debug)]
pub struct GetClientsParams {
    /// Max number of results to return
    pub limit: Option<u32>,
    /// Continuation cursor for paging.
    pub cursor: Option<String>
}

/// struct for passing parameters to the method [`request_access_key`]
#[derive(Clone, Debug)]
pub struct RequestAccessKeyParams {
    /// The unique identifier of the client.
    pub client_id: String
}

/// struct for passing parameters to the method [`update_client`]
#[derive(Clone, Debug)]
pub struct UpdateClientParams {
    /// The unique identifier for the client.
    pub client_id: String,
    pub client: crate::models::Client
}


/// struct for typed errors of method [`create_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateClientError {
    Status401(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_access_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAccessKeyError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteClientError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_clients`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetClientsError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`request_access_key`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestAccessKeyError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_client`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateClientError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

pub struct ServiceClientsApi {
    pub configuration: configuration::Configuration
}

impl ServiceClientsApi {
    /// Creates a service client to interact with Authress or any other service on behalf of users. Each client has secret private keys used to authenticate with Authress. To use service clients created through other mechanisms, skip creating a client and create access records with the client identifier.
    pub async fn create_client(&self, params: CreateClientParams) -> Result<crate::models::Client, Error<CreateClientError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let client = params.client;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/clients", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&client);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateClientError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Deletes an access key for a client prevent it from being used to authenticate with Authress.
    pub async fn delete_access_key(&self, params: DeleteAccessKeyParams) -> Result<(), Error<DeleteAccessKeyError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let client_id = params.client_id;
        let key_id = params.key_id;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/clients/{clientId}/access-keys/{keyId}", "", clientId=crate::apis::urlencode(client_id), keyId=crate::apis::urlencode(key_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteAccessKeyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This deletes the service client.
    pub async fn delete_client(&self, params: DeleteClientParams) -> Result<(), Error<DeleteClientError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let client_id = params.client_id;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/clients/{clientId}", "", clientId=crate::apis::urlencode(client_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::DELETE, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteClientError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns all information related to client except for the private access keys.
    pub async fn get_client(&self, params: GetClientParams) -> Result<crate::models::Client, Error<GetClientError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let client_id = params.client_id;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/clients/{clientId}", "", clientId=crate::apis::urlencode(client_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetClientError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns all clients that the user has access to in the account.
    pub async fn get_clients(&self, params: GetClientsParams) -> Result<crate::models::ClientCollection, Error<GetClientsError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let limit = params.limit;
        let cursor = params.cursor;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/clients", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        if let Some(ref local_var_str) = limit {
            local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = cursor {
            local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
        }
        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetClientsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Create a new access key for the client so that a service can authenticate with Authress as that client. Using the client will allow delegation of permission checking of users. (Limited to 5 Active keys per client)
    pub async fn request_access_key(&self, params: RequestAccessKeyParams) -> Result<crate::models::ClientAccessKey, Error<RequestAccessKeyError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let client_id = params.client_id;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/clients/{clientId}/access-keys", "", clientId=crate::apis::urlencode(client_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<RequestAccessKeyError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Updates a client information.
    pub async fn update_client(&self, params: UpdateClientParams) -> Result<crate::models::Client, Error<UpdateClientError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let client_id = params.client_id;
        let client = params.client;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/clients/{clientId}", "", clientId=crate::apis::urlencode(client_id));
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::PUT, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&client);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UpdateClientError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
