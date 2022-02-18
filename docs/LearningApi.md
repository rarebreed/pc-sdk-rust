# \LearningApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_learning_assignment**](LearningApi.md#delete_learning_assignment) | **DELETE** /api/v2/learning/assignments/{assignmentId} | Delete a learning assignment
[**delete_learning_module**](LearningApi.md#delete_learning_module) | **DELETE** /api/v2/learning/modules/{moduleId} | Delete a learning module
[**get_learning_assignment**](LearningApi.md#get_learning_assignment) | **GET** /api/v2/learning/assignments/{assignmentId} | Get Learning Assignment
[**get_learning_assignments**](LearningApi.md#get_learning_assignments) | **GET** /api/v2/learning/assignments | List of Learning module Assignments
[**get_learning_assignments_me**](LearningApi.md#get_learning_assignments_me) | **GET** /api/v2/learning/assignments/me | List of Learning Assignments assigned to current user
[**get_learning_module**](LearningApi.md#get_learning_module) | **GET** /api/v2/learning/modules/{moduleId} | Get a learning module
[**get_learning_module_rule**](LearningApi.md#get_learning_module_rule) | **GET** /api/v2/learning/modules/{moduleId}/rule | Get a learning module rule
[**get_learning_module_version**](LearningApi.md#get_learning_module_version) | **GET** /api/v2/learning/modules/{moduleId}/versions/{versionId} | Get specific version of a published module
[**get_learning_modules**](LearningApi.md#get_learning_modules) | **GET** /api/v2/learning/modules | Get all learning modules of an organization
[**patch_learning_assignment**](LearningApi.md#patch_learning_assignment) | **PATCH** /api/v2/learning/assignments/{assignmentId} | Update Learning Assignment
[**post_learning_assessments_scoring**](LearningApi.md#post_learning_assessments_scoring) | **POST** /api/v2/learning/assessments/scoring | Score learning assessment for preview
[**post_learning_assignments**](LearningApi.md#post_learning_assignments) | **POST** /api/v2/learning/assignments | Create Learning Assignment
[**post_learning_assignments_aggregates_query**](LearningApi.md#post_learning_assignments_aggregates_query) | **POST** /api/v2/learning/assignments/aggregates/query | Retrieve aggregated assignment data
[**post_learning_assignments_bulkadd**](LearningApi.md#post_learning_assignments_bulkadd) | **POST** /api/v2/learning/assignments/bulkadd | Add multiple learning assignments
[**post_learning_assignments_bulkremove**](LearningApi.md#post_learning_assignments_bulkremove) | **POST** /api/v2/learning/assignments/bulkremove | Remove multiple Learning Assignments
[**post_learning_module_publish**](LearningApi.md#post_learning_module_publish) | **POST** /api/v2/learning/modules/{moduleId}/publish | Publish a Learning module
[**post_learning_modules**](LearningApi.md#post_learning_modules) | **POST** /api/v2/learning/modules | Create a new learning module
[**post_learning_rules_query**](LearningApi.md#post_learning_rules_query) | **POST** /api/v2/learning/rules/query | Get users for learning module rule
[**put_learning_module**](LearningApi.md#put_learning_module) | **PUT** /api/v2/learning/modules/{moduleId} | Update a learning module
[**put_learning_module_rule**](LearningApi.md#put_learning_module_rule) | **PUT** /api/v2/learning/modules/{moduleId}/rule | Update a learning module rule



## delete_learning_assignment

> delete_learning_assignment(assignment_id)
Delete a learning assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | The Learning Assignment ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_learning_module

> delete_learning_module(module_id)
Delete a learning module

This will delete a learning module if it is unpublished or it will delete a published and archived learning module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **String** | The ID of the learning module | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_learning_assignment

> crate::models::LearningAssignment get_learning_assignment(assignment_id, expand)
Get Learning Assignment

Permission not required if you are the assigned user of the learning assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | The ID of Learning Assignment | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in response |  |

### Return type

[**crate::models::LearningAssignment**](LearningAssignment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_learning_assignments

> crate::models::LearningAssignmentsDomainEntity get_learning_assignments(module_id, interval, completion_interval, overdue, page_size, page_number, pass, min_percentage_score, max_percentage_score, sort_order, sort_by, user_id, types, states, expand)
List of Learning module Assignments

Either moduleId or user value is required

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | Option<**String**> | Specifies the ID of the learning module. Fetch assignments for learning module ID |  |
**interval** | Option<**String**> | Specifies the range of dueDates to be queried. Milliseconds will be truncated. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**completion_interval** | Option<**String**> | Specifies the range of completion dates to be used for filtering. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**overdue** | Option<**String**> | Specifies if only the non-overdue (overdue is \"False\") or overdue (overdue is \"True\") assignments are returned. If overdue is \"Any\" or if the overdue parameter is not supplied, all assignments are returned |  |[default to Any]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**pass** | Option<**String**> | Specifies if only the failed (pass is \"False\") or passed (pass is \"True\") assignments (completed with assessment)are returned. If pass is \"Any\" or if the pass parameter is not supplied, all assignments are returned |  |[default to Any]
**min_percentage_score** | Option<**f32**> | The minimum assessment score for an assignment (completed with assessment) to be included in the results (inclusive) |  |
**max_percentage_score** | Option<**f32**> | The maximum assessment score for an assignment (completed with assessment) to be included in the results (inclusive) |  |
**sort_order** | Option<**String**> | Specifies result set sort order; if not specified, default sort order is descending (Desc) |  |[default to Desc]
**sort_by** | Option<**String**> | Specifies which field to sort the results by, default sort is by recommendedCompletionDate |  |
**user_id** | Option<[**Vec<String>**](String.md)> | Specifies the list of user IDs to be queried, up to 100 user IDs. |  |
**types** | Option<[**Vec<String>**](String.md)> | Specifies the assignment types, currently not supported and will be ignored. For now, all learning assignments regardless of types will be returned |  |
**states** | Option<[**Vec<String>**](String.md)> | Specifies the assignment states to filter by |  |
**expand** | Option<[**Vec<String>**](String.md)> | Specifies the expand option for returning additional information |  |

### Return type

[**crate::models::LearningAssignmentsDomainEntity**](LearningAssignmentsDomainEntity.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_learning_assignments_me

> crate::models::LearningAssignmentsDomainEntity get_learning_assignments_me(module_id, interval, completion_interval, overdue, page_size, page_number, pass, min_percentage_score, max_percentage_score, sort_order, sort_by, types, states, expand)
List of Learning Assignments assigned to current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | Option<**String**> | Specifies the ID of the learning module. Fetch assignments for learning module ID |  |
**interval** | Option<**String**> | Specifies the range of dueDates to be queried. Milliseconds will be truncated. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**completion_interval** | Option<**String**> | Specifies the range of completion dates to be used for filtering. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**overdue** | Option<**String**> | Specifies if only the non-overdue (overdue is \"False\") or overdue (overdue is \"True\") assignments are returned. If overdue is \"Any\" or if the overdue parameter is not supplied, all assignments are returned |  |[default to Any]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**pass** | Option<**String**> | Specifies if only the failed (pass is \"False\") or passed (pass is \"True\") assignments (completed with assessment)are returned. If pass is \"Any\" or if the pass parameter is not supplied, all assignments are returned |  |[default to Any]
**min_percentage_score** | Option<**f32**> | The minimum assessment score for an assignment (completed with assessment) to be included in the results (inclusive) |  |
**max_percentage_score** | Option<**f32**> | The maximum assessment score for an assignment (completed with assessment) to be included in the results (inclusive) |  |
**sort_order** | Option<**String**> | Specifies result set sort order; if not specified, default sort order is descending (Desc) |  |[default to Desc]
**sort_by** | Option<**String**> | Specifies which field to sort the results by, default sort is by recommendedCompletionDate |  |
**types** | Option<[**Vec<String>**](String.md)> | Specifies the assignment types, currently not supported and will be ignored. For now, all learning assignments regardless of types will be returned |  |
**states** | Option<[**Vec<String>**](String.md)> | Specifies the assignment states to filter by |  |
**expand** | Option<[**Vec<String>**](String.md)> | Specifies the expand option for returning additional information |  |

### Return type

[**crate::models::LearningAssignmentsDomainEntity**](LearningAssignmentsDomainEntity.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_learning_module

> crate::models::LearningModule get_learning_module(module_id, expand)
Get a learning module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **String** | The ID of the learning module | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in response(case insensitive) |  |

### Return type

[**crate::models::LearningModule**](LearningModule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_learning_module_rule

> crate::models::LearningModuleRule get_learning_module_rule(module_id)
Get a learning module rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **String** | The ID of the learning module | [required] |

### Return type

[**crate::models::LearningModuleRule**](LearningModuleRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_learning_module_version

> crate::models::LearningModule get_learning_module_version(module_id, version_id, expand)
Get specific version of a published module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **String** | The ID of the learning module | [required] |
**version_id** | **String** | The version of learning module | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in response(case insensitive) |  |

### Return type

[**crate::models::LearningModule**](LearningModule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_learning_modules

> crate::models::LearningModulesDomainEntityListing get_learning_modules(is_archived, types, page_size, page_number, sort_order, sort_by, search_term, expand, is_published)
Get all learning modules of an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**is_archived** | Option<**bool**> | Archive status |  |[default to false]
**types** | Option<[**Vec<String>**](String.md)> | Specifies the module types. |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**search_term** | Option<**String**> | Search Term (searchable by name) |  |
**expand** | Option<[**Vec<String>**](String.md)> | Fields to expand in response(case insensitive) |  |
**is_published** | Option<**String**> | Specifies if only the Unpublished (isPublished is \"False\") or Published (isPublished is \"True\") modules are returned. If isPublished is \"Any\" or omitted, both types are returned |  |[default to Any]

### Return type

[**crate::models::LearningModulesDomainEntityListing**](LearningModulesDomainEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_learning_assignment

> crate::models::LearningAssignment patch_learning_assignment(assignment_id, body)
Update Learning Assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | The ID of Learning Assignment | [required] |
**body** | Option<[**LearningAssignmentUpdate**](LearningAssignmentUpdate.md)> | The Learning Assignment to be updated |  |

### Return type

[**crate::models::LearningAssignment**](LearningAssignment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_assessments_scoring

> crate::models::AssessmentScoringSet post_learning_assessments_scoring(body)
Score learning assessment for preview

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LearningAssessmentScoringRequest**](LearningAssessmentScoringRequest.md) | Assessment form and answers to score | [required] |

### Return type

[**crate::models::AssessmentScoringSet**](AssessmentScoringSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_assignments

> crate::models::LearningAssignment post_learning_assignments(body)
Create Learning Assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**LearningAssignmentCreate**](LearningAssignmentCreate.md)> | The Learning Assignment to be created |  |

### Return type

[**crate::models::LearningAssignment**](LearningAssignment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_assignments_aggregates_query

> crate::models::LearningAssignmentAggregateResponse post_learning_assignments_aggregates_query(body)
Retrieve aggregated assignment data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LearningAssignmentAggregateParam**](LearningAssignmentAggregateParam.md) | Aggregate Request | [required] |

### Return type

[**crate::models::LearningAssignmentAggregateResponse**](LearningAssignmentAggregateResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_assignments_bulkadd

> crate::models::LearningAssignmentBulkAddResponse post_learning_assignments_bulkadd(body)
Add multiple learning assignments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Vec<crate::models::LearningAssignmentItem>**](LearningAssignmentItem.md)> | The learning assignments to be created |  |

### Return type

[**crate::models::LearningAssignmentBulkAddResponse**](LearningAssignmentBulkAddResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_assignments_bulkremove

> crate::models::LearningAssignmentBulkRemoveResponse post_learning_assignments_bulkremove(body)
Remove multiple Learning Assignments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Vec<String>**](String.md)> | The IDs of the learning assignments to be removed |  |

### Return type

[**crate::models::LearningAssignmentBulkRemoveResponse**](LearningAssignmentBulkRemoveResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_module_publish

> crate::models::LearningModulePublishResponse post_learning_module_publish(module_id)
Publish a Learning module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **String** | The ID of the learning module | [required] |

### Return type

[**crate::models::LearningModulePublishResponse**](LearningModulePublishResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_modules

> crate::models::LearningModule post_learning_modules(body)
Create a new learning module

This will create a new unpublished learning module with the specified fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LearningModuleRequest**](LearningModuleRequest.md) | The learning module to be created | [required] |

### Return type

[**crate::models::LearningModule**](LearningModule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_learning_rules_query

> crate::models::LearningAssignmentUserListing post_learning_rules_query(page_size, page_number, body)
Get users for learning module rule

This will get the users who matches the given rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | **i32** | Page size | [required] |[default to 50]
**page_number** | **i32** | Page number | [required] |[default to 1]
**body** | [**LearningAssignmentUserQuery**](LearningAssignmentUserQuery.md) | The learning module rule to fetch users | [required] |

### Return type

[**crate::models::LearningAssignmentUserListing**](LearningAssignmentUserListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_learning_module

> crate::models::LearningModule put_learning_module(module_id, body)
Update a learning module

This will update the name, description, completion time in days and inform steps for a learning module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **String** | The ID of the learning module | [required] |
**body** | [**LearningModuleRequest**](LearningModuleRequest.md) | The learning module to be updated | [required] |

### Return type

[**crate::models::LearningModule**](LearningModule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_learning_module_rule

> crate::models::LearningModuleRule put_learning_module_rule(module_id, body)
Update a learning module rule

This will update a learning module rule with the specified fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | **String** | The ID of the learning module | [required] |
**body** | [**LearningModuleRule**](LearningModuleRule.md) | The learning module rule to be updated | [required] |

### Return type

[**crate::models::LearningModuleRule**](LearningModuleRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

