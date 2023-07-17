# \ConnectionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_connection**](ConnectionsApi.md#create_connection) | **POST** /v1/connections | Create SSO connection
[**delete_connection**](ConnectionsApi.md#delete_connection) | **DELETE** /v1/connections/{connectionId} | Delete SSO connection
[**get_connection**](ConnectionsApi.md#get_connection) | **GET** /v1/connections/{connectionId} | Retrieve SSO connection
[**get_connection_credentials**](ConnectionsApi.md#get_connection_credentials) | **GET** /v1/connections/{connectionId}/users/{userId}/credentials | Retrieve user connection credentials
[**get_connections**](ConnectionsApi.md#get_connections) | **GET** /v1/connections | List SSO connections
[**update_connection**](ConnectionsApi.md#update_connection) | **PUT** /v1/connections/{connectionId} | Update SSO connection



## create_connection

> crate::models::Connection create_connection(connection)
Create SSO connection

Specify identity connection details for Authress identity aggregation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection** | [**Connection**](Connection.md) |  | [required] |

### Return type

[**crate::models::Connection**](Connection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## delete_connection

> delete_connection(connection_id)
Delete SSO connection

Delete an identity connection details for Authress identity aggregation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The connection identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_connection

> crate::models::Connection get_connection(connection_id)
Retrieve SSO connection

Get the identity connection details for Authress identity aggregation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The connection identifier. | [required] |

### Return type

[**crate::models::Connection**](Connection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_connection_credentials

> crate::models::UserConnectionCredentials get_connection_credentials(connection_id, user_id)
Retrieve user connection credentials

Get the credentials for the user that were generated as part of the latest user login flow. Returns an access token that can be used with originating connection provider, based on the original scopes and approved permissions by that service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The connection identifier. | [required] |
**user_id** | **String** | The connection user. | [required] |

### Return type

[**crate::models::UserConnectionCredentials**](UserConnectionCredentials.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_connections

> crate::models::ConnectionCollection get_connections()
List SSO connections

Returns a paginated connection list for the account. Only connections the user has access to are returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ConnectionCollection**](ConnectionCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## update_connection

> crate::models::Connection update_connection(connection_id, connection)
Update SSO connection

Specify identity connection details for Authress identity aggregation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connection_id** | **String** | The connection identifier. | [required] |
**connection** | [**Connection**](Connection.md) |  | [required] |

### Return type

[**crate::models::Connection**](Connection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

