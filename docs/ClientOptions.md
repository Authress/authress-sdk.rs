# ClientOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**grant_user_permissions_access** | Option<**bool**> | Grant the client access to verify authorization on behalf of any user. | [optional][default to false]
**grant_token_generation** | Option<**bool**> | Grant the client access to generate oauth tokens on behalf of the Authress account. **Security Warning**: This means that this client can impersonate any user, and should only be used when connecting an existing custom Authorization Server to Authress, when that server does not support a standard OAuth connection. | [optional][default to false]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](./README.md)


