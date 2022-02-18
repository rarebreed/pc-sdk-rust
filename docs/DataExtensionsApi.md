# \DataExtensionsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dataextensions_coretype**](DataExtensionsApi.md#get_dataextensions_coretype) | **GET** /api/v2/dataextensions/coretypes/{coretypeName} | Get a specific named core type.
[**get_dataextensions_coretypes**](DataExtensionsApi.md#get_dataextensions_coretypes) | **GET** /api/v2/dataextensions/coretypes | Get the core types from which all schemas are built.
[**get_dataextensions_limits**](DataExtensionsApi.md#get_dataextensions_limits) | **GET** /api/v2/dataextensions/limits | Get quantitative limits on schemas



## get_dataextensions_coretype

> crate::models::Coretype get_dataextensions_coretype(coretype_name)
Get a specific named core type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**coretype_name** | **String** | The core type's name | [required] |

### Return type

[**crate::models::Coretype**](Coretype.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dataextensions_coretypes

> crate::models::CoretypeListing get_dataextensions_coretypes()
Get the core types from which all schemas are built.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CoretypeListing**](CoretypeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dataextensions_limits

> crate::models::SchemaQuantityLimits get_dataextensions_limits()
Get quantitative limits on schemas

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SchemaQuantityLimits**](SchemaQuantityLimits.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

