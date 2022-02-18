# \SuggestApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_search**](SuggestApi.md#get_search) | **GET** /api/v2/search | Search using the q64 value returned from a previous search.
[**get_search_suggest**](SuggestApi.md#get_search_suggest) | **GET** /api/v2/search/suggest | Suggest resources using the q64 value returned from a previous suggest query.
[**post_search**](SuggestApi.md#post_search) | **POST** /api/v2/search | Search resources.
[**post_search_suggest**](SuggestApi.md#post_search_suggest) | **POST** /api/v2/search/suggest | Suggest resources.



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

