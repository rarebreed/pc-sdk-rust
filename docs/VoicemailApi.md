# \VoicemailApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_voicemail_message**](VoicemailApi.md#delete_voicemail_message) | **DELETE** /api/v2/voicemail/messages/{messageId} | Delete a voicemail message.
[**delete_voicemail_messages**](VoicemailApi.md#delete_voicemail_messages) | **DELETE** /api/v2/voicemail/messages | Delete all voicemail messages
[**get_voicemail_group_mailbox**](VoicemailApi.md#get_voicemail_group_mailbox) | **GET** /api/v2/voicemail/groups/{groupId}/mailbox | Get the group's mailbox information
[**get_voicemail_group_messages**](VoicemailApi.md#get_voicemail_group_messages) | **GET** /api/v2/voicemail/groups/{groupId}/messages | List voicemail messages
[**get_voicemail_group_policy**](VoicemailApi.md#get_voicemail_group_policy) | **GET** /api/v2/voicemail/groups/{groupId}/policy | Get a group's voicemail policy
[**get_voicemail_mailbox**](VoicemailApi.md#get_voicemail_mailbox) | **GET** /api/v2/voicemail/mailbox | Get the current user's mailbox information
[**get_voicemail_me_mailbox**](VoicemailApi.md#get_voicemail_me_mailbox) | **GET** /api/v2/voicemail/me/mailbox | Get the current user's mailbox information
[**get_voicemail_me_messages**](VoicemailApi.md#get_voicemail_me_messages) | **GET** /api/v2/voicemail/me/messages | List voicemail messages
[**get_voicemail_me_policy**](VoicemailApi.md#get_voicemail_me_policy) | **GET** /api/v2/voicemail/me/policy | Get the current user's voicemail policy
[**get_voicemail_message**](VoicemailApi.md#get_voicemail_message) | **GET** /api/v2/voicemail/messages/{messageId} | Get a voicemail message
[**get_voicemail_message_media**](VoicemailApi.md#get_voicemail_message_media) | **GET** /api/v2/voicemail/messages/{messageId}/media | Get media playback URI for this voicemail message
[**get_voicemail_messages**](VoicemailApi.md#get_voicemail_messages) | **GET** /api/v2/voicemail/messages | List voicemail messages
[**get_voicemail_policy**](VoicemailApi.md#get_voicemail_policy) | **GET** /api/v2/voicemail/policy | Get a policy
[**get_voicemail_queue_messages**](VoicemailApi.md#get_voicemail_queue_messages) | **GET** /api/v2/voicemail/queues/{queueId}/messages | List voicemail messages
[**get_voicemail_search**](VoicemailApi.md#get_voicemail_search) | **GET** /api/v2/voicemail/search | Search voicemails using the q64 value returned from a previous search
[**get_voicemail_userpolicy**](VoicemailApi.md#get_voicemail_userpolicy) | **GET** /api/v2/voicemail/userpolicies/{userId} | Get a user's voicemail policy
[**patch_voicemail_group_policy**](VoicemailApi.md#patch_voicemail_group_policy) | **PATCH** /api/v2/voicemail/groups/{groupId}/policy | Update a group's voicemail policy
[**patch_voicemail_me_policy**](VoicemailApi.md#patch_voicemail_me_policy) | **PATCH** /api/v2/voicemail/me/policy | Update the current user's voicemail policy
[**patch_voicemail_message**](VoicemailApi.md#patch_voicemail_message) | **PATCH** /api/v2/voicemail/messages/{messageId} | Update a voicemail message
[**patch_voicemail_userpolicy**](VoicemailApi.md#patch_voicemail_userpolicy) | **PATCH** /api/v2/voicemail/userpolicies/{userId} | Update a user's voicemail policy
[**post_voicemail_messages**](VoicemailApi.md#post_voicemail_messages) | **POST** /api/v2/voicemail/messages | Copy a voicemail message to a user or group
[**post_voicemail_search**](VoicemailApi.md#post_voicemail_search) | **POST** /api/v2/voicemail/search | Search voicemails
[**put_voicemail_message**](VoicemailApi.md#put_voicemail_message) | **PUT** /api/v2/voicemail/messages/{messageId} | Update a voicemail message
[**put_voicemail_policy**](VoicemailApi.md#put_voicemail_policy) | **PUT** /api/v2/voicemail/policy | Update a policy
[**put_voicemail_userpolicy**](VoicemailApi.md#put_voicemail_userpolicy) | **PUT** /api/v2/voicemail/userpolicies/{userId} | Update a user's voicemail policy



## delete_voicemail_message

> delete_voicemail_message(message_id)
Delete a voicemail message.

A user voicemail can only be deleted by its associated user. A group voicemail can only be deleted by a user that is a member of the group. A queue voicemail can only be deleted by a user with the acd voicemail delete permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_voicemail_messages

> delete_voicemail_messages()
Delete all voicemail messages

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_group_mailbox

> crate::models::VoicemailMailboxInfo get_voicemail_group_mailbox(group_id)
Get the group's mailbox information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | groupId | [required] |

### Return type

[**crate::models::VoicemailMailboxInfo**](VoicemailMailboxInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_group_messages

> crate::models::VoicemailMessageEntityListing get_voicemail_group_messages(group_id, page_size, page_number)
List voicemail messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::VoicemailMessageEntityListing**](VoicemailMessageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_group_policy

> crate::models::VoicemailGroupPolicy get_voicemail_group_policy(group_id)
Get a group's voicemail policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |

### Return type

[**crate::models::VoicemailGroupPolicy**](VoicemailGroupPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_mailbox

> crate::models::VoicemailMailboxInfo get_voicemail_mailbox()
Get the current user's mailbox information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoicemailMailboxInfo**](VoicemailMailboxInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_me_mailbox

> crate::models::VoicemailMailboxInfo get_voicemail_me_mailbox()
Get the current user's mailbox information

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoicemailMailboxInfo**](VoicemailMailboxInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_me_messages

> crate::models::VoicemailMessageEntityListing get_voicemail_me_messages(page_size, page_number)
List voicemail messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::VoicemailMessageEntityListing**](VoicemailMessageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_me_policy

> crate::models::VoicemailUserPolicy get_voicemail_me_policy()
Get the current user's voicemail policy

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoicemailUserPolicy**](VoicemailUserPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_message

> crate::models::VoicemailMessage get_voicemail_message(message_id, expand)
Get a voicemail message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | If the caller is a known user, which fields, if any, to expand |  |

### Return type

[**crate::models::VoicemailMessage**](VoicemailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_message_media

> crate::models::VoicemailMediaInfo get_voicemail_message_media(message_id, format_id)
Get media playback URI for this voicemail message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID | [required] |
**format_id** | Option<**String**> | The desired media format. |  |[default to WEBM]

### Return type

[**crate::models::VoicemailMediaInfo**](VoicemailMediaInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_messages

> crate::models::VoicemailMessageEntityListing get_voicemail_messages(ids, expand)
List voicemail messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<**String**> | An optional comma separated list of VoicemailMessage ids |  |
**expand** | Option<[**Vec<String>**](String.md)> | If the caller is a known user, which fields, if any, to expand |  |

### Return type

[**crate::models::VoicemailMessageEntityListing**](VoicemailMessageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_policy

> crate::models::VoicemailOrganizationPolicy get_voicemail_policy()
Get a policy

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoicemailOrganizationPolicy**](VoicemailOrganizationPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_queue_messages

> crate::models::VoicemailMessageEntityListing get_voicemail_queue_messages(queue_id, page_size, page_number)
List voicemail messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::VoicemailMessageEntityListing**](VoicemailMessageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_search

> crate::models::VoicemailsSearchResponse get_voicemail_search(q64, expand)
Search voicemails using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | expand |  |

### Return type

[**crate::models::VoicemailsSearchResponse**](VoicemailsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voicemail_userpolicy

> crate::models::VoicemailUserPolicy get_voicemail_userpolicy(user_id)
Get a user's voicemail policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::VoicemailUserPolicy**](VoicemailUserPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_voicemail_group_policy

> crate::models::VoicemailGroupPolicy patch_voicemail_group_policy(group_id, body)
Update a group's voicemail policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Group ID | [required] |
**body** | [**VoicemailGroupPolicy**](VoicemailGroupPolicy.md) | The group's voicemail policy | [required] |

### Return type

[**crate::models::VoicemailGroupPolicy**](VoicemailGroupPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_voicemail_me_policy

> crate::models::VoicemailUserPolicy patch_voicemail_me_policy(body)
Update the current user's voicemail policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**VoicemailUserPolicy**](VoicemailUserPolicy.md) | The user's voicemail policy | [required] |

### Return type

[**crate::models::VoicemailUserPolicy**](VoicemailUserPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_voicemail_message

> crate::models::VoicemailMessage patch_voicemail_message(message_id, body)
Update a voicemail message

A user voicemail can only be modified by its associated user. A group voicemail can only be modified by a user that is a member of the group. A queue voicemail can only be modified by a participant of the conversation the voicemail is associated with.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID | [required] |
**body** | [**VoicemailMessage**](VoicemailMessage.md) | VoicemailMessage | [required] |

### Return type

[**crate::models::VoicemailMessage**](VoicemailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_voicemail_userpolicy

> crate::models::VoicemailUserPolicy patch_voicemail_userpolicy(user_id, body)
Update a user's voicemail policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**VoicemailUserPolicy**](VoicemailUserPolicy.md) | The user's voicemail policy | [required] |

### Return type

[**crate::models::VoicemailUserPolicy**](VoicemailUserPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_voicemail_messages

> crate::models::VoicemailMessage post_voicemail_messages(body)
Copy a voicemail message to a user or group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CopyVoicemailMessage**](CopyVoicemailMessage.md)> |  |  |

### Return type

[**crate::models::VoicemailMessage**](VoicemailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_voicemail_search

> crate::models::VoicemailsSearchResponse post_voicemail_search(body)
Search voicemails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**VoicemailSearchRequest**](VoicemailSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::VoicemailsSearchResponse**](VoicemailsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voicemail_message

> crate::models::VoicemailMessage put_voicemail_message(message_id, body)
Update a voicemail message

A user voicemail can only be modified by its associated user. A group voicemail can only be modified by a user that is a member of the group. A queue voicemail can only be modified by a participant of the conversation the voicemail is associated with.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **String** | Message ID | [required] |
**body** | [**VoicemailMessage**](VoicemailMessage.md) | VoicemailMessage | [required] |

### Return type

[**crate::models::VoicemailMessage**](VoicemailMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voicemail_policy

> crate::models::VoicemailOrganizationPolicy put_voicemail_policy(body)
Update a policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**VoicemailOrganizationPolicy**](VoicemailOrganizationPolicy.md) | Policy | [required] |

### Return type

[**crate::models::VoicemailOrganizationPolicy**](VoicemailOrganizationPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voicemail_userpolicy

> crate::models::VoicemailUserPolicy put_voicemail_userpolicy(user_id, body)
Update a user's voicemail policy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**VoicemailUserPolicy**](VoicemailUserPolicy.md) | The user's voicemail policy | [required] |

### Return type

[**crate::models::VoicemailUserPolicy**](VoicemailUserPolicy.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

