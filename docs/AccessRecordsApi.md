# AccessRecords
Method | HTTP request | Description
------------- | ------------- | -------------
[**create_claim**](#create_claim) | **POST** /v1/claims | Create resource Claim
[**create_invite**](#create_invite) | **POST** /v1/invites | Create user invite
[**create_record**](#create_record) | **POST** /v1/records | Create access record
[**create_request**](#create_request) | **POST** /v1/requests | Create access request
[**delete_invite**](#delete_invite) | **DELETE** /v1/invites/{inviteId} | Delete invite
[**delete_record**](#delete_record) | **DELETE** /v1/records/{recordId} | Deletes access record
[**delete_request**](#delete_request) | **DELETE** /v1/requests/{requestId} | Deletes access request
[**get_record**](#get_record) | **GET** /v1/records/{recordId} | Retrieve access record
[**get_records**](#get_records) | **GET** /v1/records | List access records
[**get_request**](#get_request) | **GET** /v1/requests/{requestId} | Retrieve access request
[**get_requests**](#get_requests) | **GET** /v1/requests | List access requests
[**respond_to_access_request**](#respond_to_access_request) | **PATCH** /v1/requests/{requestId} | Approve or deny access request
[**respond_to_invite**](#respond_to_invite) | **PATCH** /v1/invites/{inviteId} | Accept invite
[**update_record**](#update_record) | **PUT** /v1/records/{recordId} | Update access record



## create_claim

> serde_json::Value create_claim(claim_request)
Create resource Claim

Claim a resource by allowing a user to pick an identifier and receive admin access to that resource if it hasn't already been claimed. This only works for resources specifically marked as <strong>CLAIM</strong>. The result will be a new access record listing that user as the admin for this resource. The resourceUri will be appended to the collection resource uri using a '/' (forward slash) automatically.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**claim_request** | [**ClaimRequest**](ClaimRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

---


## create_invite

> crate::models::Invite create_invite(invite)
Create user invite

Invites are used to easily assign permissions to users that have not been created in your identity provider yet. 1. This generates an invite record. 2. Send the invite ID to the user. 3. Log the user in. 4. As the user PATCH the /invite/{inviteId} endpoint 5. This accepts the invite.         When the user accepts the invite they will automatically receive the permissions assigned in the Invite. Invites automatically expire after 7 days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite** | [**Invite**](Invite.md) |  | [required] |

### Return type

[**crate::models::Invite**](Invite.md)

---


## create_record

> crate::models::AccessRecord create_record(access_record)
Create access record

Specify user roles for specific resources. (Records have a maximum size of ~100KB)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_record** | [**AccessRecord**](AccessRecord.md) |  | [required] |

### Return type

[**crate::models::AccessRecord**](AccessRecord.md)

---


## create_request

> crate::models::AccessRequest create_request(access_request)
Create access request

Specify a request in the form of an access record that an admin can approve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**access_request** | [**AccessRequest**](AccessRequest.md) |  | [required] |

### Return type

[**crate::models::AccessRequest**](AccessRequest.md)

---


## delete_invite

> delete_invite(invite_id)
Delete invite

Deletes an invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_id** | **String** | The identifier of the invite. | [required] |

### Return type

 (empty response body)


---


## delete_record

> delete_record(record_id)
Deletes access record

Remove an access record, removing associated permissions from all users in record. If a user has a permission from another record, that permission will not be removed. (Note: This disables the record by changing the status to <strong>DELETED</strong> but not completely remove the record for tracking purposes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**record_id** | **String** | The identifier of the access record. | [required] |

### Return type

 (empty response body)


---


## delete_request

> delete_request(request_id)
Deletes access request

Remove an access request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | The identifier of the access request. | [required] |

### Return type

 (empty response body)


---


## get_record

> crate::models::AccessRecord get_record(record_id)
Retrieve access record

Access records contain information assigning permissions to users for resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**record_id** | **String** | The identifier of the access record. | [required] |

### Return type

[**crate::models::AccessRecord**](AccessRecord.md)


---


## get_records

> crate::models::AccessRecordCollection get_records(limit, cursor, filter, status)
List access records

Returns a paginated records list for the account. Only records the user has access to are returned. This query resource is meant for administrative actions only, therefore has a lower rate limit tier than user permissions related resources. Additionally, the results from a query to Access Records may be delayed up to 5 minutes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Max number of results to return |  |[default to 20]
**cursor** | Option<**String**> | Continuation cursor for paging |  |
**filter** | Option<**String**> | Filter to search records by. This is a case insensitive search through every text field. |  |
**status** | Option<**String**> | Filter records by their current status. |  |

### Return type

[**crate::models::AccessRecordCollection**](AccessRecordCollection.md)


---


## get_request

> crate::models::AccessRequest get_request(request_id)
Retrieve access request

Access request contain information requesting permissions for users to resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | The identifier of the access request. | [required] |

### Return type

[**crate::models::AccessRequest**](AccessRequest.md)


---


## get_requests

> crate::models::AccessRequestCollection get_requests(limit, cursor, status)
List access requests

Returns a paginated request list. Only requests the user has access to are returned. This query resource is meant for administrative actions only, therefore has a lower rate limit tier than user permissions related resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**u32**> | Max number of results to return |  |[default to 20]
**cursor** | Option<**String**> | Continuation cursor for paging |  |
**status** | Option<**String**> | Filter requests by their current status. |  |

### Return type

[**crate::models::AccessRequestCollection**](AccessRequestCollection.md)


---


## respond_to_access_request

> crate::models::AccessRequest respond_to_access_request(request_id, access_request_response)
Approve or deny access request

Updates an access request, approving it or denying it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | The identifier of the access request. | [required] |
**access_request_response** | [**AccessRequestResponse**](AccessRequestResponse.md) |  | [required] |

### Return type

[**crate::models::AccessRequest**](AccessRequest.md)

---


## respond_to_invite

> crate::models::Account respond_to_invite(invite_id)
Accept invite

Accepts an invite by claiming this invite by this user. The user access token used for this request will gain the permissions associated with the invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_id** | **String** | The identifier of the invite. | [required] |

### Return type

[**crate::models::Account**](Account.md)


---


## update_record

> update_record(record_id, access_record, { expected_last_modified_time })
Update access record

Updates an access record adding or removing user permissions to resources. (Records have a maximum size of ~100KB)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**record_id** | **String** | The identifier of the access record. | [required] |
**access_record** | [**AccessRecord**](AccessRecord.md) |  | [required] |
**expected_last_modified_time** | Option<**DateTime**> | The expected last time the record was modified. |  |

### Return type

 (empty response body)

---

