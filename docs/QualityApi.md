# \QualityApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_quality_calibration**](QualityApi.md#delete_quality_calibration) | **DELETE** /api/v2/quality/calibrations/{calibrationId} | Delete a calibration by id.
[**delete_quality_conversation_evaluation**](QualityApi.md#delete_quality_conversation_evaluation) | **DELETE** /api/v2/quality/conversations/{conversationId}/evaluations/{evaluationId} | Delete an evaluation
[**delete_quality_form**](QualityApi.md#delete_quality_form) | **DELETE** /api/v2/quality/forms/{formId} | Delete an evaluation form.
[**delete_quality_forms_evaluation**](QualityApi.md#delete_quality_forms_evaluation) | **DELETE** /api/v2/quality/forms/evaluations/{formId} | Delete an evaluation form.
[**delete_quality_forms_survey**](QualityApi.md#delete_quality_forms_survey) | **DELETE** /api/v2/quality/forms/surveys/{formId} | Delete a survey form.
[**get_quality_agents_activity**](QualityApi.md#get_quality_agents_activity) | **GET** /api/v2/quality/agents/activity | Gets a list of Agent Activities
[**get_quality_calibration**](QualityApi.md#get_quality_calibration) | **GET** /api/v2/quality/calibrations/{calibrationId} | Get a calibration by id.  Requires either calibrator id or conversation id
[**get_quality_calibrations**](QualityApi.md#get_quality_calibrations) | **GET** /api/v2/quality/calibrations | Get the list of calibrations
[**get_quality_conversation_evaluation**](QualityApi.md#get_quality_conversation_evaluation) | **GET** /api/v2/quality/conversations/{conversationId}/evaluations/{evaluationId} | Get an evaluation
[**get_quality_conversation_surveys**](QualityApi.md#get_quality_conversation_surveys) | **GET** /api/v2/quality/conversations/{conversationId}/surveys | Get the surveys for a conversation
[**get_quality_conversations_audits_query_transaction_id**](QualityApi.md#get_quality_conversations_audits_query_transaction_id) | **GET** /api/v2/quality/conversations/audits/query/{transactionId} | Get status of audit query execution
[**get_quality_conversations_audits_query_transaction_id_results**](QualityApi.md#get_quality_conversations_audits_query_transaction_id_results) | **GET** /api/v2/quality/conversations/audits/query/{transactionId}/results | Get results of audit query
[**get_quality_evaluations_query**](QualityApi.md#get_quality_evaluations_query) | **GET** /api/v2/quality/evaluations/query | Queries Evaluations and returns a paged list
[**get_quality_evaluators_activity**](QualityApi.md#get_quality_evaluators_activity) | **GET** /api/v2/quality/evaluators/activity | Get an evaluator activity
[**get_quality_form**](QualityApi.md#get_quality_form) | **GET** /api/v2/quality/forms/{formId} | Get an evaluation form
[**get_quality_form_versions**](QualityApi.md#get_quality_form_versions) | **GET** /api/v2/quality/forms/{formId}/versions | Gets all the revisions for a specific evaluation.
[**get_quality_forms**](QualityApi.md#get_quality_forms) | **GET** /api/v2/quality/forms | Get the list of evaluation forms
[**get_quality_forms_evaluation**](QualityApi.md#get_quality_forms_evaluation) | **GET** /api/v2/quality/forms/evaluations/{formId} | Get an evaluation form
[**get_quality_forms_evaluation_versions**](QualityApi.md#get_quality_forms_evaluation_versions) | **GET** /api/v2/quality/forms/evaluations/{formId}/versions | Gets all the revisions for a specific evaluation.
[**get_quality_forms_evaluations**](QualityApi.md#get_quality_forms_evaluations) | **GET** /api/v2/quality/forms/evaluations | Get the list of evaluation forms
[**get_quality_forms_evaluations_bulk_contexts**](QualityApi.md#get_quality_forms_evaluations_bulk_contexts) | **GET** /api/v2/quality/forms/evaluations/bulk/contexts | Retrieve a list of the latest published evaluation form versions by context ids
[**get_quality_forms_survey**](QualityApi.md#get_quality_forms_survey) | **GET** /api/v2/quality/forms/surveys/{formId} | Get a survey form
[**get_quality_forms_survey_versions**](QualityApi.md#get_quality_forms_survey_versions) | **GET** /api/v2/quality/forms/surveys/{formId}/versions | Gets all the revisions for a specific survey.
[**get_quality_forms_surveys**](QualityApi.md#get_quality_forms_surveys) | **GET** /api/v2/quality/forms/surveys | Get the list of survey forms
[**get_quality_forms_surveys_bulk**](QualityApi.md#get_quality_forms_surveys_bulk) | **GET** /api/v2/quality/forms/surveys/bulk | Retrieve a list of survey forms by their ids
[**get_quality_forms_surveys_bulk_contexts**](QualityApi.md#get_quality_forms_surveys_bulk_contexts) | **GET** /api/v2/quality/forms/surveys/bulk/contexts | Retrieve a list of the latest form versions by context ids
[**get_quality_publishedform**](QualityApi.md#get_quality_publishedform) | **GET** /api/v2/quality/publishedforms/{formId} | Get the published evaluation forms.
[**get_quality_publishedforms**](QualityApi.md#get_quality_publishedforms) | **GET** /api/v2/quality/publishedforms | Get the published evaluation forms.
[**get_quality_publishedforms_evaluation**](QualityApi.md#get_quality_publishedforms_evaluation) | **GET** /api/v2/quality/publishedforms/evaluations/{formId} | Get the most recent published version of an evaluation form.
[**get_quality_publishedforms_evaluations**](QualityApi.md#get_quality_publishedforms_evaluations) | **GET** /api/v2/quality/publishedforms/evaluations | Get the published evaluation forms.
[**get_quality_publishedforms_survey**](QualityApi.md#get_quality_publishedforms_survey) | **GET** /api/v2/quality/publishedforms/surveys/{formId} | Get the most recent published version of a survey form.
[**get_quality_publishedforms_surveys**](QualityApi.md#get_quality_publishedforms_surveys) | **GET** /api/v2/quality/publishedforms/surveys | Get the published survey forms.
[**get_quality_survey**](QualityApi.md#get_quality_survey) | **GET** /api/v2/quality/surveys/{surveyId} | Get a survey for a conversation
[**get_quality_surveys_scorable**](QualityApi.md#get_quality_surveys_scorable) | **GET** /api/v2/quality/surveys/scorable | Get a survey as an end-customer, for the purposes of scoring it.
[**patch_quality_forms_survey**](QualityApi.md#patch_quality_forms_survey) | **PATCH** /api/v2/quality/forms/surveys/{formId} | Disable a particular version of a survey form and invalidates any invitations that have already been sent to customers using this version of the form.
[**post_analytics_evaluations_aggregates_query**](QualityApi.md#post_analytics_evaluations_aggregates_query) | **POST** /api/v2/analytics/evaluations/aggregates/query | Query for evaluation aggregates
[**post_analytics_surveys_aggregates_query**](QualityApi.md#post_analytics_surveys_aggregates_query) | **POST** /api/v2/analytics/surveys/aggregates/query | Query for survey aggregates
[**post_quality_calibrations**](QualityApi.md#post_quality_calibrations) | **POST** /api/v2/quality/calibrations | Create a calibration
[**post_quality_conversation_evaluations**](QualityApi.md#post_quality_conversation_evaluations) | **POST** /api/v2/quality/conversations/{conversationId}/evaluations | Create an evaluation
[**post_quality_conversations_audits_query**](QualityApi.md#post_quality_conversations_audits_query) | **POST** /api/v2/quality/conversations/audits/query | Create audit query execution
[**post_quality_evaluations_aggregates_query_me**](QualityApi.md#post_quality_evaluations_aggregates_query_me) | **POST** /api/v2/quality/evaluations/aggregates/query/me | Query for evaluation aggregates for the current user
[**post_quality_evaluations_scoring**](QualityApi.md#post_quality_evaluations_scoring) | **POST** /api/v2/quality/evaluations/scoring | Score evaluation
[**post_quality_forms**](QualityApi.md#post_quality_forms) | **POST** /api/v2/quality/forms | Create an evaluation form.
[**post_quality_forms_evaluations**](QualityApi.md#post_quality_forms_evaluations) | **POST** /api/v2/quality/forms/evaluations | Create an evaluation form.
[**post_quality_forms_surveys**](QualityApi.md#post_quality_forms_surveys) | **POST** /api/v2/quality/forms/surveys | Create a survey form.
[**post_quality_publishedforms**](QualityApi.md#post_quality_publishedforms) | **POST** /api/v2/quality/publishedforms | Publish an evaluation form.
[**post_quality_publishedforms_evaluations**](QualityApi.md#post_quality_publishedforms_evaluations) | **POST** /api/v2/quality/publishedforms/evaluations | Publish an evaluation form.
[**post_quality_publishedforms_surveys**](QualityApi.md#post_quality_publishedforms_surveys) | **POST** /api/v2/quality/publishedforms/surveys | Publish a survey form.
[**post_quality_surveys_scoring**](QualityApi.md#post_quality_surveys_scoring) | **POST** /api/v2/quality/surveys/scoring | Score survey
[**put_quality_calibration**](QualityApi.md#put_quality_calibration) | **PUT** /api/v2/quality/calibrations/{calibrationId} | Update a calibration to the specified calibration via PUT.  Editable fields include: evaluators, expertEvaluator, and scoringIndex
[**put_quality_conversation_evaluation**](QualityApi.md#put_quality_conversation_evaluation) | **PUT** /api/v2/quality/conversations/{conversationId}/evaluations/{evaluationId} | Update an evaluation
[**put_quality_form**](QualityApi.md#put_quality_form) | **PUT** /api/v2/quality/forms/{formId} | Update an evaluation form.
[**put_quality_forms_evaluation**](QualityApi.md#put_quality_forms_evaluation) | **PUT** /api/v2/quality/forms/evaluations/{formId} | Update an evaluation form.
[**put_quality_forms_survey**](QualityApi.md#put_quality_forms_survey) | **PUT** /api/v2/quality/forms/surveys/{formId} | Update a survey form.
[**put_quality_surveys_scorable**](QualityApi.md#put_quality_surveys_scorable) | **PUT** /api/v2/quality/surveys/scorable | Update a survey as an end-customer, for the purposes of scoring it.



## delete_quality_calibration

> crate::models::Calibration delete_quality_calibration(calibration_id, calibrator_id)
Delete a calibration by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calibration_id** | **String** | Calibration ID | [required] |
**calibrator_id** | **String** | calibratorId | [required] |

### Return type

[**crate::models::Calibration**](Calibration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_quality_conversation_evaluation

> crate::models::Evaluation delete_quality_conversation_evaluation(conversation_id, evaluation_id, expand)
Delete an evaluation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**evaluation_id** | **String** | evaluationId | [required] |
**expand** | Option<**String**> | evaluatorId |  |

### Return type

[**crate::models::Evaluation**](Evaluation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_quality_form

> delete_quality_form(form_id)
Delete an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_quality_forms_evaluation

> delete_quality_forms_evaluation(form_id)
Delete an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_quality_forms_survey

> delete_quality_forms_survey(form_id)
Delete a survey form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_agents_activity

> crate::models::AgentActivityEntityListing get_quality_agents_activity(page_size, page_number, sort_by, expand, next_page, previous_page, start_time, end_time, agent_user_id, evaluator_user_id, name, group)
Gets a list of Agent Activities

Includes the number of evaluations and average evaluation score. These statistics include released evaluations only when evaluatorUserId is provided. In the absence of evaluatorUserId in the request, the api excludes evaluations which are set to never release for the calculation of evaluation statistics. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**start_time** | Option<**String**> | Start time of agent activity based on assigned date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z |  |
**end_time** | Option<**String**> | End time of agent activity based on assigned date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z |  |
**agent_user_id** | Option<[**Vec<String>**](String.md)> | user id of agent requested |  |
**evaluator_user_id** | Option<**String**> | user id of the evaluator |  |
**name** | Option<**String**> | name |  |
**group** | Option<**String**> | group id |  |

### Return type

[**crate::models::AgentActivityEntityListing**](AgentActivityEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_calibration

> crate::models::Calibration get_quality_calibration(calibration_id, calibrator_id, conversation_id)
Get a calibration by id.  Requires either calibrator id or conversation id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calibration_id** | **String** | Calibration ID | [required] |
**calibrator_id** | Option<**String**> | calibratorId |  |
**conversation_id** | Option<**String**> | conversationId |  |

### Return type

[**crate::models::Calibration**](Calibration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_calibrations

> crate::models::CalibrationEntityListing get_quality_calibrations(calibrator_id, page_size, page_number, sort_by, expand, next_page, previous_page, conversation_id, start_time, end_time)
Get the list of calibrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calibrator_id** | **String** | user id of calibrator | [required] |
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**conversation_id** | Option<**String**> | conversation id |  |
**start_time** | Option<**String**> | Beginning of the calibration query. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z |  |
**end_time** | Option<**String**> | end of the calibration query. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z |  |

### Return type

[**crate::models::CalibrationEntityListing**](CalibrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_conversation_evaluation

> crate::models::Evaluation get_quality_conversation_evaluation(conversation_id, evaluation_id, expand)
Get an evaluation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**evaluation_id** | **String** | evaluationId | [required] |
**expand** | Option<**String**> | agent, evaluator, evaluationForm |  |

### Return type

[**crate::models::Evaluation**](Evaluation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_conversation_surveys

> Vec<crate::models::Survey> get_quality_conversation_surveys(conversation_id)
Get the surveys for a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |

### Return type

[**Vec<crate::models::Survey>**](Survey.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_conversations_audits_query_transaction_id

> crate::models::QualityAuditQueryExecutionStatusResponse get_quality_conversations_audits_query_transaction_id(transaction_id)
Get status of audit query execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | Transaction ID | [required] |

### Return type

[**crate::models::QualityAuditQueryExecutionStatusResponse**](QualityAuditQueryExecutionStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_conversations_audits_query_transaction_id_results

> crate::models::QualityAuditQueryExecutionResultsResponse get_quality_conversations_audits_query_transaction_id_results(transaction_id, cursor, page_size, expand)
Get results of audit query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | Transaction ID | [required] |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::QualityAuditQueryExecutionResultsResponse**](QualityAuditQueryExecutionResultsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_evaluations_query

> crate::models::EvaluationEntityListing get_quality_evaluations_query(page_size, page_number, sort_by, expand, next_page, previous_page, conversation_id, agent_user_id, evaluator_user_id, queue_id, start_time, end_time, evaluation_state, is_released, agent_has_read, expand_answer_total_scores, maximum, sort_order)
Queries Evaluations and returns a paged list

Query params must include one of conversationId, evaluatorUserId, or agentUserId. When querying by agentUserId (and not conversationId or evaluatorUserId), the results are sorted by release date. Evaluations set to 'Never Release' are omitted in this case. When querying by evaluatorUserId or conversationId (including when combined with agentUserId), the results are sorted by assigned date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**conversation_id** | Option<**String**> | conversationId specified |  |
**agent_user_id** | Option<**String**> | user id of the agent |  |
**evaluator_user_id** | Option<**String**> | evaluator user id |  |
**queue_id** | Option<**String**> | queue id |  |
**start_time** | Option<**String**> | start time of the evaluation query |  |
**end_time** | Option<**String**> | end time of the evaluation query |  |
**evaluation_state** | Option<[**Vec<String>**](String.md)> |  |  |
**is_released** | Option<**bool**> | the evaluation has been released |  |
**agent_has_read** | Option<**bool**> | agent has the evaluation |  |
**expand_answer_total_scores** | Option<**bool**> | get the total scores for evaluations |  |
**maximum** | Option<**i32**> | maximum |  |
**sort_order** | Option<**String**> | sort order options for agentUserId or evaluatorUserId query. Valid options are 'a', 'asc', 'ascending', 'd', 'desc', 'descending' |  |

### Return type

[**crate::models::EvaluationEntityListing**](EvaluationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_evaluators_activity

> crate::models::EvaluatorActivityEntityListing get_quality_evaluators_activity(page_size, page_number, sort_by, expand, next_page, previous_page, start_time, end_time, name, permission, group)
Get an evaluator activity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**start_time** | Option<**String**> | The start time specified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z |  |
**end_time** | Option<**String**> | The end time specified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z |  |
**name** | Option<**String**> | Evaluator name |  |
**permission** | Option<[**Vec<String>**](String.md)> | permission strings |  |
**group** | Option<**String**> | group id |  |

### Return type

[**crate::models::EvaluatorActivityEntityListing**](EvaluatorActivityEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_form

> crate::models::EvaluationForm get_quality_form(form_id)
Get an evaluation form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_form_versions

> crate::models::EvaluationFormEntityListing get_quality_form_versions(form_id, page_size, page_number)
Gets all the revisions for a specific evaluation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::EvaluationFormEntityListing**](EvaluationFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms

> crate::models::EvaluationFormEntityListing get_quality_forms(page_size, page_number, sort_by, next_page, previous_page, expand, name, sort_order)
Get the list of evaluation forms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**expand** | Option<**String**> | Expand |  |
**name** | Option<**String**> | Name |  |
**sort_order** | Option<**String**> | Order to sort results, either asc or desc |  |

### Return type

[**crate::models::EvaluationFormEntityListing**](EvaluationFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_evaluation

> crate::models::EvaluationForm get_quality_forms_evaluation(form_id)
Get an evaluation form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_evaluation_versions

> crate::models::EvaluationFormEntityListing get_quality_forms_evaluation_versions(form_id, page_size, page_number, sort_order)
Gets all the revisions for a specific evaluation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]

### Return type

[**crate::models::EvaluationFormEntityListing**](EvaluationFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_evaluations

> crate::models::EvaluationFormEntityListing get_quality_forms_evaluations(page_size, page_number, sort_by, next_page, previous_page, expand, name, sort_order)
Get the list of evaluation forms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**expand** | Option<**String**> | Expand |  |
**name** | Option<**String**> | Name |  |
**sort_order** | Option<**String**> | Order to sort results, either asc or desc |  |

### Return type

[**crate::models::EvaluationFormEntityListing**](EvaluationFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_evaluations_bulk_contexts

> Vec<crate::models::EvaluationForm> get_quality_forms_evaluations_bulk_contexts(context_id)
Retrieve a list of the latest published evaluation form versions by context ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | [**Vec<String>**](String.md) | A comma-delimited list of valid evaluation form context ids | [required] |

### Return type

[**Vec<crate::models::EvaluationForm>**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_survey

> crate::models::SurveyForm get_quality_forms_survey(form_id)
Get a survey form

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

[**crate::models::SurveyForm**](SurveyForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_survey_versions

> crate::models::SurveyFormEntityListing get_quality_forms_survey_versions(form_id, page_size, page_number)
Gets all the revisions for a specific survey.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::SurveyFormEntityListing**](SurveyFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_surveys

> crate::models::SurveyFormEntityListing get_quality_forms_surveys(page_size, page_number, sort_by, next_page, previous_page, expand, name, sort_order)
Get the list of survey forms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**expand** | Option<**String**> | Expand |  |
**name** | Option<**String**> | Name |  |
**sort_order** | Option<**String**> | Order to sort results, either asc or desc |  |

### Return type

[**crate::models::SurveyFormEntityListing**](SurveyFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_surveys_bulk

> crate::models::SurveyFormEntityListing get_quality_forms_surveys_bulk(id)
Retrieve a list of survey forms by their ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | A comma-delimited list of valid survey form ids | [required] |

### Return type

[**crate::models::SurveyFormEntityListing**](SurveyFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_forms_surveys_bulk_contexts

> Vec<crate::models::SurveyForm> get_quality_forms_surveys_bulk_contexts(context_id, published)
Retrieve a list of the latest form versions by context ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_id** | [**Vec<String>**](String.md) | A comma-delimited list of valid survey form context ids | [required] |
**published** | Option<**bool**> | If true, the latest published version will be included. If false, only the unpublished version will be included. |  |[default to true]

### Return type

[**Vec<crate::models::SurveyForm>**](SurveyForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_publishedform

> crate::models::EvaluationForm get_quality_publishedform(form_id)
Get the published evaluation forms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_publishedforms

> crate::models::EvaluationFormEntityListing get_quality_publishedforms(page_size, page_number, name, only_latest_per_context)
Get the published evaluation forms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**only_latest_per_context** | Option<**bool**> | onlyLatestPerContext |  |[default to false]

### Return type

[**crate::models::EvaluationFormEntityListing**](EvaluationFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_publishedforms_evaluation

> crate::models::EvaluationForm get_quality_publishedforms_evaluation(form_id)
Get the most recent published version of an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_publishedforms_evaluations

> crate::models::EvaluationFormEntityListing get_quality_publishedforms_evaluations(page_size, page_number, name, only_latest_per_context)
Get the published evaluation forms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**only_latest_per_context** | Option<**bool**> | onlyLatestPerContext |  |[default to false]

### Return type

[**crate::models::EvaluationFormEntityListing**](EvaluationFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_publishedforms_survey

> crate::models::SurveyForm get_quality_publishedforms_survey(form_id)
Get the most recent published version of a survey form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |

### Return type

[**crate::models::SurveyForm**](SurveyForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_publishedforms_surveys

> crate::models::SurveyFormEntityListing get_quality_publishedforms_surveys(page_size, page_number, name, only_latest_enabled_per_context)
Get the published survey forms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Name |  |
**only_latest_enabled_per_context** | Option<**bool**> | onlyLatestEnabledPerContext |  |[default to false]

### Return type

[**crate::models::SurveyFormEntityListing**](SurveyFormEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_survey

> crate::models::Survey get_quality_survey(survey_id)
Get a survey for a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**survey_id** | **String** | surveyId | [required] |

### Return type

[**crate::models::Survey**](Survey.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quality_surveys_scorable

> crate::models::ScorableSurvey get_quality_surveys_scorable(customer_survey_url)
Get a survey as an end-customer, for the purposes of scoring it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_survey_url** | **String** | customerSurveyUrl | [required] |

### Return type

[**crate::models::ScorableSurvey**](ScorableSurvey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_quality_forms_survey

> crate::models::SurveyForm patch_quality_forms_survey(form_id, body)
Disable a particular version of a survey form and invalidates any invitations that have already been sent to customers using this version of the form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |
**body** | [**SurveyForm**](SurveyForm.md) | Survey form | [required] |

### Return type

[**crate::models::SurveyForm**](SurveyForm.md)

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


## post_quality_calibrations

> crate::models::Calibration post_quality_calibrations(body, expand)
Create a calibration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CalibrationCreate**](CalibrationCreate.md) | calibration | [required] |
**expand** | Option<**String**> | calibratorId |  |

### Return type

[**crate::models::Calibration**](Calibration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_conversation_evaluations

> crate::models::Evaluation post_quality_conversation_evaluations(conversation_id, body, expand)
Create an evaluation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**body** | [**Evaluation**](Evaluation.md) | evaluation | [required] |
**expand** | Option<**String**> | evaluatorId |  |

### Return type

[**crate::models::Evaluation**](Evaluation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_conversations_audits_query

> crate::models::QualityAuditQueryExecutionStatusResponse post_quality_conversations_audits_query(body)
Create audit query execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**QmAuditQueryRequest**](QmAuditQueryRequest.md) | query | [required] |

### Return type

[**crate::models::QualityAuditQueryExecutionStatusResponse**](QualityAuditQueryExecutionStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_evaluations_aggregates_query_me

> crate::models::EvaluationAggregateQueryResponse post_quality_evaluations_aggregates_query_me(body)
Query for evaluation aggregates for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EvaluationAggregationQueryMe**](EvaluationAggregationQueryMe.md) | query | [required] |

### Return type

[**crate::models::EvaluationAggregateQueryResponse**](EvaluationAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_evaluations_scoring

> crate::models::EvaluationScoringSet post_quality_evaluations_scoring(body)
Score evaluation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EvaluationFormAndScoringSet**](EvaluationFormAndScoringSet.md) | evaluationAndScoringSet | [required] |

### Return type

[**crate::models::EvaluationScoringSet**](EvaluationScoringSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_forms

> crate::models::EvaluationForm post_quality_forms(body)
Create an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EvaluationForm**](EvaluationForm.md) | Evaluation form | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_forms_evaluations

> crate::models::EvaluationForm post_quality_forms_evaluations(body)
Create an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EvaluationForm**](EvaluationForm.md) | Evaluation form | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_forms_surveys

> crate::models::SurveyForm post_quality_forms_surveys(body)
Create a survey form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SurveyForm**](SurveyForm.md) | Survey form | [required] |

### Return type

[**crate::models::SurveyForm**](SurveyForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_publishedforms

> crate::models::EvaluationForm post_quality_publishedforms(body)
Publish an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PublishForm**](PublishForm.md) | Publish request containing id of form to publish | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_publishedforms_evaluations

> crate::models::EvaluationForm post_quality_publishedforms_evaluations(body)
Publish an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PublishForm**](PublishForm.md) | Publish request containing id of form to publish | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_publishedforms_surveys

> crate::models::SurveyForm post_quality_publishedforms_surveys(body)
Publish a survey form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PublishForm**](PublishForm.md) | Survey form | [required] |

### Return type

[**crate::models::SurveyForm**](SurveyForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_quality_surveys_scoring

> crate::models::SurveyScoringSet post_quality_surveys_scoring(body)
Score survey

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SurveyFormAndScoringSet**](SurveyFormAndScoringSet.md) | surveyAndScoringSet | [required] |

### Return type

[**crate::models::SurveyScoringSet**](SurveyScoringSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quality_calibration

> crate::models::Calibration put_quality_calibration(calibration_id, body)
Update a calibration to the specified calibration via PUT.  Editable fields include: evaluators, expertEvaluator, and scoringIndex

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calibration_id** | **String** | Calibration ID | [required] |
**body** | [**Calibration**](Calibration.md) | Calibration | [required] |

### Return type

[**crate::models::Calibration**](Calibration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quality_conversation_evaluation

> crate::models::Evaluation put_quality_conversation_evaluation(conversation_id, evaluation_id, body, expand)
Update an evaluation

The quality:evaluation:edit permission allows modification of most fields, while the quality:evaluation:editScore permission allows an evaluator to change just the question scores, and the quality:evaluation:editAgentSignoff permission allows an agent to change the agent comments and sign off on the evaluation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | conversationId | [required] |
**evaluation_id** | **String** | evaluationId | [required] |
**body** | [**Evaluation**](Evaluation.md) | evaluation | [required] |
**expand** | Option<**String**> | evaluatorId |  |

### Return type

[**crate::models::Evaluation**](Evaluation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quality_form

> crate::models::EvaluationForm put_quality_form(form_id, body)
Update an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |
**body** | [**EvaluationForm**](EvaluationForm.md) | Evaluation form | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quality_forms_evaluation

> crate::models::EvaluationForm put_quality_forms_evaluation(form_id, body)
Update an evaluation form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |
**body** | [**EvaluationForm**](EvaluationForm.md) | Evaluation form | [required] |

### Return type

[**crate::models::EvaluationForm**](EvaluationForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quality_forms_survey

> crate::models::SurveyForm put_quality_forms_survey(form_id, body)
Update a survey form.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**form_id** | **String** | Form ID | [required] |
**body** | [**SurveyForm**](SurveyForm.md) | Survey form | [required] |

### Return type

[**crate::models::SurveyForm**](SurveyForm.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_quality_surveys_scorable

> crate::models::ScorableSurvey put_quality_surveys_scorable(customer_survey_url, body)
Update a survey as an end-customer, for the purposes of scoring it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**customer_survey_url** | **String** | customerSurveyUrl | [required] |
**body** | [**ScorableSurvey**](ScorableSurvey.md) | survey | [required] |

### Return type

[**crate::models::ScorableSurvey**](ScorableSurvey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

