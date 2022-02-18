# \FlowsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_analytics_flows_aggregates_query**](FlowsApi.md#post_analytics_flows_aggregates_query) | **POST** /api/v2/analytics/flows/aggregates/query | Query for flow aggregates
[**post_analytics_flows_observations_query**](FlowsApi.md#post_analytics_flows_observations_query) | **POST** /api/v2/analytics/flows/observations/query | Query for flow observations



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

