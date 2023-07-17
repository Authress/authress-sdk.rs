[Back to Repository](../README.md)

## Documentation for API Endpoints

All URIs are relative to your [Authress Host URL](https://authress.io/app/#/api).

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccessRecordsApi* | [**create_claim**](docs/AccessRecordsApi.md#create_claim) | **POST** /v1/claims | Create resource Claim
*AccessRecordsApi* | [**create_invite**](docs/AccessRecordsApi.md#create_invite) | **POST** /v1/invites | Create user invite
*AccessRecordsApi* | [**create_record**](docs/AccessRecordsApi.md#create_record) | **POST** /v1/records | Create access record
*AccessRecordsApi* | [**create_request**](docs/AccessRecordsApi.md#create_request) | **POST** /v1/requests | Create access request
*AccessRecordsApi* | [**delete_invite**](docs/AccessRecordsApi.md#delete_invite) | **DELETE** /v1/invites/{inviteId} | Delete invite
*AccessRecordsApi* | [**delete_record**](docs/AccessRecordsApi.md#delete_record) | **DELETE** /v1/records/{recordId} | Deletes access record
*AccessRecordsApi* | [**delete_request**](docs/AccessRecordsApi.md#delete_request) | **DELETE** /v1/requests/{requestId} | Deletes access request
*AccessRecordsApi* | [**get_record**](docs/AccessRecordsApi.md#get_record) | **GET** /v1/records/{recordId} | Retrieve access record
*AccessRecordsApi* | [**get_records**](docs/AccessRecordsApi.md#get_records) | **GET** /v1/records | List access records
*AccessRecordsApi* | [**get_request**](docs/AccessRecordsApi.md#get_request) | **GET** /v1/requests/{requestId} | Retrieve access request
*AccessRecordsApi* | [**get_requests**](docs/AccessRecordsApi.md#get_requests) | **GET** /v1/requests | List access requests
*AccessRecordsApi* | [**respond_to_access_request**](docs/AccessRecordsApi.md#respond_to_access_request) | **PATCH** /v1/requests/{requestId} | Approve or deny access request
*AccessRecordsApi* | [**respond_to_invite**](docs/AccessRecordsApi.md#respond_to_invite) | **PATCH** /v1/invites/{inviteId} | Accept invite
*AccessRecordsApi* | [**update_record**](docs/AccessRecordsApi.md#update_record) | **PUT** /v1/records/{recordId} | Update access record
*AccountsApi* | [**delegate_authentication**](docs/AccountsApi.md#delegate_authentication) | **POST** /v1/identities | Link external provider
*AccountsApi* | [**get_account**](docs/AccountsApi.md#get_account) | **GET** /v1/accounts/{accountId} | Retrieve account information
*AccountsApi* | [**get_account_identities**](docs/AccountsApi.md#get_account_identities) | **GET** /v1/identities | List linked external providers
*AccountsApi* | [**get_accounts**](docs/AccountsApi.md#get_accounts) | **GET** /v1/accounts | List user Authress accounts
*ApplicationsApi* | [**delegate_user_login**](docs/ApplicationsApi.md#delegate_user_login) | **POST** /v1/applications/{applicationId}/users/{userId}/delegation | Log user into third-party application
*ConnectionsApi* | [**create_connection**](docs/ConnectionsApi.md#create_connection) | **POST** /v1/connections | Create SSO connection
*ConnectionsApi* | [**delete_connection**](docs/ConnectionsApi.md#delete_connection) | **DELETE** /v1/connections/{connectionId} | Delete SSO connection
*ConnectionsApi* | [**get_connection**](docs/ConnectionsApi.md#get_connection) | **GET** /v1/connections/{connectionId} | Retrieve SSO connection
*ConnectionsApi* | [**get_connection_credentials**](docs/ConnectionsApi.md#get_connection_credentials) | **GET** /v1/connections/{connectionId}/users/{userId}/credentials | Retrieve user connection credentials
*ConnectionsApi* | [**get_connections**](docs/ConnectionsApi.md#get_connections) | **GET** /v1/connections | List SSO connections
*ConnectionsApi* | [**update_connection**](docs/ConnectionsApi.md#update_connection) | **PUT** /v1/connections/{connectionId} | Update SSO connection
*ExtensionsApi* | [**create_extension**](docs/ExtensionsApi.md#create_extension) | **POST** /v1/extensions | Create extension
*ExtensionsApi* | [**delete_extension**](docs/ExtensionsApi.md#delete_extension) | **DELETE** /v1/extensions/{extensionId} | Delete extension
*ExtensionsApi* | [**get_extension**](docs/ExtensionsApi.md#get_extension) | **GET** /v1/extensions/{extensionId} | Retrieve extension
*ExtensionsApi* | [**get_extensions**](docs/ExtensionsApi.md#get_extensions) | **GET** /v1/extensions | List extensions
*ExtensionsApi* | [**login**](docs/ExtensionsApi.md#login) | **GET** / | OAuth Authorize
*ExtensionsApi* | [**request_token**](docs/ExtensionsApi.md#request_token) | **POST** /api/authentication/oauth/tokens | OAuth Token
*ExtensionsApi* | [**update_extension**](docs/ExtensionsApi.md#update_extension) | **PUT** /v1/extensions/{extensionId} | Update extension
*GroupsApi* | [**create_group**](docs/GroupsApi.md#create_group) | **POST** /v1/groups | Create group
*GroupsApi* | [**delete_group**](docs/GroupsApi.md#delete_group) | **DELETE** /v1/groups/{groupId} | Deletes group
*GroupsApi* | [**get_group**](docs/GroupsApi.md#get_group) | **GET** /v1/groups/{groupId} | Retrieve group
*GroupsApi* | [**get_groups**](docs/GroupsApi.md#get_groups) | **GET** /v1/groups | List groups
*GroupsApi* | [**update_group**](docs/GroupsApi.md#update_group) | **PUT** /v1/groups/{groupId} | Update a group
*ResourcePermissionsApi* | [**get_permissioned_resource**](docs/ResourcePermissionsApi.md#get_permissioned_resource) | **GET** /v1/resources/{resourceUri} | Retrieve resource configuration
*ResourcePermissionsApi* | [**get_permissioned_resources**](docs/ResourcePermissionsApi.md#get_permissioned_resources) | **GET** /v1/resources | List all resource configurations
*ResourcePermissionsApi* | [**get_resource_users**](docs/ResourcePermissionsApi.md#get_resource_users) | **GET** /v1/resources/{resourceUri}/users | List users with resource access
*ResourcePermissionsApi* | [**update_permissioned_resource**](docs/ResourcePermissionsApi.md#update_permissioned_resource) | **PUT** /v1/resources/{resourceUri} | Update resource configuration
*RolesApi* | [**create_role**](docs/RolesApi.md#create_role) | **POST** /v1/roles | Create role
*RolesApi* | [**delete_role**](docs/RolesApi.md#delete_role) | **DELETE** /v1/roles/{roleId} | Deletes role
*RolesApi* | [**get_role**](docs/RolesApi.md#get_role) | **GET** /v1/roles/{roleId} | Retrieve role
*RolesApi* | [**get_roles**](docs/RolesApi.md#get_roles) | **GET** /v1/roles | List roles
*RolesApi* | [**update_role**](docs/RolesApi.md#update_role) | **PUT** /v1/roles/{roleId} | Update role
*ServiceClientsApi* | [**create_client**](docs/ServiceClientsApi.md#create_client) | **POST** /v1/clients | Create service client
*ServiceClientsApi* | [**delete_access_key**](docs/ServiceClientsApi.md#delete_access_key) | **DELETE** /v1/clients/{clientId}/access-keys/{keyId} | Delete service client access key
*ServiceClientsApi* | [**delete_client**](docs/ServiceClientsApi.md#delete_client) | **DELETE** /v1/clients/{clientId} | Delete service client
*ServiceClientsApi* | [**get_client**](docs/ServiceClientsApi.md#get_client) | **GET** /v1/clients/{clientId} | Retrieve service client
*ServiceClientsApi* | [**get_clients**](docs/ServiceClientsApi.md#get_clients) | **GET** /v1/clients | List service clients
*ServiceClientsApi* | [**request_access_key**](docs/ServiceClientsApi.md#request_access_key) | **POST** /v1/clients/{clientId}/access-keys | Generate service client access key
*ServiceClientsApi* | [**update_client**](docs/ServiceClientsApi.md#update_client) | **PUT** /v1/clients/{clientId} | Update service client
*TenantsApi* | [**create_tenant**](docs/TenantsApi.md#create_tenant) | **POST** /v1/tenants | Create tenant
*TenantsApi* | [**delete_tenant**](docs/TenantsApi.md#delete_tenant) | **DELETE** /v1/tenants/{tenantId} | Delete tenant
*TenantsApi* | [**get_tenant**](docs/TenantsApi.md#get_tenant) | **GET** /v1/tenants/{tenantId} | Retrieve tenant
*TenantsApi* | [**get_tenants**](docs/TenantsApi.md#get_tenants) | **GET** /v1/tenants | List tenants
*TenantsApi* | [**update_tenant**](docs/TenantsApi.md#update_tenant) | **PUT** /v1/tenants/{tenantId} | Update tenant
*UserPermissionsApi* | [**authorize_user**](docs/UserPermissionsApi.md#authorize_user) | **GET** /v1/users/{userId}/resources/{resourceUri}/permissions/{permission} | Verify user authorization
*UserPermissionsApi* | [**get_user_permissions_for_resource**](docs/UserPermissionsApi.md#get_user_permissions_for_resource) | **GET** /v1/users/{userId}/resources/{resourceUri}/permissions | Get user permissions for resource
*UserPermissionsApi* | [**get_user_resources**](docs/UserPermissionsApi.md#get_user_resources) | **GET** /v1/users/{userId}/resources | List user resources
*UserPermissionsApi* | [**get_user_roles_for_resource**](docs/UserPermissionsApi.md#get_user_roles_for_resource) | **GET** /v1/users/{userId}/resources/{resourceUri}/roles | Get user roles for resource
*UsersApi* | [**delete_user**](docs/UsersApi.md#delete_user) | **DELETE** /v1/users/{userId} | Delete a user
*UsersApi* | [**get_user**](docs/UsersApi.md#get_user) | **GET** /v1/users/{userId} | Retrieve a user
*UsersApi* | [**get_users**](docs/UsersApi.md#get_users) | **GET** /v1/users | List users


## Documentation For Models

 - [AccessRecord](docs/AccessRecord.md)
 - [AccessRecordAccount](docs/AccessRecordAccount.md)
 - [AccessRecordCollection](docs/AccessRecordCollection.md)
 - [AccessRequest](docs/AccessRequest.md)
 - [AccessRequestCollection](docs/AccessRequestCollection.md)
 - [AccessRequestResponse](docs/AccessRequestResponse.md)
 - [AccessTemplate](docs/AccessTemplate.md)
 - [Account](docs/Account.md)
 - [AccountCollection](docs/AccountCollection.md)
 - [AccountLinks](docs/AccountLinks.md)
 - [ApplicationDelegation](docs/ApplicationDelegation.md)
 - [ClaimRequest](docs/ClaimRequest.md)
 - [Client](docs/Client.md)
 - [ClientAccessKey](docs/ClientAccessKey.md)
 - [ClientCollection](docs/ClientCollection.md)
 - [ClientOptions](docs/ClientOptions.md)
 - [CollectionLinks](docs/CollectionLinks.md)
 - [Connection](docs/Connection.md)
 - [ConnectionCollection](docs/ConnectionCollection.md)
 - [ConnectionData](docs/ConnectionData.md)
 - [ConnectionDefaultConnectionProperties](docs/ConnectionDefaultConnectionProperties.md)
 - [Extension](docs/Extension.md)
 - [ExtensionApplication](docs/ExtensionApplication.md)
 - [ExtensionClient](docs/ExtensionClient.md)
 - [ExtensionCollection](docs/ExtensionCollection.md)
 - [Group](docs/Group.md)
 - [GroupCollection](docs/GroupCollection.md)
 - [Identity](docs/Identity.md)
 - [IdentityCollection](docs/IdentityCollection.md)
 - [IdentityRequest](docs/IdentityRequest.md)
 - [Invite](docs/Invite.md)
 - [Link](docs/Link.md)
 - [LinkedGroup](docs/LinkedGroup.md)
 - [Links](docs/Links.md)
 - [MetadataObject](docs/MetadataObject.md)
 - [MetadataObjectAccount](docs/MetadataObjectAccount.md)
 - [OAuthAuthorizeResponse](docs/OAuthAuthorizeResponse.md)
 - [OAuthTokenRequest](docs/OAuthTokenRequest.md)
 - [OAuthTokenResponse](docs/OAuthTokenResponse.md)
 - [Pagination](docs/Pagination.md)
 - [PaginationNext](docs/PaginationNext.md)
 - [PermissionCollection](docs/PermissionCollection.md)
 - [PermissionCollectionAccount](docs/PermissionCollectionAccount.md)
 - [PermissionObject](docs/PermissionObject.md)
 - [PermissionedResource](docs/PermissionedResource.md)
 - [PermissionedResourceCollection](docs/PermissionedResourceCollection.md)
 - [Resource](docs/Resource.md)
 - [ResourcePermission](docs/ResourcePermission.md)
 - [ResourceUsersCollection](docs/ResourceUsersCollection.md)
 - [Role](docs/Role.md)
 - [RoleCollection](docs/RoleCollection.md)
 - [Statement](docs/Statement.md)
 - [Tenant](docs/Tenant.md)
 - [TenantCollection](docs/TenantCollection.md)
 - [TenantConnection](docs/TenantConnection.md)
 - [TenantData](docs/TenantData.md)
 - [TokenRequest](docs/TokenRequest.md)
 - [User](docs/User.md)
 - [UserConnectionCredentials](docs/UserConnectionCredentials.md)
 - [UserIdentity](docs/UserIdentity.md)
 - [UserIdentityCollection](docs/UserIdentityCollection.md)
 - [UserResources](docs/UserResources.md)
 - [UserRole](docs/UserRole.md)
 - [UserRoleCollection](docs/UserRoleCollection.md)
 - [UserToken](docs/UserToken.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```