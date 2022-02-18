# \SearchApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_documentation_gkn_search**](SearchApi.md#get_documentation_gkn_search) | **GET** /api/v2/documentation/gkn/search | Search gkn documentation using the q64 value returned from a previous search
[**get_documentation_search**](SearchApi.md#get_documentation_search) | **GET** /api/v2/documentation/search | Search documentation using the q64 value returned from a previous search
[**get_groups_search**](SearchApi.md#get_groups_search) | **GET** /api/v2/groups/search | Search groups using the q64 value returned from a previous search
[**get_locations_search**](SearchApi.md#get_locations_search) | **GET** /api/v2/locations/search | Search locations using the q64 value returned from a previous search
[**get_search**](SearchApi.md#get_search) | **GET** /api/v2/search | Search using the q64 value returned from a previous search.
[**get_search_suggest**](SearchApi.md#get_search_suggest) | **GET** /api/v2/search/suggest | Suggest resources using the q64 value returned from a previous suggest query.
[**get_users_search**](SearchApi.md#get_users_search) | **GET** /api/v2/users/search | Search users using the q64 value returned from a previous search
[**get_voicemail_search**](SearchApi.md#get_voicemail_search) | **GET** /api/v2/voicemail/search | Search voicemails using the q64 value returned from a previous search
[**post_analytics_conversations_transcripts_query**](SearchApi.md#post_analytics_conversations_transcripts_query) | **POST** /api/v2/analytics/conversations/transcripts/query | Search resources.
[**post_documentation_gkn_search**](SearchApi.md#post_documentation_gkn_search) | **POST** /api/v2/documentation/gkn/search | Search gkn documentation
[**post_documentation_search**](SearchApi.md#post_documentation_search) | **POST** /api/v2/documentation/search | Search documentation
[**post_groups_search**](SearchApi.md#post_groups_search) | **POST** /api/v2/groups/search | Search groups
[**post_knowledge_knowledgebase_search**](SearchApi.md#post_knowledge_knowledgebase_search) | **POST** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/search | Search Documents
[**post_locations_search**](SearchApi.md#post_locations_search) | **POST** /api/v2/locations/search | Search locations
[**post_search**](SearchApi.md#post_search) | **POST** /api/v2/search | Search resources.
[**post_search_suggest**](SearchApi.md#post_search_suggest) | **POST** /api/v2/search/suggest | Suggest resources.
[**post_speechandtextanalytics_transcripts_search**](SearchApi.md#post_speechandtextanalytics_transcripts_search) | **POST** /api/v2/speechandtextanalytics/transcripts/search | Search resources.
[**post_users_search**](SearchApi.md#post_users_search) | **POST** /api/v2/users/search | Search users
[**post_voicemail_search**](SearchApi.md#post_voicemail_search) | **POST** /api/v2/voicemail/search | Search voicemails



## get_documentation_gkn_search

> crate::models::GknDocumentationSearchResponse get_documentation_gkn_search(q64)
Search gkn documentation using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |

### Return type

[**crate::models::GknDocumentationSearchResponse**](GKNDocumentationSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_documentation_search

> crate::models::DocumentationSearchResponse get_documentation_search(q64)
Search documentation using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |

### Return type

[**crate::models::DocumentationSearchResponse**](DocumentationSearchResponse.md)

### Authorization

No authorization required

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


## get_locations_search

> crate::models::LocationsSearchResponse get_locations_search(q64, expand)
Search locations using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Provides more details about a specified resource |  |

### Return type

[**crate::models::LocationsSearchResponse**](LocationsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search

> crate::models::JsonNodeSearchResponse get_search(q64, expand, profile)
Search using the q64 value returned from a previous search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |
**profile** | Option<**bool**> | profile |  |[default to true]

### Return type

[**crate::models::JsonNodeSearchResponse**](JsonNodeSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_suggest

> crate::models::JsonNodeSearchResponse get_search_suggest(q64, expand, profile)
Suggest resources using the q64 value returned from a previous suggest query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |
**profile** | Option<**bool**> | profile |  |[default to true]

### Return type

[**crate::models::JsonNodeSearchResponse**](JsonNodeSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_search

> crate::models::UsersSearchResponse get_users_search(q64, expand, integration_presence_source)
Search users using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | expand |  |
**integration_presence_source** | Option<**String**> | integrationPresenceSource |  |

### Return type

[**crate::models::UsersSearchResponse**](UsersSearchResponse.md)

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


## post_analytics_conversations_transcripts_query

> crate::models::AnalyticsConversationWithoutAttributesMultiGetResponse post_analytics_conversations_transcripts_query(body)
Search resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TranscriptConversationDetailSearchRequest**](TranscriptConversationDetailSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::AnalyticsConversationWithoutAttributesMultiGetResponse**](AnalyticsConversationWithoutAttributesMultiGetResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_documentation_gkn_search

> crate::models::GknDocumentationSearchResponse post_documentation_gkn_search(body)
Search gkn documentation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GknDocumentationSearchRequest**](GknDocumentationSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::GknDocumentationSearchResponse**](GKNDocumentationSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_documentation_search

> crate::models::DocumentationSearchResponse post_documentation_search(body)
Search documentation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DocumentationSearchRequest**](DocumentationSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::DocumentationSearchResponse**](DocumentationSearchResponse.md)

### Authorization

No authorization required

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


## post_knowledge_knowledgebase_search

> crate::models::KnowledgeSearchResponse post_knowledge_knowledgebase_search(knowledge_base_id, body)
Search Documents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**body** | Option<[**KnowledgeSearchRequest**](KnowledgeSearchRequest.md)> |  |  |

### Return type

[**crate::models::KnowledgeSearchResponse**](KnowledgeSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_locations_search

> crate::models::LocationsSearchResponse post_locations_search(body)
Search locations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LocationSearchRequest**](LocationSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::LocationsSearchResponse**](LocationsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_search

> crate::models::JsonNodeSearchResponse post_search(body, profile)
Search resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SearchRequest**](SearchRequest.md) | Search request options | [required] |
**profile** | Option<**bool**> | profile |  |[default to true]

### Return type

[**crate::models::JsonNodeSearchResponse**](JsonNodeSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_search_suggest

> crate::models::JsonNodeSearchResponse post_search_suggest(body, profile)
Suggest resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SuggestSearchRequest**](SuggestSearchRequest.md) | Search request options | [required] |
**profile** | Option<**bool**> | profile |  |[default to true]

### Return type

[**crate::models::JsonNodeSearchResponse**](JsonNodeSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_speechandtextanalytics_transcripts_search

> crate::models::JsonSearchResponse post_speechandtextanalytics_transcripts_search(body)
Search resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TranscriptSearchRequest**](TranscriptSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::JsonSearchResponse**](JsonSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_search

> crate::models::UsersSearchResponse post_users_search(body)
Search users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserSearchRequest**](UserSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::UsersSearchResponse**](UsersSearchResponse.md)

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

