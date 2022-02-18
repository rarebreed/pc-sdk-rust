# \FaxApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_fax_document**](FaxApi.md#delete_fax_document) | **DELETE** /api/v2/fax/documents/{documentId} | Delete a fax document.
[**get_fax_document**](FaxApi.md#get_fax_document) | **GET** /api/v2/fax/documents/{documentId} | Get a document.
[**get_fax_document_content**](FaxApi.md#get_fax_document_content) | **GET** /api/v2/fax/documents/{documentId}/content | Download a fax document.
[**get_fax_documents**](FaxApi.md#get_fax_documents) | **GET** /api/v2/fax/documents | Get a list of fax documents.
[**get_fax_summary**](FaxApi.md#get_fax_summary) | **GET** /api/v2/fax/summary | Get fax summary
[**put_fax_document**](FaxApi.md#put_fax_document) | **PUT** /api/v2/fax/documents/{documentId} | Update a fax document.



## delete_fax_document

> delete_fax_document(document_id)
Delete a fax document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fax_document

> crate::models::FaxDocument get_fax_document(document_id)
Get a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |

### Return type

[**crate::models::FaxDocument**](FaxDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fax_document_content

> crate::models::DownloadResponse get_fax_document_content(document_id)
Download a fax document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |

### Return type

[**crate::models::DownloadResponse**](DownloadResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fax_documents

> crate::models::FaxDocumentEntityListing get_fax_documents(page_size, page_number)
Get a list of fax documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::FaxDocumentEntityListing**](FaxDocumentEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fax_summary

> crate::models::FaxSummary get_fax_summary()
Get fax summary

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


## put_fax_document

> crate::models::FaxDocument put_fax_document(document_id, body)
Update a fax document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**body** | [**FaxDocument**](FaxDocument.md) | Document | [required] |

### Return type

[**crate::models::FaxDocument**](FaxDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

