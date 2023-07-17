# IdentityRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jwt** | Option<**String**> | A valid JWT OIDC compliant token which will still pass authentication requests to the identity provider. Must contain a unique audience and issuer. | [optional]
**issuer** | Option<**String**> | The issuer of the OAuth OIDC provider's JWTs. This value should match the `iss` claim in the provided tokens exactly. | [optional]
**preferred_audience** | Option<**String**> | If the `jwt` token contains more than one valid audience, then the single audience that should associated with Authress. If more than one audience is preferred, repeat this call with each one. | [optional][default to *]

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](./README.md)


