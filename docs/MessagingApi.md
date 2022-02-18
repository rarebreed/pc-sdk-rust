# \MessagingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_messaging_supportedcontent_supported_content_id**](MessagingApi.md#delete_messaging_supportedcontent_supported_content_id) | **DELETE** /api/v2/messaging/supportedcontent/{supportedContentId} | Delete a supported content profile
[**get_messaging_supportedcontent**](MessagingApi.md#get_messaging_supportedcontent) | **GET** /api/v2/messaging/supportedcontent | Get a list of Supported Content profiles
[**get_messaging_supportedcontent_supported_content_id**](MessagingApi.md#get_messaging_supportedcontent_supported_content_id) | **GET** /api/v2/messaging/supportedcontent/{supportedContentId} | Get a supported content profile
[**patch_messaging_supportedcontent_supported_content_id**](MessagingApi.md#patch_messaging_supportedcontent_supported_content_id) | **PATCH** /api/v2/messaging/supportedcontent/{supportedContentId} | Update a supported content profile
[**post_messaging_supportedcontent**](MessagingApi.md#post_messaging_supportedcontent) | **POST** /api/v2/messaging/supportedcontent | Create a Supported Content profile



## delete_messaging_supportedcontent_supported_content_id

> delete_messaging_supportedcontent_supported_content_id(supported_content_id)
Delete a supported content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supported_content_id** | **String** | Supported Content ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_messaging_supportedcontent

> crate::models::SupportedContentListing get_messaging_supportedcontent(page_size, page_number)
Get a list of Supported Content profiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::SupportedContentListing**](SupportedContentListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_messaging_supportedcontent_supported_content_id

> crate::models::SupportedContent get_messaging_supportedcontent_supported_content_id(supported_content_id)
Get a supported content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supported_content_id** | **String** | Supported Content ID | [required] |

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_messaging_supportedcontent_supported_content_id

> crate::models::SupportedContent patch_messaging_supportedcontent_supported_content_id(supported_content_id, body)
Update a supported content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**supported_content_id** | **String** | Supported Content ID | [required] |
**body** | [**SupportedContent**](SupportedContent.md) | SupportedContent | [required] |

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_messaging_supportedcontent

> crate::models::SupportedContent post_messaging_supportedcontent(body)
Create a Supported Content profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SupportedContent**](SupportedContent.md) | SupportedContent | [required] |

### Return type

[**crate::models::SupportedContent**](SupportedContent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

