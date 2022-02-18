# \PresenceApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_presencedefinition**](PresenceApi.md#delete_presencedefinition) | **DELETE** /api/v2/presencedefinitions/{presenceId} | Delete a Presence Definition
[**get_presencedefinition**](PresenceApi.md#get_presencedefinition) | **GET** /api/v2/presencedefinitions/{presenceId} | Get a Presence Definition
[**get_presencedefinitions**](PresenceApi.md#get_presencedefinitions) | **GET** /api/v2/presencedefinitions | Get an Organization's list of Presence Definitions
[**get_systempresences**](PresenceApi.md#get_systempresences) | **GET** /api/v2/systempresences | Get the list of SystemPresences
[**get_user_presence**](PresenceApi.md#get_user_presence) | **GET** /api/v2/users/{userId}/presences/{sourceId} | Get a user's Presence
[**get_user_presences_microsoftteams**](PresenceApi.md#get_user_presences_microsoftteams) | **GET** /api/v2/users/{userId}/presences/microsoftteams | Get a user's Microsoft Teams presence.
[**get_user_presences_purecloud**](PresenceApi.md#get_user_presences_purecloud) | **GET** /api/v2/users/{userId}/presences/purecloud | Get a user's Genesys Cloud presence.
[**get_user_presences_zoomphone**](PresenceApi.md#get_user_presences_zoomphone) | **GET** /api/v2/users/{userId}/presences/zoomphone | Get a user's Zoom Phone presence.
[**patch_user_presence**](PresenceApi.md#patch_user_presence) | **PATCH** /api/v2/users/{userId}/presences/{sourceId} | Patch a user's Presence
[**patch_user_presences_purecloud**](PresenceApi.md#patch_user_presences_purecloud) | **PATCH** /api/v2/users/{userId}/presences/purecloud | Patch a Genesys Cloud user's presence
[**post_presencedefinitions**](PresenceApi.md#post_presencedefinitions) | **POST** /api/v2/presencedefinitions | Create a Presence Definition
[**put_presencedefinition**](PresenceApi.md#put_presencedefinition) | **PUT** /api/v2/presencedefinitions/{presenceId} | Update a Presence Definition
[**put_users_presences_bulk**](PresenceApi.md#put_users_presences_bulk) | **PUT** /api/v2/users/presences/bulk | Update bulk user Presences



## delete_presencedefinition

> delete_presencedefinition(presence_id)
Delete a Presence Definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**presence_id** | **String** | Organization Presence ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_presencedefinition

> crate::models::OrganizationPresence get_presencedefinition(presence_id, locale_code)
Get a Presence Definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**presence_id** | **String** | Organization Presence ID | [required] |
**locale_code** | Option<**String**> | The locale code to fetch for the presence definition. Use ALL to fetch everything. |  |

### Return type

[**crate::models::OrganizationPresence**](OrganizationPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_presencedefinitions

> crate::models::OrganizationPresenceEntityListing get_presencedefinitions(page_number, page_size, deleted, locale_code)
Get an Organization's list of Presence Definitions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**deleted** | Option<**String**> | Deleted query can be TRUE, FALSE or ALL |  |[default to false]
**locale_code** | Option<**String**> | The locale code to fetch for each presence definition. Use ALL to fetch everything. |  |

### Return type

[**crate::models::OrganizationPresenceEntityListing**](OrganizationPresenceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_systempresences

> Vec<crate::models::SystemPresence> get_systempresences()
Get the list of SystemPresences

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SystemPresence>**](SystemPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_presence

> crate::models::UserPresence get_user_presence(user_id, source_id)
Get a user's Presence

Get a user's presence for the specified source that is not specifically listed.  Used to support custom presence sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |
**source_id** | **String** | Presence source ID | [required] |

### Return type

[**crate::models::UserPresence**](UserPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_presences_microsoftteams

> crate::models::PresenceExpand get_user_presences_microsoftteams(user_id)
Get a user's Microsoft Teams presence.

Gets the presence for a Microsoft Teams user.  This will return the Microsoft Teams presence mapped to Genesys Cloud presence with additional activity details in the message field. This presence source is read-only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |

### Return type

[**crate::models::PresenceExpand**](PresenceExpand.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_presences_purecloud

> crate::models::UserPresence get_user_presences_purecloud(user_id)
Get a user's Genesys Cloud presence.

Get the default Genesys Cloud user presence source PURECLOUD

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |

### Return type

[**crate::models::UserPresence**](UserPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_presences_zoomphone

> crate::models::PresenceExpand get_user_presences_zoomphone(user_id)
Get a user's Zoom Phone presence.

Gets the presence for a Zoom user.  This will return the Zoom Phone presence mapped to Genesys Cloud presence with additional activity details in the message field. This presence source is read-only.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |

### Return type

[**crate::models::PresenceExpand**](PresenceExpand.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_presence

> crate::models::UserPresence patch_user_presence(user_id, source_id, body)
Patch a user's Presence

Patch a user's presence for the specified source that is not specifically listed. The presence object can be patched one of three ways. Option 1: Set the 'primary' property to true. This will set the 'source' defined in the path as the user's primary presence source. Option 2: Provide the presenceDefinition value. The 'id' is the only value required within the presenceDefinition. Option 3: Provide the message value. Option 1 can be combined with Option 2 and/or Option 3.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |
**source_id** | **String** | Presence source ID | [required] |
**body** | [**UserPresence**](UserPresence.md) | User presence | [required] |

### Return type

[**crate::models::UserPresence**](UserPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_presences_purecloud

> crate::models::UserPresence patch_user_presences_purecloud(user_id, body)
Patch a Genesys Cloud user's presence

The presence object can be patched one of three ways. Option 1: Set the 'primary' property to true. This will set the PURECLOUD source as the user's primary presence source. Option 2: Provide the presenceDefinition value. The 'id' is the only value required within the presenceDefinition. Option 3: Provide the message value. Option 1 can be combined with Option 2 and/or Option 3.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |
**body** | [**UserPresence**](UserPresence.md) | User presence | [required] |

### Return type

[**crate::models::UserPresence**](UserPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_presencedefinitions

> crate::models::OrganizationPresence post_presencedefinitions(body)
Create a Presence Definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OrganizationPresence**](OrganizationPresence.md) | The Presence Definition to create | [required] |

### Return type

[**crate::models::OrganizationPresence**](OrganizationPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_presencedefinition

> crate::models::OrganizationPresence put_presencedefinition(presence_id, body)
Update a Presence Definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**presence_id** | **String** | Organization Presence ID | [required] |
**body** | [**OrganizationPresence**](OrganizationPresence.md) | The OrganizationPresence to update | [required] |

### Return type

[**crate::models::OrganizationPresence**](OrganizationPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_users_presences_bulk

> Vec<crate::models::UserPresence> put_users_presences_bulk(body)
Update bulk user Presences

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::UserPresence>**](UserPresence.md) | List of User presences | [required] |

### Return type

[**Vec<crate::models::UserPresence>**](UserPresence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

