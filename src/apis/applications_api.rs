
use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`delegate_user_login`]
#[derive(Clone, Debug)]
pub struct DelegateUserLoginParams {
    /// The application to have the user log into.
    pub application_id: String,
    /// The user.
    pub user_id: String
}


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
    pub configuration: configuration::Configuration
}

impl ApplicationsApi {
    /// Redirect the user to an external application to login them in. Authress uses OpenID and SAML configuration to securely pass the user's login session in your platform to an external platform. The user will then be logged into that platform.
    pub async fn delegate_user_login(&self, params: DelegateUserLoginParams) -> Result<crate::models::ApplicationDelegation, Error<DelegateUserLoginError>> {
        let local_var_configuration = &self.configuration;

        // unbox the parameters
        let application_id = params.application_id;
        let user_id = params.user_id;


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
