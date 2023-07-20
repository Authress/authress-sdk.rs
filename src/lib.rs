#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;
extern crate reqwest;

pub mod apis;
pub mod models;

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
    pub fn new(configuration: &apis::configuration::Configuration) -> Self {
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