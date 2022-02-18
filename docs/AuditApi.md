# \AuditApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_audits_query_realtime_servicemapping**](AuditApi.md#get_audits_query_realtime_servicemapping) | **GET** /api/v2/audits/query/realtime/servicemapping | Get service mapping information used in realtime audits.
[**get_audits_query_servicemapping**](AuditApi.md#get_audits_query_servicemapping) | **GET** /api/v2/audits/query/servicemapping | Get service mapping information used in audits.
[**get_audits_query_transaction_id**](AuditApi.md#get_audits_query_transaction_id) | **GET** /api/v2/audits/query/{transactionId} | Get status of audit query execution
[**get_audits_query_transaction_id_results**](AuditApi.md#get_audits_query_transaction_id_results) | **GET** /api/v2/audits/query/{transactionId}/results | Get results of audit query
[**post_audits_query**](AuditApi.md#post_audits_query) | **POST** /api/v2/audits/query | Create audit query execution
[**post_audits_query_realtime**](AuditApi.md#post_audits_query_realtime) | **POST** /api/v2/audits/query/realtime | This endpoint will only retrieve 14 days worth of audits for certain services. Please use /query to get a full list and older audits.



## get_audits_query_realtime_servicemapping

> crate::models::AuditQueryServiceMapping get_audits_query_realtime_servicemapping()
Get service mapping information used in realtime audits.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuditQueryServiceMapping**](AuditQueryServiceMapping.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audits_query_servicemapping

> crate::models::AuditQueryServiceMapping get_audits_query_servicemapping()
Get service mapping information used in audits.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuditQueryServiceMapping**](AuditQueryServiceMapping.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audits_query_transaction_id

> crate::models::AuditQueryExecutionStatusResponse get_audits_query_transaction_id(transaction_id)
Get status of audit query execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | Transaction ID | [required] |

### Return type

[**crate::models::AuditQueryExecutionStatusResponse**](AuditQueryExecutionStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audits_query_transaction_id_results

> crate::models::AuditQueryExecutionResultsResponse get_audits_query_transaction_id_results(transaction_id, cursor, page_size, expand)
Get results of audit query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | Transaction ID | [required] |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::AuditQueryExecutionResultsResponse**](AuditQueryExecutionResultsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_audits_query

> crate::models::AuditQueryExecutionStatusResponse post_audits_query(body)
Create audit query execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AuditQueryRequest**](AuditQueryRequest.md) | query | [required] |

### Return type

[**crate::models::AuditQueryExecutionStatusResponse**](AuditQueryExecutionStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_audits_query_realtime

> crate::models::AuditRealtimeQueryResultsResponse post_audits_query_realtime(body, expand)
This endpoint will only retrieve 14 days worth of audits for certain services. Please use /query to get a full list and older audits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AuditRealtimeQueryRequest**](AuditRealtimeQueryRequest.md) | query | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::AuditRealtimeQueryResultsResponse**](AuditRealtimeQueryResultsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

