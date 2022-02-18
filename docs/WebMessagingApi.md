# \WebMessagingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_webmessaging_messages**](WebMessagingApi.md#get_webmessaging_messages) | **GET** /api/v2/webmessaging/messages | Get the messages for a web messaging session.



## get_webmessaging_messages

> crate::models::WebMessagingMessageEntityList get_webmessaging_messages(page_size, page_number)
Get the messages for a web messaging session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::WebMessagingMessageEntityList**](WebMessagingMessageEntityList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

