# OAuthTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | The client identifier to constrain the token to. | 
**client_secret** | Option<**String**> | The secret associated with the client that authorizes the generation of token it's behalf. (Either the `client_secret` or the `code_verifier` is required) | [optional]
**code_verifier** | Option<**String**> | The code verifier is the the value used in the generation of the OAuth authorization request `code_challenge` property. (Either the `client_secret` or the `code_verifier` is required) | [optional]
**grant_type** | Option<**String**> | A suggestion to the token generation which type of credentials are being provided. | [optional]
**username** | Option<**String**> | When using the user password grant_type, specify the username. Authress recommends this should always be an email address. | [optional]
**password** | Option<**String**> | When using the user password grant_type, specify the user's password. | [optional]
**r#type** | Option<**String**> | Enables additional configuration of the grant_type. In the case of user password grant_type, set this to **signup**, to enable the creation of users. Set this to **update**, force update the password associated with the user. | [optional]

[[API Models]](./README.md#documentation-for-models) ☆ [[API Endpoints]](./README.md#documentation-for-api-endpoints) ☆ [[Back to Repo]](./README.md)


