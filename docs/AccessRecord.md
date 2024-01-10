# AccessRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**record_id** | Option<**String**> | Unique identifier for the record, can be specified on record creation. | [optional]
**name** | **String** | A helpful name for this record | 
**description** | Option<**String**> | More details about this record | [optional]
**capacity** | Option<**f32**> | Percentage capacity of record that is filled. | [optional][readonly]
**last_updated** | Option<**DateTime**> | The expected last time the record was updated | [optional][readonly]
**status** | Option<**String**> | Current status of the access record. | [optional][readonly]
**users** | Option<Vec<**[crate::models::User](User.md)**>> | The list of users this record applies to | [optional]
**admins** | Option<Vec<**[crate::models::User](User.md)**>> | The list of admin that can edit this record even if they do not have global record edit permissions. | [optional]
**groups** | Option<Vec<**[crate::models::LinkedGroup](LinkedGroup.md)**>> | The list of groups this record applies to. Users in these groups will be receive access to the resources listed. | [optional]
**statements** | Vec<**[crate::models::Statement](Statement.md)**> | A list of statements which match roles to resources. | 
**links** | Option<[**crate::models::Links**](Links.md)> |  |
**tags** | Option<**::std::collections::HashMap<String, String>**> | The tags associated with this resource, this property is an map. { key1: value1, key2: value2 } | [optional]

[[API Models]](./README.md#documentation-for-models) ☆ [[API Endpoints]](./README.md#documentation-for-api-endpoints) ☆ [[Back to Repo]](../README.md)


