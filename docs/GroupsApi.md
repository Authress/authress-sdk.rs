# \GroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group**](GroupsApi.md#create_group) | **POST** /v1/groups | Create group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /v1/groups/{groupId} | Deletes group
[**get_group**](GroupsApi.md#get_group) | **GET** /v1/groups/{groupId} | Retrieve group
[**get_groups**](GroupsApi.md#get_groups) | **GET** /v1/groups | List groups
[**update_group**](GroupsApi.md#update_group) | **PUT** /v1/groups/{groupId} | Update a group



## create_group

> crate::models::Group create_group(group)
Create group

Specify users to be included in a new group. (Groups have a maximum size of ~100KB)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | [**Group**](Group.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## delete_group

> delete_group(group_id)
Deletes group

Remove a group, users will lose any role that was assigned through membership of this group. This action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The identifier of the group. | [required] |

### Return type

 (empty response body)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_group

> crate::models::Group get_group(group_id)
Retrieve group

A group contains multiple users which can be added to an access record, and should be assigned the same roles at the same time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The identifier of the group. | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## get_groups

> crate::models::GroupCollection get_groups(limit, cursor, filter)
List groups

Returns a paginated groups list for the account. Only groups the user has access to are returned. This query resource is meant for administrative actions only, therefore has a lower rate limit tier than user permissions related resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Max number of results to return |  |[default to 20]
**cursor** | Option<**String**> | Continuation cursor for paging |  |
**filter** | Option<**String**> | Filter to search groups by. This is a case insensitive search through every text field. |  |

### Return type

[**crate::models::GroupCollection**](GroupCollection.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)


## update_group

> crate::models::Group update_group(group_id, group)
Update a group

Updates a group adding or removing user. Change a group updates the permissions and roles the users have access to. (Groups have a maximum size of ~100KB)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The identifier of the group. | [required] |
**group** | [**Group**](Group.md) |  | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[oauth2](./README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/links+json

[[Back to top]](#) [[Back to API list]](./README.md#documentation-for-api-endpoints) [[Back to Model list]](./README.md#documentation-for-models) [[Back to README]](./README.md)

