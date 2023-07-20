use reqwest::RequestBuilder;



#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
    pub service_client_access_key: String
}

impl Configuration {
    pub fn new(authress_base_url: String, service_client_access_key: String) -> Configuration {
        Configuration {
            base_path: authress_base_url.to_owned(),
            service_client_access_key: service_client_access_key,
            client: reqwest::Client::new()
        }
    }

    pub fn get_request_builder(&self, method: reqwest::Method, path_uri: String) -> RequestBuilder {
        let local_var_uri_str = format!("{}{}", self.base_path, path_uri);
        
        return self.client
            .request(method, local_var_uri_str)
            .header(reqwest::header::USER_AGENT, "Authress Rust SDK")
            .bearer_auth(self.service_client_access_key.to_owned());
    }
}
