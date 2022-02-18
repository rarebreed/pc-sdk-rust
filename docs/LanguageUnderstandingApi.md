# \LanguageUnderstandingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_languageunderstanding_domain**](LanguageUnderstandingApi.md#delete_languageunderstanding_domain) | **DELETE** /api/v2/languageunderstanding/domains/{domainId} | Delete an NLU Domain.
[**delete_languageunderstanding_domain_feedback_feedback_id**](LanguageUnderstandingApi.md#delete_languageunderstanding_domain_feedback_feedback_id) | **DELETE** /api/v2/languageunderstanding/domains/{domainId}/feedback/{feedbackId} | Delete the feedback on the NLU Domain Version.
[**delete_languageunderstanding_domain_version**](LanguageUnderstandingApi.md#delete_languageunderstanding_domain_version) | **DELETE** /api/v2/languageunderstanding/domains/{domainId}/versions/{domainVersionId} | Delete an NLU Domain Version
[**delete_languageunderstanding_miner**](LanguageUnderstandingApi.md#delete_languageunderstanding_miner) | **DELETE** /api/v2/languageunderstanding/miners/{minerId} | Delete a miner.
[**delete_languageunderstanding_miner_draft**](LanguageUnderstandingApi.md#delete_languageunderstanding_miner_draft) | **DELETE** /api/v2/languageunderstanding/miners/{minerId}/drafts/{draftId} | Delete a draft
[**get_languageunderstanding_domain**](LanguageUnderstandingApi.md#get_languageunderstanding_domain) | **GET** /api/v2/languageunderstanding/domains/{domainId} | Find an NLU Domain.
[**get_languageunderstanding_domain_feedback**](LanguageUnderstandingApi.md#get_languageunderstanding_domain_feedback) | **GET** /api/v2/languageunderstanding/domains/{domainId}/feedback | Get all feedback in the given NLU Domain Version.
[**get_languageunderstanding_domain_feedback_feedback_id**](LanguageUnderstandingApi.md#get_languageunderstanding_domain_feedback_feedback_id) | **GET** /api/v2/languageunderstanding/domains/{domainId}/feedback/{feedbackId} | Find a Feedback
[**get_languageunderstanding_domain_version**](LanguageUnderstandingApi.md#get_languageunderstanding_domain_version) | **GET** /api/v2/languageunderstanding/domains/{domainId}/versions/{domainVersionId} | Find an NLU Domain Version.
[**get_languageunderstanding_domain_version_report**](LanguageUnderstandingApi.md#get_languageunderstanding_domain_version_report) | **GET** /api/v2/languageunderstanding/domains/{domainId}/versions/{domainVersionId}/report | Retrieved quality report for the specified NLU Domain Version
[**get_languageunderstanding_domain_versions**](LanguageUnderstandingApi.md#get_languageunderstanding_domain_versions) | **GET** /api/v2/languageunderstanding/domains/{domainId}/versions | Get all NLU Domain Versions for a given Domain.
[**get_languageunderstanding_domains**](LanguageUnderstandingApi.md#get_languageunderstanding_domains) | **GET** /api/v2/languageunderstanding/domains | Get all NLU Domains.
[**get_languageunderstanding_miner**](LanguageUnderstandingApi.md#get_languageunderstanding_miner) | **GET** /api/v2/languageunderstanding/miners/{minerId} | Get information about a miner.
[**get_languageunderstanding_miner_draft**](LanguageUnderstandingApi.md#get_languageunderstanding_miner_draft) | **GET** /api/v2/languageunderstanding/miners/{minerId}/drafts/{draftId} | Get information about a draft.
[**get_languageunderstanding_miner_drafts**](LanguageUnderstandingApi.md#get_languageunderstanding_miner_drafts) | **GET** /api/v2/languageunderstanding/miners/{minerId}/drafts | Retrieve the list of drafts created.
[**get_languageunderstanding_miner_intent**](LanguageUnderstandingApi.md#get_languageunderstanding_miner_intent) | **GET** /api/v2/languageunderstanding/miners/{minerId}/intents/{intentId} | Get information about a mined intent
[**get_languageunderstanding_miner_intents**](LanguageUnderstandingApi.md#get_languageunderstanding_miner_intents) | **GET** /api/v2/languageunderstanding/miners/{minerId}/intents | Retrieve a list of mined intents.
[**get_languageunderstanding_miners**](LanguageUnderstandingApi.md#get_languageunderstanding_miners) | **GET** /api/v2/languageunderstanding/miners | Retrieve the list of miners created.
[**patch_languageunderstanding_domain**](LanguageUnderstandingApi.md#patch_languageunderstanding_domain) | **PATCH** /api/v2/languageunderstanding/domains/{domainId} | Update an NLU Domain.
[**patch_languageunderstanding_miner_draft**](LanguageUnderstandingApi.md#patch_languageunderstanding_miner_draft) | **PATCH** /api/v2/languageunderstanding/miners/{minerId}/drafts/{draftId} | Save information for the draft. Either topic draft or intent draft should be sent.
[**post_languageunderstanding_domain_feedback**](LanguageUnderstandingApi.md#post_languageunderstanding_domain_feedback) | **POST** /api/v2/languageunderstanding/domains/{domainId}/feedback | Create feedback for the NLU Domain Version.
[**post_languageunderstanding_domain_version_detect**](LanguageUnderstandingApi.md#post_languageunderstanding_domain_version_detect) | **POST** /api/v2/languageunderstanding/domains/{domainId}/versions/{domainVersionId}/detect | Detect intent, entities, etc. in the submitted text using the specified NLU domain version.
[**post_languageunderstanding_domain_version_publish**](LanguageUnderstandingApi.md#post_languageunderstanding_domain_version_publish) | **POST** /api/v2/languageunderstanding/domains/{domainId}/versions/{domainVersionId}/publish | Publish the draft NLU Domain Version.
[**post_languageunderstanding_domain_version_train**](LanguageUnderstandingApi.md#post_languageunderstanding_domain_version_train) | **POST** /api/v2/languageunderstanding/domains/{domainId}/versions/{domainVersionId}/train | Train the draft NLU Domain Version.
[**post_languageunderstanding_domain_versions**](LanguageUnderstandingApi.md#post_languageunderstanding_domain_versions) | **POST** /api/v2/languageunderstanding/domains/{domainId}/versions | Create an NLU Domain Version.
[**post_languageunderstanding_domains**](LanguageUnderstandingApi.md#post_languageunderstanding_domains) | **POST** /api/v2/languageunderstanding/domains | Create an NLU Domain.
[**post_languageunderstanding_miner_drafts**](LanguageUnderstandingApi.md#post_languageunderstanding_miner_drafts) | **POST** /api/v2/languageunderstanding/miners/{minerId}/drafts | Create a new draft resource.
[**post_languageunderstanding_miner_execute**](LanguageUnderstandingApi.md#post_languageunderstanding_miner_execute) | **POST** /api/v2/languageunderstanding/miners/{minerId}/execute | Start the mining process. Specify date range pair with mediaType, queueIds, participantType for mining data from Genesys Cloud. Specify only uploadKey for mining through an external file.
[**post_languageunderstanding_miners**](LanguageUnderstandingApi.md#post_languageunderstanding_miners) | **POST** /api/v2/languageunderstanding/miners | Create a unique miner.
[**put_languageunderstanding_domain_version**](LanguageUnderstandingApi.md#put_languageunderstanding_domain_version) | **PUT** /api/v2/languageunderstanding/domains/{domainId}/versions/{domainVersionId} | Update an NLU Domain Version.



## delete_languageunderstanding_domain

> delete_languageunderstanding_domain(domain_id)
Delete an NLU Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_languageunderstanding_domain_feedback_feedback_id

> delete_languageunderstanding_domain_feedback_feedback_id(domain_id, feedback_id)
Delete the feedback on the NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**feedback_id** | **String** | ID of the Feedback | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_languageunderstanding_domain_version

> delete_languageunderstanding_domain_version(domain_id, domain_version_id)
Delete an NLU Domain Version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**domain_version_id** | **String** | ID of the NLU domain version. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_languageunderstanding_miner

> delete_languageunderstanding_miner(miner_id)
Delete a miner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_languageunderstanding_miner_draft

> delete_languageunderstanding_miner_draft(miner_id, draft_id)
Delete a draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**draft_id** | **String** | Draft ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_domain

> crate::models::NluDomain get_languageunderstanding_domain(domain_id)
Find an NLU Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |

### Return type

[**crate::models::NluDomain**](NluDomain.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_domain_feedback

> crate::models::NluFeedbackListing get_languageunderstanding_domain_feedback(domain_id, intent_name, assessment, date_start, date_end, include_deleted, page_number, page_size, enable_cursor_pagination, after, fields)
Get all feedback in the given NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**intent_name** | Option<**String**> | The top intent name to retrieve feedback for. |  |
**assessment** | Option<**String**> | The top assessment to retrieve feedback for. |  |
**date_start** | Option<**String**> | Begin of time window as ISO-8601 date. |  |
**date_end** | Option<**String**> | End of time window as ISO-8601 date. |  |
**include_deleted** | Option<**bool**> | Whether to include soft-deleted items in the result. |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**enable_cursor_pagination** | Option<**bool**> | Enable Cursor Pagination |  |[default to false]
**after** | Option<**String**> | The cursor that points to the end of the set of entities that has been returned. This is considered only when enableCursorPagination=true |  |
**fields** | Option<[**Vec<String>**](String.md)> | Fields and properties to get, comma-separated |  |

### Return type

[**crate::models::NluFeedbackListing**](NluFeedbackListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_domain_feedback_feedback_id

> crate::models::NluFeedbackResponse get_languageunderstanding_domain_feedback_feedback_id(domain_id, feedback_id, fields)
Find a Feedback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**feedback_id** | **String** | ID of the Feedback | [required] |
**fields** | Option<[**Vec<String>**](String.md)> | Fields and properties to get, comma-separated |  |

### Return type

[**crate::models::NluFeedbackResponse**](NluFeedbackResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_domain_version

> crate::models::NluDomainVersion get_languageunderstanding_domain_version(domain_id, domain_version_id, include_utterances)
Find an NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**domain_version_id** | **String** | ID of the NLU domain version. | [required] |
**include_utterances** | Option<**bool**> | Whether utterances for intent definition should be included when marshalling response. |  |

### Return type

[**crate::models::NluDomainVersion**](NluDomainVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_domain_version_report

> crate::models::NluDomainVersionQualityReport get_languageunderstanding_domain_version_report(domain_id, domain_version_id)
Retrieved quality report for the specified NLU Domain Version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**domain_version_id** | **String** | ID of the NLU domain version. | [required] |

### Return type

[**crate::models::NluDomainVersionQualityReport**](NluDomainVersionQualityReport.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_domain_versions

> crate::models::NluDomainVersionListing get_languageunderstanding_domain_versions(domain_id, include_utterances, page_number, page_size)
Get all NLU Domain Versions for a given Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**include_utterances** | Option<**bool**> | Whether utterances for intent definition should be included when marshalling response. |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::NluDomainVersionListing**](NluDomainVersionListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_domains

> crate::models::NluDomainListing get_languageunderstanding_domains(page_number, page_size)
Get all NLU Domains.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::NluDomainListing**](NluDomainListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_miner

> crate::models::Miner get_languageunderstanding_miner(miner_id)
Get information about a miner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |

### Return type

[**crate::models::Miner**](Miner.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_miner_draft

> crate::models::Draft get_languageunderstanding_miner_draft(miner_id, draft_id)
Get information about a draft.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**draft_id** | **String** | Draft ID | [required] |

### Return type

[**crate::models::Draft**](Draft.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_miner_drafts

> crate::models::DraftListing get_languageunderstanding_miner_drafts(miner_id)
Retrieve the list of drafts created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |

### Return type

[**crate::models::DraftListing**](DraftListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_miner_intent

> crate::models::MinerIntent get_languageunderstanding_miner_intent(miner_id, intent_id, expand)
Get information about a mined intent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**intent_id** | **String** | The ID of the intent to be retrieved. | [required] |
**expand** | Option<**String**> | Option to fetch utterances |  |

### Return type

[**crate::models::MinerIntent**](MinerIntent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_miner_intents

> crate::models::MinedIntentsListing get_languageunderstanding_miner_intents(miner_id, expand)
Retrieve a list of mined intents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**expand** | Option<**String**> | Option to fetch utterances. |  |

### Return type

[**crate::models::MinedIntentsListing**](MinedIntentsListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languageunderstanding_miners

> crate::models::MinerListing get_languageunderstanding_miners()
Retrieve the list of miners created.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MinerListing**](MinerListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_languageunderstanding_domain

> crate::models::NluDomain patch_languageunderstanding_domain(domain_id, body)
Update an NLU Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**body** | [**NluDomain**](NluDomain.md) | The updated NLU Domain. | [required] |

### Return type

[**crate::models::NluDomain**](NluDomain.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_languageunderstanding_miner_draft

> crate::models::Draft patch_languageunderstanding_miner_draft(miner_id, draft_id, body)
Save information for the draft. Either topic draft or intent draft should be sent.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**draft_id** | **String** | Draft ID | [required] |
**body** | Option<[**DraftRequest**](DraftRequest.md)> |  |  |

### Return type

[**crate::models::Draft**](Draft.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_domain_feedback

> crate::models::NluFeedbackResponse post_languageunderstanding_domain_feedback(domain_id, body)
Create feedback for the NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**body** | [**NluFeedbackRequest**](NluFeedbackRequest.md) | The Feedback to create. | [required] |

### Return type

[**crate::models::NluFeedbackResponse**](NluFeedbackResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_domain_version_detect

> crate::models::NluDetectionResponse post_languageunderstanding_domain_version_detect(domain_id, domain_version_id, body)
Detect intent, entities, etc. in the submitted text using the specified NLU domain version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**domain_version_id** | **String** | ID of the NLU domain version. | [required] |
**body** | [**NluDetectionRequest**](NluDetectionRequest.md) | The input data to perform detection on. | [required] |

### Return type

[**crate::models::NluDetectionResponse**](NluDetectionResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_domain_version_publish

> crate::models::NluDomainVersion post_languageunderstanding_domain_version_publish(domain_id, domain_version_id)
Publish the draft NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**domain_version_id** | **String** | ID of the NLU domain version. | [required] |

### Return type

[**crate::models::NluDomainVersion**](NluDomainVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_domain_version_train

> crate::models::NluDomainVersionTrainingResponse post_languageunderstanding_domain_version_train(domain_id, domain_version_id)
Train the draft NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**domain_version_id** | **String** | ID of the NLU domain version. | [required] |

### Return type

[**crate::models::NluDomainVersionTrainingResponse**](NluDomainVersionTrainingResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_domain_versions

> crate::models::NluDomainVersion post_languageunderstanding_domain_versions(domain_id, body)
Create an NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**body** | [**NluDomainVersion**](NluDomainVersion.md) | The NLU Domain Version to create. | [required] |

### Return type

[**crate::models::NluDomainVersion**](NluDomainVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_domains

> crate::models::NluDomain post_languageunderstanding_domains(body)
Create an NLU Domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**NluDomain**](NluDomain.md) | The NLU Domain to create. | [required] |

### Return type

[**crate::models::NluDomain**](NluDomain.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_miner_drafts

> crate::models::Draft post_languageunderstanding_miner_drafts(miner_id, body)
Create a new draft resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**body** | [**Draft**](Draft.md) | Details for creating draft resource | [required] |

### Return type

[**crate::models::Draft**](Draft.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_miner_execute

> crate::models::Miner post_languageunderstanding_miner_execute(miner_id, body)
Start the mining process. Specify date range pair with mediaType, queueIds, participantType for mining data from Genesys Cloud. Specify only uploadKey for mining through an external file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**body** | Option<[**MinerExecuteRequest**](MinerExecuteRequest.md)> |  |  |

### Return type

[**crate::models::Miner**](Miner.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_miners

> crate::models::Miner post_languageunderstanding_miners(body)
Create a unique miner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Miner**](Miner.md) | Details for creating a new miner resource. | [required] |

### Return type

[**crate::models::Miner**](Miner.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_languageunderstanding_domain_version

> crate::models::NluDomainVersion put_languageunderstanding_domain_version(domain_id, domain_version_id, body)
Update an NLU Domain Version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | ID of the NLU domain. | [required] |
**domain_version_id** | **String** | ID of the NLU domain version. | [required] |
**body** | [**NluDomainVersion**](NluDomainVersion.md) | The updated NLU Domain Version. | [required] |

### Return type

[**crate::models::NluDomainVersion**](NluDomainVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

