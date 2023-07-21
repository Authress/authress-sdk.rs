# ResourcePermissions
Method | HTTP request | Description
------------- | ------------- | -------------
[**get_permissioned_resource**](#get_permissioned_resource) | **GET** /v1/resources/{resourceUri} | Retrieve resource configuration
[**get_permissioned_resources**](#get_permissioned_resources) | **GET** /v1/resources | List all resource configurations
[**get_resource_users**](#get_resource_users) | **GET** /v1/resources/{resourceUri}/users | List users with resource access
[**update_permissioned_resource**](#update_permissioned_resource) | **PUT** /v1/resources/{resourceUri} | Update resource configuration



## get_permissioned_resource

> crate::models::PermissionedResource get_permissioned_resource(resource_uri)
Retrieve resource configuration

Permissions can be set globally at a resource level. This will apply to all users in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_uri** | **String** | The uri path of a resource to validate, must be URL encoded, uri segments are allowed. | [required] |

### Return type

[**crate::models::PermissionedResource**](PermissionedResource.md)


[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_permissioned_resources

> crate::models::PermissionedResourceCollection get_permissioned_resources()
List all resource configurations

Permissions can be set globally at a resource level. Lists any resources with a globally set resource policy.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PermissionedResourceCollection**](PermissionedResourceCollection.md)


[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_resource_users

> crate::models::ResourceUsersCollection get_resource_users(resource_uri, limit, cursor)
List users with resource access

Get the resource users with explicit access to the resource. This result is a list of users that have some permission to the resource. Users with access to parent resources and users with access only to a sub-resource will not be returned in this result. In the case that the resource has multiple users, the list will be paginated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_uri** | **String** | The uri path of a resource to validate, must be URL encoded, uri segments are allowed. | [required] |
**limit** | Option<**u32**> | Max number of results to return |  |[default to 20]
**cursor** | Option<**String**> | Continuation cursor for paging |  |

### Return type

[**crate::models::ResourceUsersCollection**](ResourceUsersCollection.md)


[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## update_permissioned_resource

> update_permissioned_resource(resource_uri, permissioned_resource)
Update resource configuration

Updates the global permissions on a resource. This applies to all users in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_uri** | **String** | The uri path of a resource to validate, must be URL encoded, uri segments are allowed. | [required] |
**permissioned_resource** | [**PermissionedResource**](PermissionedResource.md) | The contents of the permission to set on the resource. Overwrites existing data. | [required] |

### Return type

 (empty response body)

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

