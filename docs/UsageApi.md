# \UsageApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_usage_query_execution_id_results**](UsageApi.md#get_usage_query_execution_id_results) | **GET** /api/v2/usage/query/{executionId}/results | Get the results of a usage query
[**post_usage_query**](UsageApi.md#post_usage_query) | **POST** /api/v2/usage/query | Query organization API Usage - 



## get_usage_query_execution_id_results

> crate::models::ApiUsageQueryResult get_usage_query_execution_id_results(execution_id)
Get the results of a usage query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **String** | ID of the query execution | [required] |

### Return type

[**crate::models::ApiUsageQueryResult**](ApiUsageQueryResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_usage_query

> crate::models::UsageExecutionResult post_usage_query(body)
Query organization API Usage - 

After calling this method, you will then need to poll for the query results based on the returned execution Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ApiUsageQuery**](ApiUsageQuery.md) | Query | [required] |

### Return type

[**crate::models::UsageExecutionResult**](UsageExecutionResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

