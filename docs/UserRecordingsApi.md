# \UserRecordingsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_userrecording**](UserRecordingsApi.md#delete_userrecording) | **DELETE** /api/v2/userrecordings/{recordingId} | Delete a user recording.
[**get_userrecording**](UserRecordingsApi.md#get_userrecording) | **GET** /api/v2/userrecordings/{recordingId} | Get a user recording.
[**get_userrecording_media**](UserRecordingsApi.md#get_userrecording_media) | **GET** /api/v2/userrecordings/{recordingId}/media | Download a user recording.
[**get_userrecordings**](UserRecordingsApi.md#get_userrecordings) | **GET** /api/v2/userrecordings | Get a list of user recordings.
[**get_userrecordings_summary**](UserRecordingsApi.md#get_userrecordings_summary) | **GET** /api/v2/userrecordings/summary | Get user recording summary
[**put_userrecording**](UserRecordingsApi.md#put_userrecording) | **PUT** /api/v2/userrecordings/{recordingId} | Update a user recording.



## delete_userrecording

> delete_userrecording(recording_id)
Delete a user recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_id** | **String** | User Recording ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_userrecording

> crate::models::UserRecording get_userrecording(recording_id, expand)
Get a user recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_id** | **String** | User Recording ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::UserRecording**](UserRecording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_userrecording_media

> crate::models::DownloadResponse get_userrecording_media(recording_id, format_id)
Download a user recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_id** | **String** | User Recording ID | [required] |
**format_id** | Option<**String**> | The desired media format. |  |[default to WEBM]

### Return type

[**crate::models::DownloadResponse**](DownloadResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_userrecordings

> crate::models::UserRecordingEntityListing get_userrecordings(page_size, page_number, expand)
Get a list of user recordings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::UserRecordingEntityListing**](UserRecordingEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_userrecordings_summary

> crate::models::FaxSummary get_userrecordings_summary()
Get user recording summary

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FaxSummary**](FaxSummary.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_userrecording

> crate::models::UserRecording put_userrecording(recording_id, body, expand)
Update a user recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recording_id** | **String** | User Recording ID | [required] |
**body** | [**UserRecording**](UserRecording.md) | UserRecording | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::UserRecording**](UserRecording.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

