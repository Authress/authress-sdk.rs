# PermissionObject

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | The action the permission grants, can be scoped using `:` and parent actions imply sub-resource permissions, action:* or action implies action:sub-action. This property is case-insensitive, it will always be cast to lowercase before comparing actions to user permissions. | 
**allow** | **bool** | Does this permission grant the user the ability to execute the action? | 
**grant** | **bool** | Allows the user to give the permission to others without being able to execute the action. | 
**delegate** | **bool** | Allows delegating or granting the permission to others without being able to execute the action. | 

[[Back to Model list]](./README.md#documentation-for-models) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to README]](./README.md)


