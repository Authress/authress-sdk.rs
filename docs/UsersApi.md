# Users
Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user**](#delete_user) | **DELETE** /v1/users/{userId} | Delete a user
[**get_user**](#get_user) | **GET** /v1/users/{userId} | Retrieve a user
[**get_users**](#get_users) | **GET** /v1/users | List users



## delete_user

> delete_user(user_id)
Delete a user

Removes the user, all access the user has been granted through Authress access records, and any related user data. This action is completed asynchronously.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**UserId**](.md) | The user identifier. | [required] |

### Return type

 (empty response body)


---


## get_user

> crate::models::UserIdentity get_user(user_id)
Retrieve a user

Get the user data associated with a user. The data returned by this endpoint is highly variable based on the source OAuth provider. Avoid depending on undocumented properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | [**UserId**](.md) | The user identifier. | [required] |

### Return type

[**crate::models::UserIdentity**](UserIdentity.md)


---


## get_users

> crate::models::UserIdentityCollection get_users(limit, cursor, filter, tenant_id)
List users

Returns a paginated user list for the account. The data returned by this endpoint is highly variable based on the source OAuth provider. Avoid depending on undocumented properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Max number of results to return |  |[default to 100]
**cursor** | Option<**String**> | Continuation cursor for paging |  |
**filter** | Option<**String**> | Filter to search users by. This is a case insensitive search through every text field. |  |
**tenant_id** | Option<**String**> | Return only users that are part of the specified tenant. Users can only be part of one tenant, using this parameter will limit returned users that have logged into this tenant. |  |

### Return type

[**crate::models::UserIdentityCollection**](UserIdentityCollection.md)


---

