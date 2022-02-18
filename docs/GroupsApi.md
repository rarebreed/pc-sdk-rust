# \GroupsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /api/v2/groups/{groupId} | Delete group
[**delete_group_members**](GroupsApi.md#delete_group_members) | **DELETE** /api/v2/groups/{groupId}/members | Remove members
[**get_fieldconfig**](GroupsApi.md#get_fieldconfig) | **GET** /api/v2/fieldconfig | Fetch field config for an entity type
[**get_group**](GroupsApi.md#get_group) | **GET** /api/v2/groups/{groupId} | Get group
[**get_group_individuals**](GroupsApi.md#get_group_individuals) | **GET** /api/v2/groups/{groupId}/individuals | Get all individuals associated with the group
[**get_group_members**](GroupsApi.md#get_group_members) | **GET** /api/v2/groups/{groupId}/members | Get group members, includes individuals, owners, and dynamically included people
[**get_group_profile**](GroupsApi.md#get_group_profile) | **GET** /api/v2/groups/{groupId}/profile | Get group profile
[**get_groups**](GroupsApi.md#get_groups) | **GET** /api/v2/groups | Get a group list
[**get_groups_search**](GroupsApi.md#get_groups_search) | **GET** /api/v2/groups/search | Search groups using the q64 value returned from a previous search
[**get_profiles_groups**](GroupsApi.md#get_profiles_groups) | **GET** /api/v2/profiles/groups | Get group profile listing
[**post_group_members**](GroupsApi.md#post_group_members) | **POST** /api/v2/groups/{groupId}/members | Add members
[**post_groups**](GroupsApi.md#post_groups) | **POST** /api/v2/groups | Create a group
[**post_groups_search**](GroupsApi.md#post_groups_search) | **POST** /api/v2/groups/search | Search groups
[**put_group**](GroupsApi.md#put_group) | **PUT** /api/v2/groups/{groupId} | Update group



## delete_group

> delete_group(group_id)
Delete group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_members

> serde_json::Value delete_group_members(group_id, ids)
Remove members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**ids** | **String** | Comma separated list of userIds to remove | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fieldconfig

> crate::models::FieldConfig get_fieldconfig(_type)
Fetch field config for an entity type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Field type | [required] |

### Return type

[**crate::models::FieldConfig**](FieldConfig.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> crate::models::Group get_group(group_id)
Get group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_individuals

> crate::models::UserEntityListing get_group_individuals(group_id)
Get all individuals associated with the group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

[**crate::models::UserEntityListing**](UserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_members

> crate::models::UserEntityListing get_group_members(group_id, page_size, page_number, sort_order, expand)
Get group members, includes individuals, owners, and dynamically included people

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::UserEntityListing**](UserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_profile

> crate::models::GroupProfile get_group_profile(group_id, fields)
Get group profile

This api is deprecated. Use /api/v2/groups instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | groupId | [required] |
**fields** | Option<**String**> | Comma separated fields to return.  Allowable values can be found by querying /api/v2/fieldconfig?type=group and using the key for the elements returned by the fieldList |  |

### Return type

[**crate::models::GroupProfile**](GroupProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups

> crate::models::GroupEntityListing get_groups(page_size, page_number, id, jabber_id, sort_order)
Get a group list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**jabber_id** | Option<[**Vec<String>**](String.md)> | A list of jabberIds to fetch by bulk (cannot be used with the \"id\" parameter) |  |
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]

### Return type

[**crate::models::GroupEntityListing**](GroupEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_search

> crate::models::GroupsSearchResponse get_groups_search(q64, expand)
Search groups using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | expand |  |

### Return type

[**crate::models::GroupsSearchResponse**](GroupsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profiles_groups

> crate::models::GroupProfileEntityListing get_profiles_groups(page_size, page_number, id, sort_order)
Get group profile listing

This api is deprecated. Use /api/v2/groups instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]

### Return type

[**crate::models::GroupProfileEntityListing**](GroupProfileEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_group_members

> serde_json::Value post_group_members(group_id, body)
Add members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**body** | [**GroupMembersUpdate**](GroupMembersUpdate.md) | Add members | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_groups

> crate::models::Group post_groups(body)
Create a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GroupCreate**](GroupCreate.md) | Group | [required] |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_groups_search

> crate::models::GroupsSearchResponse post_groups_search(body)
Search groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GroupSearchRequest**](GroupSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::GroupsSearchResponse**](GroupsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_group

> crate::models::Group put_group(group_id, body)
Update group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**body** | Option<[**GroupUpdate**](GroupUpdate.md)> | Group |  |

### Return type

[**crate::models::Group**](Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

