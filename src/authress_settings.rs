use reqwest::RequestBuilder;



#[derive(Default, Debug, Clone)]
pub struct AuthressSettings {
    pub authress_api_url: String,
    pub client: reqwest::Client,
    pub service_client_access_key: String
}

impl AuthressSettings {
    pub fn new(authress_api_url: String, service_client_access_key: String) -> AuthressSettings {
        AuthressSettings {
            authress_api_url: authress_api_url.to_owned(),
            service_client_access_key: service_client_access_key,
            client: reqwest::Client::new()
        }
    }

    pub fn get_request_builder(&self, method: reqwest::Method, path_uri: String) -> RequestBuilder {
        let local_var_uri_str = format!("{}{}", self.authress_api_url, path_uri);
        
        return self.client
            .request(method, local_var_uri_str)
            .header(reqwest::header::USER_AGENT, "Authress SDK; Rust; ;")
            .bearer_auth(self.service_client_access_key.to_owned());
    }
}
