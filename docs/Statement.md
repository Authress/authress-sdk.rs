# Statement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**roles** | **Vec<String>** |  | 
**resources** | Vec<**[crate::models::Resource](Resource.md)**> |  | 
**users** | Option<Vec<**[crate::models::User](User.md)**>> | The list of users this statement applies to. Users and groups can be set at either the statement level or the record level, but not both. | [optional]
**groups** | Option<Vec<**[crate::models::LinkedGroup](LinkedGroup.md)**>> | The list of groups this statement applies to. Users in these groups will be receive access to the resources listed. Users and groups can be set at either the statement level or the record level, but not both. | [optional]

[[API Models]](./README.md#documentation-for-models) ☆ [[API Endpoints]](./README.md#documentation-for-api-endpoints) ☆ [[Back to Repo]](../README.md)
