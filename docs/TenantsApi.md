# \TenantsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_tenant**](TenantsApi.md#create_tenant) | **POST** /v1/tenants | Create tenant
[**delete_tenant**](TenantsApi.md#delete_tenant) | **DELETE** /v1/tenants/{tenantId} | Delete tenant
[**get_tenant**](TenantsApi.md#get_tenant) | **GET** /v1/tenants/{tenantId} | Retrieve tenant
[**get_tenants**](TenantsApi.md#get_tenants) | **GET** /v1/tenants | List tenants
[**update_tenant**](TenantsApi.md#update_tenant) | **PUT** /v1/tenants/{tenantId} | Update tenant



## create_tenant

> crate::models::Tenant create_tenant(tenant)
Create tenant

Specify tenant identity details for tenant tracking, separation, and user management. Tenant identifiers are persisted to access tokens created by Authress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | [**Tenant**](Tenant.md) |  | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## delete_tenant

> delete_tenant(tenant_id)
Delete tenant

Delete a tenant. If a connection was created for the tenant then it is deleted as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The tenantId. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_tenant

> crate::models::Tenant get_tenant(tenant_id)
Retrieve tenant

Get the tenant details for an Authress tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The tenantId. | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_tenants

> crate::models::TenantCollection get_tenants()
List tenants

Returns a paginated tenants list for the account. Only tenants the user has access to are returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TenantCollection**](TenantCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## update_tenant

> crate::models::Tenant update_tenant(tenant_id, tenant)
Update tenant

Updates the tenants information and linked tenant if it exists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | The tenantId. | [required] |
**tenant** | [**Tenant**](Tenant.md) |  | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)
