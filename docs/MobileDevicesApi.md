# \MobileDevicesApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_mobiledevice**](MobileDevicesApi.md#delete_mobiledevice) | **DELETE** /api/v2/mobiledevices/{deviceId} | Delete device
[**get_mobiledevice**](MobileDevicesApi.md#get_mobiledevice) | **GET** /api/v2/mobiledevices/{deviceId} | Get device
[**get_mobiledevices**](MobileDevicesApi.md#get_mobiledevices) | **GET** /api/v2/mobiledevices | Get a list of all devices.
[**post_mobiledevices**](MobileDevicesApi.md#post_mobiledevices) | **POST** /api/v2/mobiledevices | Create User device
[**put_mobiledevice**](MobileDevicesApi.md#put_mobiledevice) | **PUT** /api/v2/mobiledevices/{deviceId} | Update device



## delete_mobiledevice

> delete_mobiledevice(device_id)
Delete device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | Device ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mobiledevice

> crate::models::UserDevice get_mobiledevice(device_id)
Get device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | Device ID | [required] |

### Return type

[**crate::models::UserDevice**](UserDevice.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mobiledevices

> crate::models::DirectoryUserDevicesListing get_mobiledevices(page_size, page_number, sort_order)
Get a list of all devices.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ascending]

### Return type

[**crate::models::DirectoryUserDevicesListing**](DirectoryUserDevicesListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_mobiledevices

> crate::models::UserDevice post_mobiledevices(body)
Create User device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserDevice**](UserDevice.md) | Device | [required] |

### Return type

[**crate::models::UserDevice**](UserDevice.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_mobiledevice

> crate::models::UserDevice put_mobiledevice(device_id, body)
Update device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | Device ID | [required] |
**body** | Option<[**UserDevice**](UserDevice.md)> | Device |  |

### Return type

[**crate::models::UserDevice**](UserDevice.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

