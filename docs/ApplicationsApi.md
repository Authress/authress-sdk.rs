# \ApplicationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delegate_user_login**](ApplicationsApi.md#delegate_user_login) | **POST** /v1/applications/{applicationId}/users/{userId}/delegation | Log user into third-party application



## delegate_user_login

> crate::models::ApplicationDelegation delegate_user_login(application_id, user_id)
Log user into third-party application

Redirect the user to an external application to login them in. Authress uses OpenID and SAML configuration to securely pass the user's login session in your platform to an external platform. The user will then be logged into that platform.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_id** | **String** | The application to have the user log into. | [required] |
**user_id** | [**UserId**](.md) | The user. | [required] |

### Return type

[**crate::models::ApplicationDelegation**](ApplicationDelegation.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

