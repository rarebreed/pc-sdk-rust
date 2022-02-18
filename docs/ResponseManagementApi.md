# \ResponseManagementApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_responsemanagement_library**](ResponseManagementApi.md#delete_responsemanagement_library) | **DELETE** /api/v2/responsemanagement/libraries/{libraryId} | Delete an existing response library.
[**delete_responsemanagement_response**](ResponseManagementApi.md#delete_responsemanagement_response) | **DELETE** /api/v2/responsemanagement/responses/{responseId} | Delete an existing response.
[**get_responsemanagement_libraries**](ResponseManagementApi.md#get_responsemanagement_libraries) | **GET** /api/v2/responsemanagement/libraries | Gets a list of existing response libraries.
[**get_responsemanagement_library**](ResponseManagementApi.md#get_responsemanagement_library) | **GET** /api/v2/responsemanagement/libraries/{libraryId} | Get details about an existing response library.
[**get_responsemanagement_response**](ResponseManagementApi.md#get_responsemanagement_response) | **GET** /api/v2/responsemanagement/responses/{responseId} | Get details about an existing response.
[**get_responsemanagement_responses**](ResponseManagementApi.md#get_responsemanagement_responses) | **GET** /api/v2/responsemanagement/responses | Gets a list of existing responses.
[**post_responsemanagement_libraries**](ResponseManagementApi.md#post_responsemanagement_libraries) | **POST** /api/v2/responsemanagement/libraries | Create a response library.
[**post_responsemanagement_responses**](ResponseManagementApi.md#post_responsemanagement_responses) | **POST** /api/v2/responsemanagement/responses | Create a response.
[**post_responsemanagement_responses_query**](ResponseManagementApi.md#post_responsemanagement_responses_query) | **POST** /api/v2/responsemanagement/responses/query | Query responses
[**put_responsemanagement_library**](ResponseManagementApi.md#put_responsemanagement_library) | **PUT** /api/v2/responsemanagement/libraries/{libraryId} | Update an existing response library.
[**put_responsemanagement_response**](ResponseManagementApi.md#put_responsemanagement_response) | **PUT** /api/v2/responsemanagement/responses/{responseId} | Update an existing response.



## delete_responsemanagement_library

> delete_responsemanagement_library(library_id)
Delete an existing response library.

This will remove any responses associated with the library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** | Library ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_responsemanagement_response

> delete_responsemanagement_response(response_id)
Delete an existing response.

This will remove the response from any libraries associated with it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | Response ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_responsemanagement_libraries

> crate::models::LibraryEntityListing get_responsemanagement_libraries(page_number, page_size, messaging_template_filter)
Gets a list of existing response libraries.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**messaging_template_filter** | Option<**String**> | Returns a list of libraries that contain responses with at least one messaging template defined for a specific message channel |  |

### Return type

[**crate::models::LibraryEntityListing**](LibraryEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_responsemanagement_library

> crate::models::Library get_responsemanagement_library(library_id)
Get details about an existing response library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** | Library ID | [required] |

### Return type

[**crate::models::Library**](Library.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_responsemanagement_response

> crate::models::Response get_responsemanagement_response(response_id, expand)
Get details about an existing response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | Response ID | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::Response**](Response.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_responsemanagement_responses

> crate::models::ResponseEntityListing get_responsemanagement_responses(library_id, page_number, page_size, expand)
Gets a list of existing responses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** | Library ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::ResponseEntityListing**](ResponseEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_responsemanagement_libraries

> crate::models::Library post_responsemanagement_libraries(body)
Create a response library.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Library**](Library.md) | Library | [required] |

### Return type

[**crate::models::Library**](Library.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_responsemanagement_responses

> crate::models::Response post_responsemanagement_responses(body, expand)
Create a response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Response**](Response.md) | Response | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::Response**](Response.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_responsemanagement_responses_query

> crate::models::ResponseQueryResults post_responsemanagement_responses_query(body)
Query responses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ResponseQueryRequest**](ResponseQueryRequest.md) | Response | [required] |

### Return type

[**crate::models::ResponseQueryResults**](ResponseQueryResults.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_responsemanagement_library

> crate::models::Library put_responsemanagement_library(library_id, body)
Update an existing response library.

Fields that can be updated: name. The most recent version is required for updates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**library_id** | **String** | Library ID | [required] |
**body** | [**Library**](Library.md) | Library | [required] |

### Return type

[**crate::models::Library**](Library.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_responsemanagement_response

> crate::models::Response put_responsemanagement_response(response_id, body, expand)
Update an existing response.

Fields that can be updated: name, libraries, and texts. The most recent version is required for updates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**response_id** | **String** | Response ID | [required] |
**body** | [**Response**](Response.md) | Response | [required] |
**expand** | Option<**String**> | Expand instructions for the return value. |  |

### Return type

[**crate::models::Response**](Response.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

