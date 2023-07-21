# \RolesApi
Method | HTTP request | Description
------------- | ------------- | -------------
[**create_role**](RolesApi.md#create_role) | **POST** /v1/roles | Create role
[**delete_role**](RolesApi.md#delete_role) | **DELETE** /v1/roles/{roleId} | Deletes role
[**get_role**](RolesApi.md#get_role) | **GET** /v1/roles/{roleId} | Retrieve role
[**get_roles**](RolesApi.md#get_roles) | **GET** /v1/roles | List roles
[**update_role**](RolesApi.md#update_role) | **PUT** /v1/roles/{roleId} | Update role



## create_role

> crate::models::Role create_role(role)
Create role

Creates a role with permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## delete_role

> delete_role(role_id)
Deletes role

Remove a role. If a record references the role, that record will not be modified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier of the role. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_role

> crate::models::Role get_role(role_id)
Retrieve role

Roles contain a list of permissions that will be applied to any user or resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier of the role. | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_roles

> crate::models::RoleCollection get_roles()
List roles

Get all the account roles. Roles contain a list of permissions that will be applied to any user or resource

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RoleCollection**](RoleCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## update_role

> crate::models::Role update_role(role_id, role)
Update role

Updates a role adding or removing permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier of the role. | [required] |
**role** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

