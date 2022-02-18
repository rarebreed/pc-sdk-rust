# \StationsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_station_associateduser**](StationsApi.md#delete_station_associateduser) | **DELETE** /api/v2/stations/{stationId}/associateduser | Unassigns the user assigned to this station
[**get_station**](StationsApi.md#get_station) | **GET** /api/v2/stations/{stationId} | Get station.
[**get_stations**](StationsApi.md#get_stations) | **GET** /api/v2/stations | Get the list of available stations.
[**get_stations_settings**](StationsApi.md#get_stations_settings) | **GET** /api/v2/stations/settings | Get an organization's StationSettings
[**patch_stations_settings**](StationsApi.md#patch_stations_settings) | **PATCH** /api/v2/stations/settings | Patch an organization's StationSettings



## delete_station_associateduser

> delete_station_associateduser(station_id)
Unassigns the user assigned to this station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**station_id** | **String** | Station ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_station

> crate::models::Station get_station(station_id)
Get station.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**station_id** | **String** | Station ID | [required] |

### Return type

[**crate::models::Station**](Station.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stations

> crate::models::StationEntityListing get_stations(page_size, page_number, sort_by, name, user_selectable, web_rtc_user_id, id, line_appearance_id)
Get the list of available stations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**name** | Option<**String**> | Name |  |
**user_selectable** | Option<**String**> | True for stations that the user can select otherwise false |  |
**web_rtc_user_id** | Option<**String**> | Filter for the webRtc station of the webRtcUserId |  |
**id** | Option<**String**> | Comma separated list of stationIds |  |
**line_appearance_id** | Option<**String**> | lineAppearanceId |  |

### Return type

[**crate::models::StationEntityListing**](StationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stations_settings

> crate::models::StationSettings get_stations_settings()
Get an organization's StationSettings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::StationSettings**](StationSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_stations_settings

> crate::models::StationSettings patch_stations_settings(body)
Patch an organization's StationSettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**StationSettings**](StationSettings.md) | Station settings | [required] |

### Return type

[**crate::models::StationSettings**](StationSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

