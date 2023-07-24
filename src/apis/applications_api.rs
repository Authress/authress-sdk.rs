
use reqwest;

use crate::{apis::ResponseContent, AuthressSettings};
use super::{Error};

/// struct for typed errors of method [`delegate_user_login`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DelegateUserLoginError {
    Status401(),
    Status403(),
    Status404(),
    UnknownValue(serde_json::Value),
}

pub struct ApplicationsApi {
    pub configuration: AuthressSettings
}

impl ApplicationsApi {
    /// Redirect the user to an external application to login them in. Authress uses OpenID and SAML configuration to securely pass the user's login session in your platform to an external platform. The user will then be logged into that platform.
    pub async fn delegate_user_login(&self, application_id: String, user_id: String) -> Result<crate::models::ApplicationDelegation, Error<DelegateUserLoginError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/v1/applications/{applicationId}/users/{userId}/delegation", "", applicationId=crate::apis::urlencode(application_id), userId=user_id);
        let local_var_req_builder = local_var_configuration.get_request_builder(reqwest::Method::POST, local_var_uri_str);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<DelegateUserLoginError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
