# \GeneralDataProtectionRegulationApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gdpr_request**](GeneralDataProtectionRegulationApi.md#get_gdpr_request) | **GET** /api/v2/gdpr/requests/{requestId} | Get an existing GDPR request
[**get_gdpr_requests**](GeneralDataProtectionRegulationApi.md#get_gdpr_requests) | **GET** /api/v2/gdpr/requests | Get all GDPR requests
[**get_gdpr_subjects**](GeneralDataProtectionRegulationApi.md#get_gdpr_subjects) | **GET** /api/v2/gdpr/subjects | Get GDPR subjects
[**post_gdpr_requests**](GeneralDataProtectionRegulationApi.md#post_gdpr_requests) | **POST** /api/v2/gdpr/requests | Submit a new GDPR request



## get_gdpr_request

> crate::models::GdprRequest get_gdpr_request(request_id)
Get an existing GDPR request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Request id | [required] |

### Return type

[**crate::models::GdprRequest**](GDPRRequest.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gdpr_requests

> crate::models::GdprRequestEntityListing get_gdpr_requests(page_size, page_number)
Get all GDPR requests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::GdprRequestEntityListing**](GDPRRequestEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gdpr_subjects

> crate::models::GdprSubjectEntityListing get_gdpr_subjects(search_type, search_value)
Get GDPR subjects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_type** | **String** | Search Type | [required] |
**search_value** | **String** | Search Value | [required] |

### Return type

[**crate::models::GdprSubjectEntityListing**](GDPRSubjectEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gdpr_requests

> crate::models::GdprRequest post_gdpr_requests(body, delete_confirmed)
Submit a new GDPR request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GdprRequest**](GdprRequest.md) | GDPR request | [required] |
**delete_confirmed** | Option<**bool**> | Confirm delete |  |[default to false]

### Return type

[**crate::models::GdprRequest**](GDPRRequest.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

