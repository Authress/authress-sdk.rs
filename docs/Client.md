# Client

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | **String** | The unique ID of the client. | [readonly]
**created_time** | **String** |  | [readonly]
**name** | Option<**String**> | The name of the client | [optional]
**options** | Option<[**crate::models::ClientOptions**](Client_options.md)> |  | [optional]
**verification_keys** | Option<Vec<**[crate::models::ClientAccessKey](ClientAccessKey.md)**>> | A list of the service client access keys. | [optional][readonly]
**tags** | Option<**::std::collections::HashMap<String, String>**> | The tags associated with this resource, this property is an map. { key1: value1, key2: value2 } | [optional]

[[API Models]](./README.md#documentation-for-models) ☆ [[API Endpoints]](./README.md#documentation-for-api-endpoints) ☆ [[Back to Repo]](./README.md)


