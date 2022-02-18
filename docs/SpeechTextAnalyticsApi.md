# \SpeechTextAnalyticsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_speechandtextanalytics_program**](SpeechTextAnalyticsApi.md#delete_speechandtextanalytics_program) | **DELETE** /api/v2/speechandtextanalytics/programs/{programId} | Delete a Speech & Text Analytics program by id
[**delete_speechandtextanalytics_sentimentfeedback**](SpeechTextAnalyticsApi.md#delete_speechandtextanalytics_sentimentfeedback) | **DELETE** /api/v2/speechandtextanalytics/sentimentfeedback | Delete All Speech & Text Analytics SentimentFeedback
[**delete_speechandtextanalytics_sentimentfeedback_sentiment_feedback_id**](SpeechTextAnalyticsApi.md#delete_speechandtextanalytics_sentimentfeedback_sentiment_feedback_id) | **DELETE** /api/v2/speechandtextanalytics/sentimentfeedback/{sentimentFeedbackId} | Delete a Speech & Text Analytics SentimentFeedback by Id
[**delete_speechandtextanalytics_topic**](SpeechTextAnalyticsApi.md#delete_speechandtextanalytics_topic) | **DELETE** /api/v2/speechandtextanalytics/topics/{topicId} | Delete a Speech & Text Analytics topic by id
[**get_speechandtextanalytics_conversation**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_conversation) | **GET** /api/v2/speechandtextanalytics/conversations/{conversationId} | Get Speech and Text Analytics for a specific conversation
[**get_speechandtextanalytics_conversation_communication_transcripturl**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_conversation_communication_transcripturl) | **GET** /api/v2/speechandtextanalytics/conversations/{conversationId}/communications/{communicationId}/transcripturl | Get the pre-signed S3 URL for the transcript of a specific communication of a conversation
[**get_speechandtextanalytics_program**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_program) | **GET** /api/v2/speechandtextanalytics/programs/{programId} | Get a Speech & Text Analytics program by id
[**get_speechandtextanalytics_program_mappings**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_program_mappings) | **GET** /api/v2/speechandtextanalytics/programs/{programId}/mappings | Get Speech & Text Analytics program mappings to queues and flows by id
[**get_speechandtextanalytics_programs**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_programs) | **GET** /api/v2/speechandtextanalytics/programs | Get the list of Speech & Text Analytics programs
[**get_speechandtextanalytics_programs_general_job**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_programs_general_job) | **GET** /api/v2/speechandtextanalytics/programs/general/jobs/{jobId} | Get a Speech & Text Analytics general program job by id
[**get_speechandtextanalytics_programs_mappings**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_programs_mappings) | **GET** /api/v2/speechandtextanalytics/programs/mappings | Get the list of Speech & Text Analytics programs mappings to queues and flows
[**get_speechandtextanalytics_programs_publishjob**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_programs_publishjob) | **GET** /api/v2/speechandtextanalytics/programs/publishjobs/{jobId} | Get a Speech & Text Analytics publish programs job by id
[**get_speechandtextanalytics_programs_unpublished**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_programs_unpublished) | **GET** /api/v2/speechandtextanalytics/programs/unpublished | Get the list of Speech & Text Analytics unpublished programs
[**get_speechandtextanalytics_sentiment_dialects**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_sentiment_dialects) | **GET** /api/v2/speechandtextanalytics/sentiment/dialects | Get the list of Speech & Text Analytics sentiment supported dialects
[**get_speechandtextanalytics_sentimentfeedback**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_sentimentfeedback) | **GET** /api/v2/speechandtextanalytics/sentimentfeedback | Get the list of Speech & Text Analytics SentimentFeedback
[**get_speechandtextanalytics_settings**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_settings) | **GET** /api/v2/speechandtextanalytics/settings | Get Speech And Text Analytics Settings
[**get_speechandtextanalytics_topic**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_topic) | **GET** /api/v2/speechandtextanalytics/topics/{topicId} | Get a Speech & Text Analytics topic by id
[**get_speechandtextanalytics_topics**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_topics) | **GET** /api/v2/speechandtextanalytics/topics | Get the list of Speech & Text Analytics topics
[**get_speechandtextanalytics_topics_dialects**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_topics_dialects) | **GET** /api/v2/speechandtextanalytics/topics/dialects | Get list of supported Speech & Text Analytics topics dialects
[**get_speechandtextanalytics_topics_general**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_topics_general) | **GET** /api/v2/speechandtextanalytics/topics/general | Get the Speech & Text Analytics general topics for a given dialect
[**get_speechandtextanalytics_topics_publishjob**](SpeechTextAnalyticsApi.md#get_speechandtextanalytics_topics_publishjob) | **GET** /api/v2/speechandtextanalytics/topics/publishjobs/{jobId} | Get a Speech & Text Analytics publish topics job by id
[**patch_speechandtextanalytics_settings**](SpeechTextAnalyticsApi.md#patch_speechandtextanalytics_settings) | **PATCH** /api/v2/speechandtextanalytics/settings | Patch Speech And Text Analytics Settings
[**post_speechandtextanalytics_programs**](SpeechTextAnalyticsApi.md#post_speechandtextanalytics_programs) | **POST** /api/v2/speechandtextanalytics/programs | Create new Speech & Text Analytics program
[**post_speechandtextanalytics_programs_general_jobs**](SpeechTextAnalyticsApi.md#post_speechandtextanalytics_programs_general_jobs) | **POST** /api/v2/speechandtextanalytics/programs/general/jobs | Create new Speech & Text Analytics general program job
[**post_speechandtextanalytics_programs_publishjobs**](SpeechTextAnalyticsApi.md#post_speechandtextanalytics_programs_publishjobs) | **POST** /api/v2/speechandtextanalytics/programs/publishjobs | Create new Speech & Text Analytics publish programs job
[**post_speechandtextanalytics_sentimentfeedback**](SpeechTextAnalyticsApi.md#post_speechandtextanalytics_sentimentfeedback) | **POST** /api/v2/speechandtextanalytics/sentimentfeedback | Create a Speech & Text Analytics SentimentFeedback
[**post_speechandtextanalytics_topics**](SpeechTextAnalyticsApi.md#post_speechandtextanalytics_topics) | **POST** /api/v2/speechandtextanalytics/topics | Create new Speech & Text Analytics topic
[**post_speechandtextanalytics_topics_publishjobs**](SpeechTextAnalyticsApi.md#post_speechandtextanalytics_topics_publishjobs) | **POST** /api/v2/speechandtextanalytics/topics/publishjobs | Create new Speech & Text Analytics publish topics job
[**post_speechandtextanalytics_transcripts_search**](SpeechTextAnalyticsApi.md#post_speechandtextanalytics_transcripts_search) | **POST** /api/v2/speechandtextanalytics/transcripts/search | Search resources.
[**put_speechandtextanalytics_program**](SpeechTextAnalyticsApi.md#put_speechandtextanalytics_program) | **PUT** /api/v2/speechandtextanalytics/programs/{programId} | Update existing Speech & Text Analytics program
[**put_speechandtextanalytics_program_mappings**](SpeechTextAnalyticsApi.md#put_speechandtextanalytics_program_mappings) | **PUT** /api/v2/speechandtextanalytics/programs/{programId}/mappings | Set Speech & Text Analytics program mappings to queues and flows
[**put_speechandtextanalytics_settings**](SpeechTextAnalyticsApi.md#put_speechandtextanalytics_settings) | **PUT** /api/v2/speechandtextanalytics/settings | Update Speech And Text Analytics Settings
[**put_speechandtextanalytics_topic**](SpeechTextAnalyticsApi.md#put_speechandtextanalytics_topic) | **PUT** /api/v2/speechandtextanalytics/topics/{topicId} | Update existing Speech & Text Analytics topic



## delete_speechandtextanalytics_program

> delete_speechandtextanalytics_program(program_id, force_delete)
Delete a Speech & Text Analytics program by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The id of the program | [required] |
**force_delete** | Option<**bool**> | Indicates whether the program is forced to be deleted or not. Required when the program to delete is the default program. |  |[default to false]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_speechandtextanalytics_sentimentfeedback

> delete_speechandtextanalytics_sentimentfeedback()
Delete All Speech & Text Analytics SentimentFeedback

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


## delete_speechandtextanalytics_sentimentfeedback_sentiment_feedback_id

> delete_speechandtextanalytics_sentimentfeedback_sentiment_feedback_id(sentiment_feedback_id)
Delete a Speech & Text Analytics SentimentFeedback by Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sentiment_feedback_id** | **String** | The Id of the SentimentFeedback | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_speechandtextanalytics_topic

> delete_speechandtextanalytics_topic(topic_id)
Delete a Speech & Text Analytics topic by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_id** | **String** | The id of the topic | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_conversation

> crate::models::ConversationMetrics get_speechandtextanalytics_conversation(conversation_id)
Get Speech and Text Analytics for a specific conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation Id | [required] |

### Return type

[**crate::models::ConversationMetrics**](ConversationMetrics.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_conversation_communication_transcripturl

> crate::models::TranscriptUrl get_speechandtextanalytics_conversation_communication_transcripturl(conversation_id, communication_id)
Get the pre-signed S3 URL for the transcript of a specific communication of a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**communication_id** | **String** | Communication ID | [required] |

### Return type

[**crate::models::TranscriptUrl**](TranscriptUrl.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_program

> crate::models::Program get_speechandtextanalytics_program(program_id)
Get a Speech & Text Analytics program by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The id of the program | [required] |

### Return type

[**crate::models::Program**](Program.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_program_mappings

> crate::models::ProgramMappings get_speechandtextanalytics_program_mappings(program_id)
Get Speech & Text Analytics program mappings to queues and flows by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The id of the program | [required] |

### Return type

[**crate::models::ProgramMappings**](ProgramMappings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_programs

> crate::models::ProgramsEntityListing get_speechandtextanalytics_programs(next_page, page_size)
Get the list of Speech & Text Analytics programs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**next_page** | Option<**String**> | The key for listing the next page |  |
**page_size** | Option<**i32**> | The page size for the listing |  |[default to 20]

### Return type

[**crate::models::ProgramsEntityListing**](ProgramsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_programs_general_job

> crate::models::GeneralProgramJob get_speechandtextanalytics_programs_general_job(job_id)
Get a Speech & Text Analytics general program job by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id of the publish programs job | [required] |

### Return type

[**crate::models::GeneralProgramJob**](GeneralProgramJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_programs_mappings

> crate::models::ProgramsMappingsEntityListing get_speechandtextanalytics_programs_mappings(next_page, page_size)
Get the list of Speech & Text Analytics programs mappings to queues and flows

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**next_page** | Option<**String**> | The key for listing the next page |  |
**page_size** | Option<**i32**> | The page size for the listing |  |[default to 20]

### Return type

[**crate::models::ProgramsMappingsEntityListing**](ProgramsMappingsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_programs_publishjob

> crate::models::ProgramJob get_speechandtextanalytics_programs_publishjob(job_id)
Get a Speech & Text Analytics publish programs job by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id of the publish programs job | [required] |

### Return type

[**crate::models::ProgramJob**](ProgramJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_programs_unpublished

> crate::models::UnpublishedProgramsEntityListing get_speechandtextanalytics_programs_unpublished(next_page, page_size)
Get the list of Speech & Text Analytics unpublished programs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**next_page** | Option<**String**> | The key for listing the next page |  |
**page_size** | Option<**i32**> | The page size for the listing |  |[default to 20]

### Return type

[**crate::models::UnpublishedProgramsEntityListing**](UnpublishedProgramsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_sentiment_dialects

> crate::models::EntityListing get_speechandtextanalytics_sentiment_dialects()
Get the list of Speech & Text Analytics sentiment supported dialects

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EntityListing**](EntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_sentimentfeedback

> crate::models::SentimentFeedbackEntityListing get_speechandtextanalytics_sentimentfeedback(dialect)
Get the list of Speech & Text Analytics SentimentFeedback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dialect** | Option<**String**> | The key for filter the listing by dialect, dialect format is {language}-{country} where language follows ISO 639-1 standard and country follows ISO 3166-1 alpha 2 standard |  |

### Return type

[**crate::models::SentimentFeedbackEntityListing**](SentimentFeedbackEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_settings

> crate::models::SpeechTextAnalyticsSettingsResponse get_speechandtextanalytics_settings()
Get Speech And Text Analytics Settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SpeechTextAnalyticsSettingsResponse**](SpeechTextAnalyticsSettingsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_topic

> crate::models::Topic get_speechandtextanalytics_topic(topic_id)
Get a Speech & Text Analytics topic by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_id** | **String** | The id of the topic | [required] |

### Return type

[**crate::models::Topic**](Topic.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_topics

> crate::models::TopicsEntityListing get_speechandtextanalytics_topics(next_page, page_size, state, name, ids, sort_by, sort_order)
Get the list of Speech & Text Analytics topics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**next_page** | Option<**String**> | The key for listing the next page |  |
**page_size** | Option<**i32**> | The page size for the listing |  |[default to 20]
**state** | Option<**String**> | Topic state. Defaults to latest |  |
**name** | Option<**String**> | Case insensitive partial name to filter by |  |
**ids** | Option<[**Vec<String>**](String.md)> | Comma separated Topic IDs to filter by. Cannot be used with other filters. Maximum of 50 IDs allowed. |  |
**sort_by** | Option<**String**> | Sort results by. Defaults to name |  |
**sort_order** | Option<**String**> | Sort order. Defaults to asc |  |

### Return type

[**crate::models::TopicsEntityListing**](TopicsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_topics_dialects

> crate::models::EntityListing get_speechandtextanalytics_topics_dialects()
Get list of supported Speech & Text Analytics topics dialects

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EntityListing**](EntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_topics_general

> crate::models::GeneralTopicsEntityListing get_speechandtextanalytics_topics_general(dialect)
Get the Speech & Text Analytics general topics for a given dialect

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dialect** | Option<**String**> | The dialect of the general topics, dialect format is {language}-{country} where language follows ISO 639-1 standard and country follows ISO 3166-1 alpha 2 standard |  |

### Return type

[**crate::models::GeneralTopicsEntityListing**](GeneralTopicsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_speechandtextanalytics_topics_publishjob

> crate::models::TopicJob get_speechandtextanalytics_topics_publishjob(job_id)
Get a Speech & Text Analytics publish topics job by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | The id of the publish topics job | [required] |

### Return type

[**crate::models::TopicJob**](TopicJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_speechandtextanalytics_settings

> crate::models::SpeechTextAnalyticsSettingsResponse patch_speechandtextanalytics_settings(body)
Patch Speech And Text Analytics Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SpeechTextAnalyticsSettingsRequest**](SpeechTextAnalyticsSettingsRequest.md) | Speech And Text Analytics Settings | [required] |

### Return type

[**crate::models::SpeechTextAnalyticsSettingsResponse**](SpeechTextAnalyticsSettingsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_speechandtextanalytics_programs

> crate::models::Program post_speechandtextanalytics_programs(body)
Create new Speech & Text Analytics program

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ProgramRequest**](ProgramRequest.md) | The program to create | [required] |

### Return type

[**crate::models::Program**](Program.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_speechandtextanalytics_programs_general_jobs

> crate::models::GeneralProgramJob post_speechandtextanalytics_programs_general_jobs(body)
Create new Speech & Text Analytics general program job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GeneralProgramJobRequest**](GeneralProgramJobRequest.md) | The general programs job to create | [required] |

### Return type

[**crate::models::GeneralProgramJob**](GeneralProgramJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_speechandtextanalytics_programs_publishjobs

> crate::models::ProgramJob post_speechandtextanalytics_programs_publishjobs(body)
Create new Speech & Text Analytics publish programs job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ProgramJobRequest**](ProgramJobRequest.md) | The publish programs job to create | [required] |

### Return type

[**crate::models::ProgramJob**](ProgramJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_speechandtextanalytics_sentimentfeedback

> crate::models::SentimentFeedback post_speechandtextanalytics_sentimentfeedback(body)
Create a Speech & Text Analytics SentimentFeedback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SentimentFeedback**](SentimentFeedback.md) | The SentimentFeedback to create | [required] |

### Return type

[**crate::models::SentimentFeedback**](SentimentFeedback.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_speechandtextanalytics_topics

> crate::models::Topic post_speechandtextanalytics_topics(body)
Create new Speech & Text Analytics topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TopicRequest**](TopicRequest.md) | The topic to create | [required] |

### Return type

[**crate::models::Topic**](Topic.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_speechandtextanalytics_topics_publishjobs

> crate::models::TopicJob post_speechandtextanalytics_topics_publishjobs(body)
Create new Speech & Text Analytics publish topics job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TopicJobRequest**](TopicJobRequest.md) | The publish topics job to create | [required] |

### Return type

[**crate::models::TopicJob**](TopicJob.md)

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


## put_speechandtextanalytics_program

> crate::models::Program put_speechandtextanalytics_program(program_id, body)
Update existing Speech & Text Analytics program

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The id of the program | [required] |
**body** | [**ProgramRequest**](ProgramRequest.md) | The program to update | [required] |

### Return type

[**crate::models::Program**](Program.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_speechandtextanalytics_program_mappings

> crate::models::ProgramMappings put_speechandtextanalytics_program_mappings(program_id, body)
Set Speech & Text Analytics program mappings to queues and flows

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**program_id** | **String** | The id of the program | [required] |
**body** | [**ProgramMappingsRequest**](ProgramMappingsRequest.md) | The program to set mappings for | [required] |

### Return type

[**crate::models::ProgramMappings**](ProgramMappings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_speechandtextanalytics_settings

> crate::models::SpeechTextAnalyticsSettingsResponse put_speechandtextanalytics_settings(body)
Update Speech And Text Analytics Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SpeechTextAnalyticsSettingsRequest**](SpeechTextAnalyticsSettingsRequest.md) | Speech And Text Analytics Settings | [required] |

### Return type

[**crate::models::SpeechTextAnalyticsSettingsResponse**](SpeechTextAnalyticsSettingsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_speechandtextanalytics_topic

> crate::models::Topic put_speechandtextanalytics_topic(topic_id, body)
Update existing Speech & Text Analytics topic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic_id** | **String** | The id of the topic | [required] |
**body** | [**TopicRequest**](TopicRequest.md) | The topic to update | [required] |

### Return type

[**crate::models::Topic**](Topic.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

