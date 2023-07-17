# \ServiceClientsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client**](ServiceClientsApi.md#create_client) | **POST** /v1/clients | Create service client
[**delete_access_key**](ServiceClientsApi.md#delete_access_key) | **DELETE** /v1/clients/{clientId}/access-keys/{keyId} | Delete service client access key
[**delete_client**](ServiceClientsApi.md#delete_client) | **DELETE** /v1/clients/{clientId} | Delete service client
[**get_client**](ServiceClientsApi.md#get_client) | **GET** /v1/clients/{clientId} | Retrieve service client
[**get_clients**](ServiceClientsApi.md#get_clients) | **GET** /v1/clients | List service clients
[**request_access_key**](ServiceClientsApi.md#request_access_key) | **POST** /v1/clients/{clientId}/access-keys | Generate service client access key
[**update_client**](ServiceClientsApi.md#update_client) | **PUT** /v1/clients/{clientId} | Update service client



## create_client

> crate::models::Client create_client(client)
Create service client

Creates a service client to interact with Authress or any other service on behalf of users. Each client has secret private keys used to authenticate with Authress. To use service clients created through other mechanisms, skip creating a client and create access records with the client identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client** | [**Client**](Client.md) |  | [required] |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## delete_access_key

> delete_access_key(client_id, key_id)
Delete service client access key

Deletes an access key for a client prevent it from being used to authenticate with Authress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The unique identifier of the client. | [required] |
**key_id** | **String** | The ID of the access key to remove from the client. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## delete_client

> delete_client(client_id)
Delete service client

This deletes the service client.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The unique identifier for the client. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_client

> crate::models::Client get_client(client_id)
Retrieve service client

Returns all information related to client except for the private access keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The unique identifier for the client. | [required] |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_clients

> crate::models::ClientCollection get_clients(limit, cursor)
List service clients

Returns all clients that the user has access to in the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Max number of results to return |  |[default to 20]
**cursor** | Option<**String**> | Continuation cursor for paging. |  |

### Return type

[**crate::models::ClientCollection**](ClientCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## request_access_key

> crate::models::ClientAccessKey request_access_key(client_id)
Generate service client access key

Create a new access key for the client so that a service can authenticate with Authress as that client. Using the client will allow delegation of permission checking of users. (Limited to 5 Active keys per client)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The unique identifier of the client. | [required] |

### Return type

[**crate::models::ClientAccessKey**](ClientAccessKey.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## update_client

> crate::models::Client update_client(client_id, client)
Update service client

Updates a client information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The unique identifier for the client. | [required] |
**client** | [**Client**](Client.md) |  | [required] |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

