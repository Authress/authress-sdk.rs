
use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_accounts`]
#[derive(Clone, Debug)]
pub struct GetAccountsParams {
    /// Ensure the accounts list is not cached before this time.
    pub earliest_cache_time: Option<String>
}


/// struct for typed errors of method [`delegate_authentication`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DelegateAuthenticationError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountError {
    Status401(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_identities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountIdentitiesError {
    Status401(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_accounts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountsError {
    Status401(),
    UnknownValue(serde_json::Value),
}


pub struct AccountsApi {
    pub configuration: configuration::Configuration
}

impl AccountsApi {
    /// An identity is a JWT subscriber *sub* and issuer *iss*. Only one account my be linked to a particular JWT combination. Allows calling the API with a federated token directly instead of using a client access key.
    pub async fn delegate_authentication(&self, identity_request: crate::models::IdentityRequest) -> Result<(), Error<DelegateAuthenticationError>> {
        let local_var_configuration = &self.configuration;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/identities", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);
        local_var_req_builder = local_var_req_builder.json(&identity_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DelegateAuthenticationError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Includes the original configuration information.
    pub async fn get_account(&self, account_id: String) -> Result<crate::models::Account, Error<GetAccountError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/accounts/{accountId}", "", accountId=crate::apis::urlencode(account_id));
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetAccountError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns a list of identities linked for this account.
    pub async fn get_account_identities(&self) -> Result<crate::models::IdentityCollection, Error<GetAccountIdentitiesError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/identities", "");
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetAccountIdentitiesError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Returns a list of accounts that the user has access to.
    pub async fn get_accounts(&self, params: GetAccountsParams) -> Result<crate::models::AccountCollection, Error<GetAccountsError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let earliest_cache_time = params.earliest_cache_time;


        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/accounts", "");
        let mut local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::GET, local_var_uri_str);

        if let Some(ref local_var_str) = earliest_cache_time {
            local_var_req_builder = local_var_req_builder.query(&[("earliestCacheTime", &local_var_str.to_string())]);
        }
        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetAccountsError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
