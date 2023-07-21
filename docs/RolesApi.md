# Roles
Method | HTTP request | Description
------------- | ------------- | -------------
[**create_role**](#create_role) | **POST** /v1/roles | Create role
[**delete_role**](#delete_role) | **DELETE** /v1/roles/{roleId} | Deletes role
[**get_role**](#get_role) | **GET** /v1/roles/{roleId} | Retrieve role
[**get_roles**](#get_roles) | **GET** /v1/roles | List roles
[**update_role**](#update_role) | **PUT** /v1/roles/{roleId} | Update role



## create_role

> crate::models::Role create_role(role)
Create role

Creates a role with permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

---


## delete_role

> delete_role(role_id)
Deletes role

Remove a role. If a record references the role, that record will not be modified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier of the role. | [required] |

### Return type

 (empty response body)


---


## get_role

> crate::models::Role get_role(role_id)
Retrieve role

Roles contain a list of permissions that will be applied to any user or resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier of the role. | [required] |

### Return type

[**crate::models::Role**](Role.md)


---


## get_roles

> crate::models::RoleCollection get_roles()
List roles

Get all the account roles. Roles contain a list of permissions that will be applied to any user or resource

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RoleCollection**](RoleCollection.md)


---


## update_role

> crate::models::Role update_role(role_id, role)
Update role

Updates a role adding or removing permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | The identifier of the role. | [required] |
**role** | [**Role**](Role.md) |  | [required] |

### Return type

[**crate::models::Role**](Role.md)

---

