# \LocationsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_location**](LocationsApi.md#delete_location) | **DELETE** /api/v2/locations/{locationId} | Delete a location
[**get_location**](LocationsApi.md#get_location) | **GET** /api/v2/locations/{locationId} | Get Location by ID.
[**get_location_sublocations**](LocationsApi.md#get_location_sublocations) | **GET** /api/v2/locations/{locationId}/sublocations | Get sublocations for location ID.
[**get_locations**](LocationsApi.md#get_locations) | **GET** /api/v2/locations | Get a list of all locations.
[**get_locations_search**](LocationsApi.md#get_locations_search) | **GET** /api/v2/locations/search | Search locations using the q64 value returned from a previous search
[**patch_location**](LocationsApi.md#patch_location) | **PATCH** /api/v2/locations/{locationId} | Update a location
[**post_locations**](LocationsApi.md#post_locations) | **POST** /api/v2/locations | Create a location
[**post_locations_search**](LocationsApi.md#post_locations_search) | **POST** /api/v2/locations/search | Search locations



## delete_location

> delete_location(location_id)
Delete a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | Location ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_location

> crate::models::LocationDefinition get_location(location_id, expand)
Get Location by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | Location ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::LocationDefinition**](LocationDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_location_sublocations

> crate::models::LocationEntityListing get_location_sublocations(location_id)
Get sublocations for location ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | Location ID | [required] |

### Return type

[**crate::models::LocationEntityListing**](LocationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locations

> crate::models::LocationEntityListing get_locations(page_size, page_number, id, sort_order)
Get a list of all locations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**sort_order** | Option<**String**> | Sort order |  |

### Return type

[**crate::models::LocationEntityListing**](LocationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_locations_search

> crate::models::LocationsSearchResponse get_locations_search(q64, expand)
Search locations using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Provides more details about a specified resource |  |

### Return type

[**crate::models::LocationsSearchResponse**](LocationsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_location

> crate::models::LocationDefinition patch_location(location_id, body)
Update a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_id** | **String** | Location ID | [required] |
**body** | [**LocationUpdateDefinition**](LocationUpdateDefinition.md) | Location | [required] |

### Return type

[**crate::models::LocationDefinition**](LocationDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_locations

> crate::models::LocationDefinition post_locations(body)
Create a location

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LocationCreateDefinition**](LocationCreateDefinition.md) | Location | [required] |

### Return type

[**crate::models::LocationDefinition**](LocationDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_locations_search

> crate::models::LocationsSearchResponse post_locations_search(body)
Search locations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LocationSearchRequest**](LocationSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::LocationsSearchResponse**](LocationsSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

