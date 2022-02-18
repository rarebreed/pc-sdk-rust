# \ObjectsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_authorization_division**](ObjectsApi.md#delete_authorization_division) | **DELETE** /api/v2/authorization/divisions/{divisionId} | Delete a division.
[**get_authorization_division**](ObjectsApi.md#get_authorization_division) | **GET** /api/v2/authorization/divisions/{divisionId} | Returns an authorization division.
[**get_authorization_divisions**](ObjectsApi.md#get_authorization_divisions) | **GET** /api/v2/authorization/divisions | Retrieve a list of all divisions defined for the organization
[**get_authorization_divisions_home**](ObjectsApi.md#get_authorization_divisions_home) | **GET** /api/v2/authorization/divisions/home | Retrieve the home division for the organization.
[**get_authorization_divisions_limit**](ObjectsApi.md#get_authorization_divisions_limit) | **GET** /api/v2/authorization/divisions/limit | Returns the maximum allowed number of divisions.
[**post_authorization_division_object**](ObjectsApi.md#post_authorization_division_object) | **POST** /api/v2/authorization/divisions/{divisionId}/objects/{objectType} | Assign a list of objects to a division
[**post_authorization_division_restore**](ObjectsApi.md#post_authorization_division_restore) | **POST** /api/v2/authorization/divisions/{divisionId}/restore | Recreate a previously deleted division.
[**post_authorization_divisions**](ObjectsApi.md#post_authorization_divisions) | **POST** /api/v2/authorization/divisions | Create a division.
[**put_authorization_division**](ObjectsApi.md#put_authorization_division) | **PUT** /api/v2/authorization/divisions/{divisionId} | Update a division.



## delete_authorization_division

> delete_authorization_division(division_id, force)
Delete a division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**force** | Option<**bool**> | Force delete this division as well as the grants and objects associated with it |  |[default to false]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_division

> crate::models::AuthzDivision get_authorization_division(division_id, object_count)
Returns an authorization division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**object_count** | Option<**bool**> | Get count of objects in this division, grouped by type |  |[default to false]

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisions

> crate::models::AuthzDivisionEntityListing get_authorization_divisions(page_size, page_number, sort_by, expand, next_page, previous_page, object_count, id, name)
Retrieve a list of all divisions defined for the organization

Request specific divisions by id using a query param \"id\", e.g.  ?id=5f777167-63be-4c24-ad41-374155d9e28b&id=72e9fb25-c484-488d-9312-7acba82435b3

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**object_count** | Option<**bool**> | Include the count of objects contained in the division |  |[default to false]
**id** | Option<[**Vec<String>**](String.md)> | Optionally request specific divisions by their IDs |  |
**name** | Option<**String**> | Search term to filter by division name |  |

### Return type

[**crate::models::AuthzDivisionEntityListing**](AuthzDivisionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisions_home

> crate::models::AuthzDivision get_authorization_divisions_home()
Retrieve the home division for the organization.

Will not include object counts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisions_limit

> i32 get_authorization_divisions_limit()
Returns the maximum allowed number of divisions.

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_division_object

> post_authorization_division_object(division_id, object_type, body)
Assign a list of objects to a division

Set the division of a specified list of objects. The objects must all be of the same type, one of:  CAMPAIGN, MANAGEMENTUNIT, FLOW, QUEUE, DATATABLES or USER.  The body of the request is a list of object IDs, which are expected to be  GUIDs, e.g. [\"206ce31f-61ec-40ed-a8b1-be6f06303998\",\"250a754e-f5e4-4f51-800f-a92f09d3bf8c\"]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**object_type** | **String** | The type of the objects. Must be one of the valid object types | [required] |
**body** | [**Vec<String>**](String.md) | Object Id List | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_division_restore

> crate::models::AuthzDivision post_authorization_division_restore(division_id, body)
Recreate a previously deleted division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**body** | [**AuthzDivision**](AuthzDivision.md) | Recreated division data | [required] |

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_divisions

> crate::models::AuthzDivision post_authorization_divisions(body)
Create a division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AuthzDivision**](AuthzDivision.md) | Division | [required] |

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_authorization_division

> crate::models::AuthzDivision put_authorization_division(division_id, body)
Update a division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**body** | [**AuthzDivision**](AuthzDivision.md) | Updated division data | [required] |

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

