# Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_id** | Option<**String**> | Unique identifier for the groupId, can be specified on record creation. | [optional]
**name** | **String** | A helpful name for this record | 
**last_updated** | Option<**DateTime**> | The expected last time the group was updated | [optional][readonly]
**users** | Vec<**[crate::models::User](User.md)**> | The list of users in this group | 
**admins** | Vec<**[crate::models::User](User.md)**> | The list of admins that can edit this record even if they do not have global record edit permissions. | 
**links** | Option<[**crate::models::Links**](Links.md)> |  |
**tags** | Option<**::std::collections::HashMap<String, String>**> | The tags associated with this resource, this property is an map. { key1: value1, key2: value2 } | [optional]

[[API Models]](./README.md#documentation-for-models) ☆ [[API Endpoints]](./README.md#documentation-for-api-endpoints) ☆ [[Back to Repo]](../README.md)


