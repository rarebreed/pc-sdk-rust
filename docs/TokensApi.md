# \TokensApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_token**](TokensApi.md#delete_token) | **DELETE** /api/v2/tokens/{userId} | Delete all auth tokens for the specified user.
[**delete_tokens_me**](TokensApi.md#delete_tokens_me) | **DELETE** /api/v2/tokens/me | Delete auth token used to make the request.
[**get_tokens_me**](TokensApi.md#get_tokens_me) | **GET** /api/v2/tokens/me | Fetch information about the current token
[**head_tokens_me**](TokensApi.md#head_tokens_me) | **HEAD** /api/v2/tokens/me | Verify user token



## delete_token

> delete_token(user_id)
Delete all auth tokens for the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tokens_me

> delete_tokens_me()
Delete auth token used to make the request.

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


## get_tokens_me

> crate::models::TokenInfo get_tokens_me()
Fetch information about the current token

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TokenInfo**](TokenInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_tokens_me

> head_tokens_me()
Verify user token

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

