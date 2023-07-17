# ClientAccessKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key_id** | Option<**String**> | The unique ID of the client. | [optional][readonly]
**client_id** | **String** | The unique ID of the client. | [readonly]
**generation_date** | Option<**String**> |  | [optional][readonly]
**client_secret** | Option<**String**> | The unencoded OAuth client secret used with the OAuth endpoints to request a JWT using the `client_credentials` grant type. Pass the clientId and the clientSecret to the documented /tokens endpoint. | [optional][readonly]
**access_key** | Option<**String**> | An encoded access key which contains identifying information for client access token creation. For direct use with the Authress SDKs. This private access key must be saved on first creation as it is discarded afterwards. Authress only saves the corresponding public key to verify the private access key. | [optional][readonly]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](./README.md)


