# \ExtensionsApi
Method | HTTP request | Description
------------- | ------------- | -------------
[**create_extension**](ExtensionsApi.md#create_extension) | **POST** /v1/extensions | Create extension
[**delete_extension**](ExtensionsApi.md#delete_extension) | **DELETE** /v1/extensions/{extensionId} | Delete extension
[**get_extension**](ExtensionsApi.md#get_extension) | **GET** /v1/extensions/{extensionId} | Retrieve extension
[**get_extensions**](ExtensionsApi.md#get_extensions) | **GET** /v1/extensions | List extensions
[**login**](ExtensionsApi.md#login) | **GET** / | OAuth Authorize
[**request_token**](ExtensionsApi.md#request_token) | **POST** /api/authentication/oauth/tokens | OAuth Token
[**update_extension**](ExtensionsApi.md#update_extension) | **PUT** /v1/extensions/{extensionId} | Update extension



## create_extension

> crate::models::Extension create_extension(extension)
Create extension

Specify the extension details for a new developer extension. Creating the extension enables developers to build applications that can log in to your platform and interact with your users' data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension** | [**Extension**](Extension.md) |  | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## delete_extension

> delete_extension(extension_id)
Delete extension

Deletes the specified extension. When deleted an extension can no longer be accessed. Additionally users cannot use that extension to log in, nor can the service client associated with the extension be used to access data secured by Authress. The related Access Records will automatically be deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_id** | **String** | The extension identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_extension

> crate::models::Extension get_extension(extension_id)
Retrieve extension

Gets the platform extension details for the existing extension.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_id** | **String** | The extension identifier. | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_extensions

> crate::models::ExtensionCollection get_extensions()
List extensions

Lists the platform extensions. Extensions are the applications that developers of your platform have created for your users to interact with. Returns a paginated extension list for the account. Only extensions the user has access to are returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ExtensionCollection**](ExtensionCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## login

> crate::models::OAuthAuthorizeResponse login(client_id, code_challenge, redirect_uri, code_challenge_method)
OAuth Authorize

*Note*: This endpoint is only to be used for [Authress Platform Extensions](https://authress.io/knowledge-base/docs/extensions/). If you are not building an app marketplace, then tokens can be directly requested for Service Clients, using the relevant [SDK](https://authress.io/app/#/api).<br><br>Start the OAuth login by redirecting the user to the OAuth Authorize endpoint. This generates a JWT for the user using the configured application, client ID, and connection.<br><br>The OAuth 2.1 authorization request that follows [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749). Enables users to request a JWT signed by Authress and Returns an OAuth JWT containing the relevant user claims. Tokens generated must be verified before usage by validating the `sub`, `iss`, and `aud` properties in the JWT. Please note, that the properties in the request and response use snake_case to explicitly follow the standard.<br><br>The ExtensionClient in the [@authress/login](https://github.com/Authress/authress-login.js#platform-extension-login) npm package provides all the necessary logic to make this easy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client identifier to constrain the token to. | [required] |
**code_challenge** | **String** | The PKCE Code challenge generated by the extension UI to secure the code exchange from [RFC 7636](https://datatracker.ietf.org/doc/html/rfc7636). | [required] |
**redirect_uri** | **String** | The location to redirect the user back to after login. This redirect_uri must be a URL that matches one of the preconfigured urls in the Authress Application. | [required] |
**code_challenge_method** | Option<**String**> | The method used to generate the code_challenge from the code_verifier. `code_challenge_method(code_verifier) = code_challenge` |  |[default to S256]

### Return type

[**crate::models::OAuthAuthorizeResponse**](OAuthAuthorizeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## request_token

> crate::models::OAuthTokenResponse request_token(oauth_token_request)
OAuth Token

*Note*: This endpoint is only to be used for [Authress Platform Extensions](https://authress.io/knowledge-base/docs/extensions/). If you are not building an app marketplace, then tokens can be directly requested for Service Clients, using the relevant [SDK](https://authress.io/app/#/api).<br><br>Request an OAuth JWT. Can either be called with service client credentials or as the second part of the user authorize login flow.<br>When using the `password` grant_type, service client authentication must be used via the Authress SDKs, and requires the `Authress:AuthenticateUser` role.<br><br>The OAuth 2.1 token request that follows [RFC 6749](https://www.rfc-editor.org/rfc/rfc6749). Enables users to request a JWT signed by Authress, and returns an OAuth JWT containing the relevant user claims. Tokens generated must be verified before usage by validating the `sub`, `iss`, and `aud` properties in the JWT. Please note, that the properties in the request and response use snake_case to explicitly follow the standard.<br><br>The ExtensionClient in the [@authress/login](https://github.com/Authress/authress-login.js#platform-extension-login) npm package provides all the necessary logic to make this easy.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**oauth_token_request** | [**OAuthTokenRequest**](OAuthTokenRequest.md) | The contents of an OAuth token request. | [required] |

### Return type

[**crate::models::OAuthTokenResponse**](OAuthTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## update_extension

> crate::models::Extension update_extension(extension_id, extension)
Update extension

Specify the updated extension. The extension will be updated and these changes will be reflected to the access control and user authentication associated with the extension.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**extension_id** | **String** | The extension identifier. | [required] |
**extension** | [**Extension**](Extension.md) |  | [required] |

### Return type

[**crate::models::Extension**](Extension.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

