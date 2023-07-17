# \UserPermissionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authorize_user**](UserPermissionsApi.md#authorize_user) | **GET** /v1/users/{userId}/resources/{resourceUri}/permissions/{permission} | Verify user authorization
[**get_user_permissions_for_resource**](UserPermissionsApi.md#get_user_permissions_for_resource) | **GET** /v1/users/{userId}/resources/{resourceUri}/permissions | Get user permissions for resource
[**get_user_resources**](UserPermissionsApi.md#get_user_resources) | **GET** /v1/users/{userId}/resources | List user resources
[**get_user_roles_for_resource**](UserPermissionsApi.md#get_user_roles_for_resource) | **GET** /v1/users/{userId}/resources/{resourceUri}/roles | Get user roles for resource



## authorize_user

> authorize_user(user_id, resource_uri, permission)
Verify user authorization

Performs the user authorization check. Does the user have the specified permission to the resource?

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**UserId**](.md) | The user to check permissions on | [required] |
**resource_uri** | **String** | The uri path of a resource to validate, must be URL encoded, uri segments are allowed, the resource must be a full path. | [required] |
**permission** | [**Action**](.md) | Permission to check, '*' and scoped permissions can also be checked here. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_user_permissions_for_resource

> crate::models::PermissionCollection get_user_permissions_for_resource(user_id, resource_uri)
Get user permissions for resource

Get a summary of the permissions a user has to a particular resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**UserId**](.md) | The user to check permissions on | [required] |
**resource_uri** | **String** | The uri path of a resource to validate, must be URL encoded, uri segments are allowed. | [required] |

### Return type

[**crate::models::PermissionCollection**](PermissionCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_user_resources

> crate::models::UserResources get_user_resources(user_id, resource_uri, collection_configuration, permissions, limit, cursor)
List user resources

Get the users resources. This result is a list of resource uris that a user has an permission to. By default only the top level matching resources are returned. To get a user's list of deeply nested resources, set the `collectionConfiguration` to be `INCLUDE_NESTED`. This collection is paginated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**UserId**](.md) | The user to check permissions on | [required] |
**resource_uri** | Option<**String**> | The top level uri path of a resource to query for. Will only match explicit or nested sub-resources. Will not partial match resource names. |  |
**collection_configuration** | Option<**String**> | `TOP_LEVEL_ONLY` - returns only directly nested resources under the resourceUri. A query to `resourceUri=Collection` will return `Collection/resource_1`.<br>`INCLUDE_NESTED` - will return all sub-resources as well as deeply nested resources that the user has the specified permission to. A query to `resourceUri=Collection` will return `Collection/namespaces/ns/resources/resource_1`.<br><br>To return matching resources for nested resources, set this parameter to `INCLUDE_NESTED`. |  |[default to TOP_LEVEL_ONLY]
**permissions** | Option<[**Action**](.md)> | Permission to check, '*' and scoped permissions can also be checked here. By default if the user has any permission explicitly to a resource, it will be included in the list. |  |
**limit** | Option<**i32**> | Max number of results to return |  |[default to 20]
**cursor** | Option<**String**> | Continuation cursor for paging |  |

### Return type

[**crate::models::UserResources**](UserResources.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_user_roles_for_resource

> crate::models::UserRoleCollection get_user_roles_for_resource(user_id, resource_uri)
Get user roles for resource

Get a summary of the roles a user has to a particular resource. Users can be assigned roles from multiple access records, this may cause the same role to appear in the list more than once.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**UserId**](.md) | The user to get roles for. | [required] |
**resource_uri** | **String** | The uri path of a resource to get roles for, must be URL encoded. Checks for explicit resource roles, roles attached to parent resources are not returned. | [required] |

### Return type

[**crate::models::UserRoleCollection**](UserRoleCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

