[Back to Repository](../README.md)

## Documentation for API Endpoints

All URIs are relative to your [Authress Host URL](https://authress.io/app/#/api).

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccessRecordsApi* | [**create_claim**](./AccessRecordsApi.md#create_claim) | **POST** /v1/claims | Create resource Claim
*AccessRecordsApi* | [**create_invite**](./AccessRecordsApi.md#create_invite) | **POST** /v1/invites | Create user invite
*AccessRecordsApi* | [**create_record**](./AccessRecordsApi.md#create_record) | **POST** /v1/records | Create access record
*AccessRecordsApi* | [**create_request**](./AccessRecordsApi.md#create_request) | **POST** /v1/requests | Create access request
*AccessRecordsApi* | [**delete_invite**](./AccessRecordsApi.md#delete_invite) | **DELETE** /v1/invites/{inviteId} | Delete invite
*AccessRecordsApi* | [**delete_record**](./AccessRecordsApi.md#delete_record) | **DELETE** /v1/records/{recordId} | Deletes access record
*AccessRecordsApi* | [**delete_request**](./AccessRecordsApi.md#delete_request) | **DELETE** /v1/requests/{requestId} | Deletes access request
*AccessRecordsApi* | [**get_record**](./AccessRecordsApi.md#get_record) | **GET** /v1/records/{recordId} | Retrieve access record
*AccessRecordsApi* | [**get_records**](./AccessRecordsApi.md#get_records) | **GET** /v1/records | List access records
*AccessRecordsApi* | [**get_request**](./AccessRecordsApi.md#get_request) | **GET** /v1/requests/{requestId} | Retrieve access request
*AccessRecordsApi* | [**get_requests**](./AccessRecordsApi.md#get_requests) | **GET** /v1/requests | List access requests
*AccessRecordsApi* | [**respond_to_access_request**](./AccessRecordsApi.md#respond_to_access_request) | **PATCH** /v1/requests/{requestId} | Approve or deny access request
*AccessRecordsApi* | [**respond_to_invite**](./AccessRecordsApi.md#respond_to_invite) | **PATCH** /v1/invites/{inviteId} | Accept invite
*AccessRecordsApi* | [**update_record**](./AccessRecordsApi.md#update_record) | **PUT** /v1/records/{recordId} | Update access record
*AccountsApi* | [**delegate_authentication**](./AccountsApi.md#delegate_authentication) | **POST** /v1/identities | Link external provider
*AccountsApi* | [**get_account**](./AccountsApi.md#get_account) | **GET** /v1/accounts/{accountId} | Retrieve account information
*AccountsApi* | [**get_account_identities**](./AccountsApi.md#get_account_identities) | **GET** /v1/identities | List linked external providers
*AccountsApi* | [**get_accounts**](./AccountsApi.md#get_accounts) | **GET** /v1/accounts | List user Authress accounts
*ApplicationsApi* | [**delegate_user_login**](./ApplicationsApi.md#delegate_user_login) | **POST** /v1/applications/{applicationId}/users/{userId}/delegation | Log user into third-party application
*ConnectionsApi* | [**create_connection**](./ConnectionsApi.md#create_connection) | **POST** /v1/connections | Create SSO connection
*ConnectionsApi* | [**delete_connection**](./ConnectionsApi.md#delete_connection) | **DELETE** /v1/connections/{connectionId} | Delete SSO connection
*ConnectionsApi* | [**get_connection**](./ConnectionsApi.md#get_connection) | **GET** /v1/connections/{connectionId} | Retrieve SSO connection
*ConnectionsApi* | [**get_connection_credentials**](./ConnectionsApi.md#get_connection_credentials) | **GET** /v1/connections/{connectionId}/users/{userId}/credentials | Retrieve user connection credentials
*ConnectionsApi* | [**get_connections**](./ConnectionsApi.md#get_connections) | **GET** /v1/connections | List SSO connections
*ConnectionsApi* | [**update_connection**](./ConnectionsApi.md#update_connection) | **PUT** /v1/connections/{connectionId} | Update SSO connection
*ExtensionsApi* | [**create_extension**](./ExtensionsApi.md#create_extension) | **POST** /v1/extensions | Create extension
*ExtensionsApi* | [**delete_extension**](./ExtensionsApi.md#delete_extension) | **DELETE** /v1/extensions/{extensionId} | Delete extension
*ExtensionsApi* | [**get_extension**](./ExtensionsApi.md#get_extension) | **GET** /v1/extensions/{extensionId} | Retrieve extension
*ExtensionsApi* | [**get_extensions**](./ExtensionsApi.md#get_extensions) | **GET** /v1/extensions | List extensions
*ExtensionsApi* | [**login**](./ExtensionsApi.md#login) | **GET** / | OAuth Authorize
*ExtensionsApi* | [**request_token**](./ExtensionsApi.md#request_token) | **POST** /api/authentication/oauth/tokens | OAuth Token
*ExtensionsApi* | [**update_extension**](./ExtensionsApi.md#update_extension) | **PUT** /v1/extensions/{extensionId} | Update extension
*GroupsApi* | [**create_group**](./GroupsApi.md#create_group) | **POST** /v1/groups | Create group
*GroupsApi* | [**delete_group**](./GroupsApi.md#delete_group) | **DELETE** /v1/groups/{groupId} | Deletes group
*GroupsApi* | [**get_group**](./GroupsApi.md#get_group) | **GET** /v1/groups/{groupId} | Retrieve group
*GroupsApi* | [**get_groups**](./GroupsApi.md#get_groups) | **GET** /v1/groups | List groups
*GroupsApi* | [**update_group**](./GroupsApi.md#update_group) | **PUT** /v1/groups/{groupId} | Update a group
*ResourcePermissionsApi* | [**get_permissioned_resource**](./ResourcePermissionsApi.md#get_permissioned_resource) | **GET** /v1/resources/{resourceUri} | Retrieve resource configuration
*ResourcePermissionsApi* | [**get_permissioned_resources**](./ResourcePermissionsApi.md#get_permissioned_resources) | **GET** /v1/resources | List all resource configurations
*ResourcePermissionsApi* | [**get_resource_users**](./ResourcePermissionsApi.md#get_resource_users) | **GET** /v1/resources/{resourceUri}/users | List users with resource access
*ResourcePermissionsApi* | [**update_permissioned_resource**](./ResourcePermissionsApi.md#update_permissioned_resource) | **PUT** /v1/resources/{resourceUri} | Update resource configuration
*RolesApi* | [**create_role**](./RolesApi.md#create_role) | **POST** /v1/roles | Create role
*RolesApi* | [**delete_role**](./RolesApi.md#delete_role) | **DELETE** /v1/roles/{roleId} | Deletes role
*RolesApi* | [**get_role**](./RolesApi.md#get_role) | **GET** /v1/roles/{roleId} | Retrieve role
*RolesApi* | [**get_roles**](./RolesApi.md#get_roles) | **GET** /v1/roles | List roles
*RolesApi* | [**update_role**](./RolesApi.md#update_role) | **PUT** /v1/roles/{roleId} | Update role
*ServiceClientsApi* | [**create_client**](./ServiceClientsApi.md#create_client) | **POST** /v1/clients | Create service client
*ServiceClientsApi* | [**delete_access_key**](./ServiceClientsApi.md#delete_access_key) | **DELETE** /v1/clients/{clientId}/access-keys/{keyId} | Delete service client access key
*ServiceClientsApi* | [**delete_client**](./ServiceClientsApi.md#delete_client) | **DELETE** /v1/clients/{clientId} | Delete service client
*ServiceClientsApi* | [**get_client**](./ServiceClientsApi.md#get_client) | **GET** /v1/clients/{clientId} | Retrieve service client
*ServiceClientsApi* | [**get_clients**](./ServiceClientsApi.md#get_clients) | **GET** /v1/clients | List service clients
*ServiceClientsApi* | [**request_access_key**](./ServiceClientsApi.md#request_access_key) | **POST** /v1/clients/{clientId}/access-keys | Generate service client access key
*ServiceClientsApi* | [**update_client**](./ServiceClientsApi.md#update_client) | **PUT** /v1/clients/{clientId} | Update service client
*TenantsApi* | [**create_tenant**](./TenantsApi.md#create_tenant) | **POST** /v1/tenants | Create tenant
*TenantsApi* | [**delete_tenant**](./TenantsApi.md#delete_tenant) | **DELETE** /v1/tenants/{tenantId} | Delete tenant
*TenantsApi* | [**get_tenant**](./TenantsApi.md#get_tenant) | **GET** /v1/tenants/{tenantId} | Retrieve tenant
*TenantsApi* | [**get_tenants**](./TenantsApi.md#get_tenants) | **GET** /v1/tenants | List tenants
*TenantsApi* | [**update_tenant**](./TenantsApi.md#update_tenant) | **PUT** /v1/tenants/{tenantId} | Update tenant
*UserPermissionsApi* | [**authorize_user**](./UserPermissionsApi.md#authorize_user) | **GET** /v1/users/{userId}/resources/{resourceUri}/permissions/{permission} | Verify user authorization
*UserPermissionsApi* | [**get_user_permissions_for_resource**](./UserPermissionsApi.md#get_user_permissions_for_resource) | **GET** /v1/users/{userId}/resources/{resourceUri}/permissions | Get user permissions for resource
*UserPermissionsApi* | [**get_user_resources**](./UserPermissionsApi.md#get_user_resources) | **GET** /v1/users/{userId}/resources | List user resources
*UserPermissionsApi* | [**get_user_roles_for_resource**](./UserPermissionsApi.md#get_user_roles_for_resource) | **GET** /v1/users/{userId}/resources/{resourceUri}/roles | Get user roles for resource
*UsersApi* | [**delete_user**](./UsersApi.md#delete_user) | **DELETE** /v1/users/{userId} | Delete a user
*UsersApi* | [**get_user**](./UsersApi.md#get_user) | **GET** /v1/users/{userId} | Retrieve a user
*UsersApi* | [**get_users**](./UsersApi.md#get_users) | **GET** /v1/users | List users


## Documentation For Models

 - [AccessRecord](./AccessRecord.md)
 - [AccessRecordCollection](./AccessRecordCollection.md)
 - [AccessRequest](./AccessRequest.md)
 - [AccessRequestCollection](./AccessRequestCollection.md)
 - [AccessRequestResponse](./AccessRequestResponse.md)
 - [AccessTemplate](./AccessTemplate.md)
 - [Account](./Account.md)
 - [AccountCollection](./AccountCollection.md)
 - [Links](./Links.md)
 - [ApplicationDelegation](./ApplicationDelegation.md)
 - [ClaimRequest](./ClaimRequest.md)
 - [Client](./Client.md)
 - [ClientAccessKey](./ClientAccessKey.md)
 - [ClientCollection](./ClientCollection.md)
 - [ClientOptions](./ClientOptions.md)
 - [CollectionLinks](./CollectionLinks.md)
 - [Connection](./Connection.md)
 - [ConnectionCollection](./ConnectionCollection.md)
 - [ConnectionData](./ConnectionData.md)
 - [ConnectionDefaultConnectionProperties](./ConnectionDefaultConnectionProperties.md)
 - [Extension](./Extension.md)
 - [ExtensionApplication](./ExtensionApplication.md)
 - [ExtensionClient](./ExtensionClient.md)
 - [ExtensionCollection](./ExtensionCollection.md)
 - [Group](./Group.md)
 - [GroupCollection](./GroupCollection.md)
 - [Identity](./Identity.md)
 - [IdentityCollection](./IdentityCollection.md)
 - [IdentityRequest](./IdentityRequest.md)
 - [Invite](./Invite.md)
 - [Link](./Link.md)
 - [LinkedGroup](./LinkedGroup.md)
 - [MetadataObject](./MetadataObject.md)
 - [MetadataObjectAccount](./MetadataObjectAccount.md)
 - [OAuthAuthorizeResponse](./OAuthAuthorizeResponse.md)
 - [OAuthTokenRequest](./OAuthTokenRequest.md)
 - [OAuthTokenResponse](./OAuthTokenResponse.md)
 - [Pagination](./Pagination.md)
 - [PaginationCursor](./PaginationCursor.md)
 - [PermissionCollection](./PermissionCollection.md)
 - [PermissionObject](./PermissionObject.md)
 - [PermissionedResource](./PermissionedResource.md)
 - [PermissionedResourceCollection](./PermissionedResourceCollection.md)
 - [Resource](./Resource.md)
 - [ResourcePermission](./ResourcePermission.md)
 - [ResourceUsersCollection](./ResourceUsersCollection.md)
 - [Role](./Role.md)
 - [RoleCollection](./RoleCollection.md)
 - [Statement](./Statement.md)
 - [Tenant](./Tenant.md)
 - [TenantCollection](./TenantCollection.md)
 - [TenantConnection](./TenantConnection.md)
 - [TenantData](./TenantData.md)
 - [TokenRequest](./TokenRequest.md)
 - [User](./User.md)
 - [UserConnectionCredentials](./UserConnectionCredentials.md)
 - [UserIdentity](./UserIdentity.md)
 - [UserIdentityCollection](./UserIdentityCollection.md)
 - [UserResources](./UserResources.md)
 - [UserRole](./UserRole.md)
 - [UserRoleCollection](./UserRoleCollection.md)
 - [UserToken](./UserToken.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```