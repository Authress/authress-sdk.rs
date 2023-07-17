# \AccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delegate_authentication**](AccountsApi.md#delegate_authentication) | **POST** /v1/identities | Link external provider
[**get_account**](AccountsApi.md#get_account) | **GET** /v1/accounts/{accountId} | Retrieve account information
[**get_account_identities**](AccountsApi.md#get_account_identities) | **GET** /v1/identities | List linked external providers
[**get_accounts**](AccountsApi.md#get_accounts) | **GET** /v1/accounts | List user Authress accounts



## delegate_authentication

> delegate_authentication(identity_request)
Link external provider

An identity is a JWT subscriber *sub* and issuer *iss*. Only one account my be linked to a particular JWT combination. Allows calling the API with a federated token directly instead of using a client access key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_request** | Option<[**IdentityRequest**](IdentityRequest.md)> |  | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_account

> crate::models::Account get_account(account_id)
Retrieve account information

Includes the original configuration information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The unique identifier for the account | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_account_identities

> crate::models::IdentityCollection get_account_identities()
List linked external providers

Returns a list of identities linked for this account.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IdentityCollection**](IdentityCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_accounts

> crate::models::AccountCollection get_accounts(earliest_cache_time)
List user Authress accounts

Returns a list of accounts that the user has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**earliest_cache_time** | Option<**String**> | Ensure the accounts list is not cached before this time. |  |

### Return type

[**crate::models::AccountCollection**](AccountCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

