# \AnalyticsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_analytics_conversations_details_job**](AnalyticsApi.md#delete_analytics_conversations_details_job) | **DELETE** /api/v2/analytics/conversations/details/jobs/{jobId} | Delete/cancel an async request
[**delete_analytics_reporting_schedule**](AnalyticsApi.md#delete_analytics_reporting_schedule) | **DELETE** /api/v2/analytics/reporting/schedules/{scheduleId} | Delete a scheduled report job.
[**delete_analytics_users_details_job**](AnalyticsApi.md#delete_analytics_users_details_job) | **DELETE** /api/v2/analytics/users/details/jobs/{jobId} | Delete/cancel an async request
[**get_analytics_botflow_reportingturns**](AnalyticsApi.md#get_analytics_botflow_reportingturns) | **GET** /api/v2/analytics/botflows/{botFlowId}/reportingturns | Get Reporting Turns.
[**get_analytics_conversation_details**](AnalyticsApi.md#get_analytics_conversation_details) | **GET** /api/v2/analytics/conversations/{conversationId}/details | Get a conversation by id
[**get_analytics_conversations_details**](AnalyticsApi.md#get_analytics_conversations_details) | **GET** /api/v2/analytics/conversations/details | Gets multiple conversations by id
[**get_analytics_conversations_details_job**](AnalyticsApi.md#get_analytics_conversations_details_job) | **GET** /api/v2/analytics/conversations/details/jobs/{jobId} | Get status for async query for conversation details
[**get_analytics_conversations_details_job_results**](AnalyticsApi.md#get_analytics_conversations_details_job_results) | **GET** /api/v2/analytics/conversations/details/jobs/{jobId}/results | Fetch a page of results for an async query
[**get_analytics_conversations_details_jobs_availability**](AnalyticsApi.md#get_analytics_conversations_details_jobs_availability) | **GET** /api/v2/analytics/conversations/details/jobs/availability | Lookup the datalake availability date and time
[**get_analytics_reporting_exports**](AnalyticsApi.md#get_analytics_reporting_exports) | **GET** /api/v2/analytics/reporting/exports | Get all view export requests for a user
[**get_analytics_reporting_exports_metadata**](AnalyticsApi.md#get_analytics_reporting_exports_metadata) | **GET** /api/v2/analytics/reporting/exports/metadata | Get all export metadata
[**get_analytics_reporting_metadata**](AnalyticsApi.md#get_analytics_reporting_metadata) | **GET** /api/v2/analytics/reporting/metadata | Get list of reporting metadata.
[**get_analytics_reporting_report_id_metadata**](AnalyticsApi.md#get_analytics_reporting_report_id_metadata) | **GET** /api/v2/analytics/reporting/{reportId}/metadata | Get a reporting metadata.
[**get_analytics_reporting_reportformats**](AnalyticsApi.md#get_analytics_reporting_reportformats) | **GET** /api/v2/analytics/reporting/reportformats | Get a list of report formats
[**get_analytics_reporting_schedule**](AnalyticsApi.md#get_analytics_reporting_schedule) | **GET** /api/v2/analytics/reporting/schedules/{scheduleId} | Get a scheduled report job.
[**get_analytics_reporting_schedule_history**](AnalyticsApi.md#get_analytics_reporting_schedule_history) | **GET** /api/v2/analytics/reporting/schedules/{scheduleId}/history | Get list of completed scheduled report jobs.
[**get_analytics_reporting_schedule_history_latest**](AnalyticsApi.md#get_analytics_reporting_schedule_history_latest) | **GET** /api/v2/analytics/reporting/schedules/{scheduleId}/history/latest | Get most recently completed scheduled report job.
[**get_analytics_reporting_schedule_history_run_id**](AnalyticsApi.md#get_analytics_reporting_schedule_history_run_id) | **GET** /api/v2/analytics/reporting/schedules/{scheduleId}/history/{runId} | A completed scheduled report job
[**get_analytics_reporting_schedules**](AnalyticsApi.md#get_analytics_reporting_schedules) | **GET** /api/v2/analytics/reporting/schedules | Get a list of scheduled report jobs
[**get_analytics_reporting_settings**](AnalyticsApi.md#get_analytics_reporting_settings) | **GET** /api/v2/analytics/reporting/settings | Get AnalyticsReportingSettings for an organization
[**get_analytics_reporting_timeperiods**](AnalyticsApi.md#get_analytics_reporting_timeperiods) | **GET** /api/v2/analytics/reporting/timeperiods | Get a list of report time periods.
[**get_analytics_users_details_job**](AnalyticsApi.md#get_analytics_users_details_job) | **GET** /api/v2/analytics/users/details/jobs/{jobId} | Get status for async query for user details
[**get_analytics_users_details_job_results**](AnalyticsApi.md#get_analytics_users_details_job_results) | **GET** /api/v2/analytics/users/details/jobs/{jobId}/results | Fetch a page of results for an async query
[**get_analytics_users_details_jobs_availability**](AnalyticsApi.md#get_analytics_users_details_jobs_availability) | **GET** /api/v2/analytics/users/details/jobs/availability | Lookup the datalake availability date and time
[**patch_analytics_reporting_settings**](AnalyticsApi.md#patch_analytics_reporting_settings) | **PATCH** /api/v2/analytics/reporting/settings | Patch AnalyticsReportingSettings values for an organization
[**post_analytics_bots_aggregates_query**](AnalyticsApi.md#post_analytics_bots_aggregates_query) | **POST** /api/v2/analytics/bots/aggregates/query | Query for bot aggregates
[**post_analytics_conversation_details_properties**](AnalyticsApi.md#post_analytics_conversation_details_properties) | **POST** /api/v2/analytics/conversations/{conversationId}/details/properties | Index conversation properties
[**post_analytics_conversations_aggregates_query**](AnalyticsApi.md#post_analytics_conversations_aggregates_query) | **POST** /api/v2/analytics/conversations/aggregates/query | Query for conversation aggregates
[**post_analytics_conversations_details_jobs**](AnalyticsApi.md#post_analytics_conversations_details_jobs) | **POST** /api/v2/analytics/conversations/details/jobs | Query for conversation details asynchronously
[**post_analytics_conversations_details_query**](AnalyticsApi.md#post_analytics_conversations_details_query) | **POST** /api/v2/analytics/conversations/details/query | Query for conversation details
[**post_analytics_conversations_transcripts_query**](AnalyticsApi.md#post_analytics_conversations_transcripts_query) | **POST** /api/v2/analytics/conversations/transcripts/query | Search resources.
[**post_analytics_evaluations_aggregates_query**](AnalyticsApi.md#post_analytics_evaluations_aggregates_query) | **POST** /api/v2/analytics/evaluations/aggregates/query | Query for evaluation aggregates
[**post_analytics_flows_aggregates_query**](AnalyticsApi.md#post_analytics_flows_aggregates_query) | **POST** /api/v2/analytics/flows/aggregates/query | Query for flow aggregates
[**post_analytics_flows_observations_query**](AnalyticsApi.md#post_analytics_flows_observations_query) | **POST** /api/v2/analytics/flows/observations/query | Query for flow observations
[**post_analytics_journeys_aggregates_query**](AnalyticsApi.md#post_analytics_journeys_aggregates_query) | **POST** /api/v2/analytics/journeys/aggregates/query | Query for journey aggregates
[**post_analytics_queues_observations_query**](AnalyticsApi.md#post_analytics_queues_observations_query) | **POST** /api/v2/analytics/queues/observations/query | Query for queue observations
[**post_analytics_reporting_exports**](AnalyticsApi.md#post_analytics_reporting_exports) | **POST** /api/v2/analytics/reporting/exports | Generate a view export request
[**post_analytics_reporting_schedule_runreport**](AnalyticsApi.md#post_analytics_reporting_schedule_runreport) | **POST** /api/v2/analytics/reporting/schedules/{scheduleId}/runreport | Place a scheduled report immediately into the reporting queue
[**post_analytics_reporting_schedules**](AnalyticsApi.md#post_analytics_reporting_schedules) | **POST** /api/v2/analytics/reporting/schedules | Create a scheduled report job
[**post_analytics_surveys_aggregates_query**](AnalyticsApi.md#post_analytics_surveys_aggregates_query) | **POST** /api/v2/analytics/surveys/aggregates/query | Query for survey aggregates
[**post_analytics_transcripts_aggregates_query**](AnalyticsApi.md#post_analytics_transcripts_aggregates_query) | **POST** /api/v2/analytics/transcripts/aggregates/query | Query for transcript aggregates
[**post_analytics_users_aggregates_query**](AnalyticsApi.md#post_analytics_users_aggregates_query) | **POST** /api/v2/analytics/users/aggregates/query | Query for user aggregates
[**post_analytics_users_details_jobs**](AnalyticsApi.md#post_analytics_users_details_jobs) | **POST** /api/v2/analytics/users/details/jobs | Query for user details asynchronously
[**post_analytics_users_details_query**](AnalyticsApi.md#post_analytics_users_details_query) | **POST** /api/v2/analytics/users/details/query | Query for user details
[**post_analytics_users_observations_query**](AnalyticsApi.md#post_analytics_users_observations_query) | **POST** /api/v2/analytics/users/observations/query | Query for user observations
[**put_analytics_reporting_schedule**](AnalyticsApi.md#put_analytics_reporting_schedule) | **PUT** /api/v2/analytics/reporting/schedules/{scheduleId} | Update a scheduled report job.



## delete_analytics_conversations_details_job

> delete_analytics_conversations_details_job(job_id)
Delete/cancel an async request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_analytics_reporting_schedule

> delete_analytics_reporting_schedule(schedule_id)
Delete a scheduled report job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_analytics_users_details_job

> delete_analytics_users_details_job(job_id)
Delete/cancel an async request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_botflow_reportingturns

> crate::models::ReportingTurnsResponse get_analytics_botflow_reportingturns(bot_flow_id, after, page_size, action_id, session_id)
Get Reporting Turns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_flow_id** | **String** | ID of the bot flow. | [required] |
**after** | Option<**String**> | The cursor that points to the ID of the last item in the list of entities that has been returned. |  |
**page_size** | Option<**String**> | Max number of entities to return. Maximum of 250 |  |[default to 50]
**action_id** | Option<**String**> | Optional action ID to get the reporting turns associated to a particular flow action |  |
**session_id** | Option<**String**> | Optional session ID to get the reporting turns for a particular session |  |

### Return type

[**crate::models::ReportingTurnsResponse**](ReportingTurnsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversation_details

> crate::models::AnalyticsConversationWithoutAttributes get_analytics_conversation_details(conversation_id)
Get a conversation by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**crate::models::AnalyticsConversationWithoutAttributes**](AnalyticsConversationWithoutAttributes.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details

> crate::models::AnalyticsConversationWithoutAttributesMultiGetResponse get_analytics_conversations_details(id)
Gets multiple conversations by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**Vec<String>**](String.md)> | Comma-separated conversation ids |  |

### Return type

[**crate::models::AnalyticsConversationWithoutAttributesMultiGetResponse**](AnalyticsConversationWithoutAttributesMultiGetResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details_job

> crate::models::AsyncQueryStatus get_analytics_conversations_details_job(job_id)
Get status for async query for conversation details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

[**crate::models::AsyncQueryStatus**](AsyncQueryStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details_job_results

> crate::models::AnalyticsConversationAsyncQueryResponse get_analytics_conversations_details_job_results(job_id, cursor, page_size)
Fetch a page of results for an async query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |
**page_size** | Option<**i32**> | The desired maximum number of results |  |

### Return type

[**crate::models::AnalyticsConversationAsyncQueryResponse**](AnalyticsConversationAsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_conversations_details_jobs_availability

> crate::models::DataAvailabilityResponse get_analytics_conversations_details_jobs_availability()
Lookup the datalake availability date and time

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataAvailabilityResponse**](DataAvailabilityResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_exports

> crate::models::ReportingExportJobListing get_analytics_reporting_exports(page_number, page_size)
Get all view export requests for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::ReportingExportJobListing**](ReportingExportJobListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_exports_metadata

> crate::models::ReportingExportMetadataJobListing get_analytics_reporting_exports_metadata()
Get all export metadata

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ReportingExportMetadataJobListing**](ReportingExportMetadataJobListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_metadata

> crate::models::ReportMetaDataEntityListing get_analytics_reporting_metadata(page_number, page_size, locale)
Get list of reporting metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**locale** | Option<**String**> | Locale |  |

### Return type

[**crate::models::ReportMetaDataEntityListing**](ReportMetaDataEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_report_id_metadata

> crate::models::ReportMetaData get_analytics_reporting_report_id_metadata(report_id, locale)
Get a reporting metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**report_id** | **String** | Report ID | [required] |
**locale** | Option<**String**> | Locale |  |

### Return type

[**crate::models::ReportMetaData**](ReportMetaData.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_reportformats

> Vec<String> get_analytics_reporting_reportformats()
Get a list of report formats

Get a list of report formats.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_schedule

> crate::models::ReportSchedule get_analytics_reporting_schedule(schedule_id)
Get a scheduled report job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |

### Return type

[**crate::models::ReportSchedule**](ReportSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_schedule_history

> crate::models::ReportRunEntryEntityDomainListing get_analytics_reporting_schedule_history(schedule_id, page_number, page_size)
Get list of completed scheduled report jobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |
**page_number** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> |  |  |[default to 25]

### Return type

[**crate::models::ReportRunEntryEntityDomainListing**](ReportRunEntryEntityDomainListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_schedule_history_latest

> crate::models::ReportRunEntry get_analytics_reporting_schedule_history_latest(schedule_id)
Get most recently completed scheduled report job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |

### Return type

[**crate::models::ReportRunEntry**](ReportRunEntry.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_schedule_history_run_id

> crate::models::ReportRunEntry get_analytics_reporting_schedule_history_run_id(run_id, schedule_id)
A completed scheduled report job

A completed scheduled report job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**run_id** | **String** | Run ID | [required] |
**schedule_id** | **String** | Schedule ID | [required] |

### Return type

[**crate::models::ReportRunEntry**](ReportRunEntry.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_schedules

> crate::models::ReportScheduleEntityListing get_analytics_reporting_schedules(page_number, page_size)
Get a list of scheduled report jobs

Get a list of scheduled report jobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::ReportScheduleEntityListing**](ReportScheduleEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_settings

> crate::models::AnalyticsReportingSettings get_analytics_reporting_settings()
Get AnalyticsReportingSettings for an organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AnalyticsReportingSettings**](AnalyticsReportingSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_reporting_timeperiods

> Vec<String> get_analytics_reporting_timeperiods()
Get a list of report time periods.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_users_details_job

> crate::models::AsyncQueryStatus get_analytics_users_details_job(job_id)
Get status for async query for user details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

[**crate::models::AsyncQueryStatus**](AsyncQueryStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_users_details_job_results

> crate::models::AnalyticsUserDetailsAsyncQueryResponse get_analytics_users_details_job_results(job_id, cursor, page_size)
Fetch a page of results for an async query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |
**page_size** | Option<**i32**> | The desired maximum number of results |  |

### Return type

[**crate::models::AnalyticsUserDetailsAsyncQueryResponse**](AnalyticsUserDetailsAsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_users_details_jobs_availability

> crate::models::DataAvailabilityResponse get_analytics_users_details_jobs_availability()
Lookup the datalake availability date and time

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataAvailabilityResponse**](DataAvailabilityResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_analytics_reporting_settings

> crate::models::AnalyticsReportingSettings patch_analytics_reporting_settings(body)
Patch AnalyticsReportingSettings values for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AnalyticsReportingSettings**](AnalyticsReportingSettings.md) | AnalyticsReportingSettingsRequest | [required] |

### Return type

[**crate::models::AnalyticsReportingSettings**](AnalyticsReportingSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_bots_aggregates_query

> crate::models::BotAggregateQueryResponse post_analytics_bots_aggregates_query(body)
Query for bot aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BotAggregationQuery**](BotAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::BotAggregateQueryResponse**](BotAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversation_details_properties

> crate::models::PropertyIndexRequest post_analytics_conversation_details_properties(conversation_id, body)
Index conversation properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**PropertyIndexRequest**](PropertyIndexRequest.md) | request | [required] |

### Return type

[**crate::models::PropertyIndexRequest**](PropertyIndexRequest.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversations_aggregates_query

> crate::models::ConversationAggregateQueryResponse post_analytics_conversations_aggregates_query(body)
Query for conversation aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConversationAggregationQuery**](ConversationAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::ConversationAggregateQueryResponse**](ConversationAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversations_details_jobs

> crate::models::AsyncQueryResponse post_analytics_conversations_details_jobs(body)
Query for conversation details asynchronously

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AsyncConversationQuery**](AsyncConversationQuery.md) | query | [required] |

### Return type

[**crate::models::AsyncQueryResponse**](AsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_conversations_details_query

> crate::models::AnalyticsConversationQueryResponse post_analytics_conversations_details_query(body)
Query for conversation details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ConversationQuery**](ConversationQuery.md) | query | [required] |

### Return type

[**crate::models::AnalyticsConversationQueryResponse**](AnalyticsConversationQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
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


## post_analytics_evaluations_aggregates_query

> crate::models::EvaluationAggregateQueryResponse post_analytics_evaluations_aggregates_query(body)
Query for evaluation aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EvaluationAggregationQuery**](EvaluationAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::EvaluationAggregateQueryResponse**](EvaluationAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_flows_aggregates_query

> crate::models::FlowAggregateQueryResponse post_analytics_flows_aggregates_query(body)
Query for flow aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FlowAggregationQuery**](FlowAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::FlowAggregateQueryResponse**](FlowAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_flows_observations_query

> crate::models::FlowObservationQueryResponse post_analytics_flows_observations_query(body)
Query for flow observations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**FlowObservationQuery**](FlowObservationQuery.md) | query | [required] |

### Return type

[**crate::models::FlowObservationQueryResponse**](FlowObservationQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_journeys_aggregates_query

> crate::models::JourneyAggregateQueryResponse post_analytics_journeys_aggregates_query(body)
Query for journey aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**JourneyAggregationQuery**](JourneyAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::JourneyAggregateQueryResponse**](JourneyAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_queues_observations_query

> crate::models::QueueObservationQueryResponse post_analytics_queues_observations_query(body)
Query for queue observations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**QueueObservationQuery**](QueueObservationQuery.md) | query | [required] |

### Return type

[**crate::models::QueueObservationQueryResponse**](QueueObservationQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_reporting_exports

> crate::models::ReportingExportJobResponse post_analytics_reporting_exports(body)
Generate a view export request

This API creates a reporting export but the desired way to export analytics data is to use the analytics query APIs instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ReportingExportJobRequest**](ReportingExportJobRequest.md) | ReportingExportJobRequest | [required] |

### Return type

[**crate::models::ReportingExportJobResponse**](ReportingExportJobResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_reporting_schedule_runreport

> crate::models::RunNowResponse post_analytics_reporting_schedule_runreport(schedule_id)
Place a scheduled report immediately into the reporting queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |

### Return type

[**crate::models::RunNowResponse**](RunNowResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_reporting_schedules

> crate::models::ReportSchedule post_analytics_reporting_schedules(body)
Create a scheduled report job

Create a scheduled report job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ReportSchedule**](ReportSchedule.md) | ReportSchedule | [required] |

### Return type

[**crate::models::ReportSchedule**](ReportSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_surveys_aggregates_query

> crate::models::SurveyAggregateQueryResponse post_analytics_surveys_aggregates_query(body)
Query for survey aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SurveyAggregationQuery**](SurveyAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::SurveyAggregateQueryResponse**](SurveyAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_transcripts_aggregates_query

> crate::models::TranscriptAggregateQueryResponse post_analytics_transcripts_aggregates_query(body)
Query for transcript aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TranscriptAggregationQuery**](TranscriptAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::TranscriptAggregateQueryResponse**](TranscriptAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_aggregates_query

> crate::models::UserAggregateQueryResponse post_analytics_users_aggregates_query(body)
Query for user aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserAggregationQuery**](UserAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::UserAggregateQueryResponse**](UserAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_details_jobs

> crate::models::AsyncQueryResponse post_analytics_users_details_jobs(body)
Query for user details asynchronously

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AsyncUserDetailsQuery**](AsyncUserDetailsQuery.md) | query | [required] |

### Return type

[**crate::models::AsyncQueryResponse**](AsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_details_query

> crate::models::AnalyticsUserDetailsQueryResponse post_analytics_users_details_query(body)
Query for user details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserDetailsQuery**](UserDetailsQuery.md) | query | [required] |

### Return type

[**crate::models::AnalyticsUserDetailsQueryResponse**](AnalyticsUserDetailsQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_observations_query

> crate::models::UserObservationQueryResponse post_analytics_users_observations_query(body)
Query for user observations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserObservationQuery**](UserObservationQuery.md) | query | [required] |

### Return type

[**crate::models::UserObservationQueryResponse**](UserObservationQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_analytics_reporting_schedule

> crate::models::ReportSchedule put_analytics_reporting_schedule(schedule_id, body)
Update a scheduled report job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |
**body** | [**ReportSchedule**](ReportSchedule.md) | ReportSchedule | [required] |

### Return type

[**crate::models::ReportSchedule**](ReportSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

