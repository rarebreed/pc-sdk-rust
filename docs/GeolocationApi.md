# \GeolocationApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_geolocations_settings**](GeolocationApi.md#get_geolocations_settings) | **GET** /api/v2/geolocations/settings | Get a organization's GeolocationSettings
[**get_user_geolocation**](GeolocationApi.md#get_user_geolocation) | **GET** /api/v2/users/{userId}/geolocations/{clientId} | Get a user's Geolocation
[**patch_geolocations_settings**](GeolocationApi.md#patch_geolocations_settings) | **PATCH** /api/v2/geolocations/settings | Patch a organization's GeolocationSettings
[**patch_user_geolocation**](GeolocationApi.md#patch_user_geolocation) | **PATCH** /api/v2/users/{userId}/geolocations/{clientId} | Patch a user's Geolocation



## get_geolocations_settings

> crate::models::GeolocationSettings get_geolocations_settings()
Get a organization's GeolocationSettings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GeolocationSettings**](GeolocationSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_geolocation

> crate::models::Geolocation get_user_geolocation(user_id, client_id)
Get a user's Geolocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |
**client_id** | **String** | client Id | [required] |

### Return type

[**crate::models::Geolocation**](Geolocation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_geolocations_settings

> crate::models::GeolocationSettings patch_geolocations_settings(body)
Patch a organization's GeolocationSettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**GeolocationSettings**](GeolocationSettings.md) | Geolocation settings | [required] |

### Return type

[**crate::models::GeolocationSettings**](GeolocationSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_geolocation

> crate::models::Geolocation patch_user_geolocation(user_id, client_id, body)
Patch a user's Geolocation

The geolocation object can be patched one of three ways. Option 1: Set the 'primary' property to true. This will set the client as the user's primary geolocation source.  Option 2: Provide the 'latitude' and 'longitude' values.  This will enqueue an asynchronous update of the 'city', 'region', and 'country', generating a notification. A subsequent GET operation will include the new values for 'city', 'region' and 'country'.  Option 3:  Provide the 'city', 'region', 'country' values.  Option 1 can be combined with Option 2 or Option 3.  For example, update the client as primary and provide latitude and longitude values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |
**client_id** | **String** | client Id | [required] |
**body** | [**Geolocation**](Geolocation.md) | Geolocation | [required] |

### Return type

[**crate::models::Geolocation**](Geolocation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

