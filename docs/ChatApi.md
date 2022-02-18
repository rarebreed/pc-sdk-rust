# \ChatApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_chat_settings**](ChatApi.md#get_chat_settings) | **GET** /api/v2/chat/settings | Get Chat Settings.
[**patch_chat_settings**](ChatApi.md#patch_chat_settings) | **PATCH** /api/v2/chat/settings | Patch Chat Settings.
[**put_chat_settings**](ChatApi.md#put_chat_settings) | **PUT** /api/v2/chat/settings | Update Chat Settings.



## get_chat_settings

> crate::models::ChatSettings get_chat_settings()
Get Chat Settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ChatSettings**](ChatSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_chat_settings

> crate::models::ChatSettings patch_chat_settings(body)
Patch Chat Settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ChatSettings**](ChatSettings.md) | Chat | [required] |

### Return type

[**crate::models::ChatSettings**](ChatSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_chat_settings

> crate::models::ChatSettings put_chat_settings(body)
Update Chat Settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ChatSettings**](ChatSettings.md) | Chat | [required] |

### Return type

[**crate::models::ChatSettings**](ChatSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

