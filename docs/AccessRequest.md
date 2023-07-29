# AccessRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | **String** | Unique identifier for the request. | [readonly]
**last_updated** | Option<**String**> | The expected last time the request was updated | [optional][readonly]
**status** | Option<**String**> | Current status of the access request. | [optional][readonly]
**access** | [**crate::models::AccessTemplate**](AccessTemplate.md) |  | 
**links** | Option<[**crate::models::Links**](Links.md)> |  |
**tags** | Option<**::std::collections::HashMap<String, String>**> | The tags associated with this resource, this property is an map. { key1: value1, key2: value2 } | [optional]

[[API Models]](./README.md#documentation-for-models) ☆ [[API Endpoints]](./README.md#documentation-for-api-endpoints) ☆ [[Back to Repo]](./README.md)


