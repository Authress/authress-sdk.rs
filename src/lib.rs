#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;

use reqwest::RequestBuilder;

#[derive(Default, Debug, Clone)]
pub struct AuthressSettings {
    pub client: reqwest::Client,

    pub base_url: String,
    pub service_client_access_key: String
}

impl AuthressSettings {
    pub fn new(authress_api_url: String, service_client_access_key: String) -> AuthressSettings {
        AuthressSettings {
            client: reqwest::Client::new(),

            base_url: authress_api_url.to_owned(),
            service_client_access_key: service_client_access_key            
        }
    }

    pub fn get_request_builder(&self, method: reqwest::Method, path_uri: String) -> RequestBuilder {
        let local_var_uri_str = format!("{}{}", self.base_url, path_uri);
        
        return self.client
            .request(method, local_var_uri_str)
            .header(reqwest::header::USER_AGENT, "Authress SDK; Rust; ;")
            .bearer_auth(self.service_client_access_key.to_owned());
    }
}


pub struct AuthressClient {
    pub access_records: apis::access_records_api::AccessRecordApi,
    pub accounts: apis::accounts_api::AccountsApi,
    pub applications: apis::applications_api::ApplicationsApi,
    pub connections: apis::connections_api::ConnectionsApi,
    pub extensions: apis::extensions_api::ExtensionsApi,
    pub groups: apis::groups_api::GroupsApi,
    pub resource_permissions: apis::resource_permissions_api::ResourcePermissionsApi,
    pub roles: apis::roles_api::RolesApi,
    pub service_clients: apis::service_clients_api::ServiceClientsApi,
    pub tenants: apis::tenants_api::TenantsApi,
    pub user_permissions: apis::user_permissions_api::UserPermissionsApi,
    pub users: apis::users_api::UsersApi
}

impl AuthressClient {
    pub fn new(configuration: &AuthressSettings) -> Self {
        Self {
            access_records: apis::access_records_api::AccessRecordApi { configuration: configuration.clone() },
            accounts: apis::accounts_api::AccountsApi { configuration: configuration.clone() },
            applications: apis::applications_api::ApplicationsApi { configuration: configuration.clone() },
            connections: apis::connections_api::ConnectionsApi { configuration: configuration.clone() },
            extensions: apis::extensions_api::ExtensionsApi { configuration: configuration.clone() },
            groups: apis::groups_api::GroupsApi { configuration: configuration.clone() },
            resource_permissions: apis::resource_permissions_api::ResourcePermissionsApi { configuration: configuration.clone() },
            roles: apis::roles_api::RolesApi { configuration: configuration.clone() },
            service_clients: apis::service_clients_api::ServiceClientsApi { configuration: configuration.clone() },
            tenants: apis::tenants_api::TenantsApi { configuration: configuration.clone() },
            user_permissions: apis::user_permissions_api::UserPermissionsApi { configuration: configuration.clone() },
            users: apis::users_api::UsersApi { configuration: configuration.clone() },
        }
    }
}