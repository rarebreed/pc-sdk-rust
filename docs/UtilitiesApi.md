# \UtilitiesApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_date**](UtilitiesApi.md#get_date) | **GET** /api/v2/date | Get the current system date/time
[**get_ipranges**](UtilitiesApi.md#get_ipranges) | **GET** /api/v2/ipranges | Get public ip address ranges for Genesys Cloud
[**get_timezones**](UtilitiesApi.md#get_timezones) | **GET** /api/v2/timezones | Get time zones list
[**post_certificate_details**](UtilitiesApi.md#post_certificate_details) | **POST** /api/v2/certificate/details | Returns the information about an X509 PEM encoded certificate or certificate chain.



## get_date

> crate::models::ServerDate get_date()
Get the current system date/time

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServerDate**](ServerDate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ipranges

> crate::models::IpAddressRangeListing get_ipranges()
Get public ip address ranges for Genesys Cloud

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IpAddressRangeListing**](IpAddressRangeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_timezones

> crate::models::TimeZoneEntityListing get_timezones(page_size, page_number)
Get time zones list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::TimeZoneEntityListing**](TimeZoneEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_certificate_details

> crate::models::ParsedCertificate post_certificate_details(body)
Returns the information about an X509 PEM encoded certificate or certificate chain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Certificate**](Certificate.md) | Certificate | [required] |

### Return type

[**crate::models::ParsedCertificate**](ParsedCertificate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

