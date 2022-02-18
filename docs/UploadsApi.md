# \UploadsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_knowledge_documentuploads**](UploadsApi.md#post_knowledge_documentuploads) | **POST** /api/v2/knowledge/documentuploads | Creates a presigned URL for uploading a knowledge import file with a set of documents
[**post_languageunderstanding_miner_uploads**](UploadsApi.md#post_languageunderstanding_miner_uploads) | **POST** /api/v2/languageunderstanding/miners/{minerId}/uploads | Creates a presigned URL for uploading a chat corpus which will be used for mining by intent miner
[**post_uploads_publicassets_images**](UploadsApi.md#post_uploads_publicassets_images) | **POST** /api/v2/uploads/publicassets/images | Creates presigned url for uploading a public asset image
[**post_uploads_recordings**](UploadsApi.md#post_uploads_recordings) | **POST** /api/v2/uploads/recordings | Creates presigned url for uploading a recording file
[**post_uploads_workforcemanagement_historicaldata_csv**](UploadsApi.md#post_uploads_workforcemanagement_historicaldata_csv) | **POST** /api/v2/uploads/workforcemanagement/historicaldata/csv | Creates presigned url for uploading WFM historical data file. Requires data in csv format.
[**post_uploads_workforcemanagement_historicaldata_json**](UploadsApi.md#post_uploads_workforcemanagement_historicaldata_json) | **POST** /api/v2/uploads/workforcemanagement/historicaldata/json | Creates presigned url for uploading WFM historical data file. Requires data in json format.



## post_knowledge_documentuploads

> crate::models::UploadUrlResponse post_knowledge_documentuploads(body)
Creates a presigned URL for uploading a knowledge import file with a set of documents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UploadUrlRequest**](UploadUrlRequest.md) | query | [required] |

### Return type

[**crate::models::UploadUrlResponse**](UploadUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languageunderstanding_miner_uploads

> crate::models::UploadUrlResponse post_languageunderstanding_miner_uploads(miner_id, body)
Creates a presigned URL for uploading a chat corpus which will be used for mining by intent miner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**miner_id** | **String** | Miner ID | [required] |
**body** | **serde_json::Value** | query | [required] |

### Return type

[**crate::models::UploadUrlResponse**](UploadUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_uploads_publicassets_images

> crate::models::UploadUrlResponse post_uploads_publicassets_images(body)
Creates presigned url for uploading a public asset image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UploadUrlRequest**](UploadUrlRequest.md) | query | [required] |

### Return type

[**crate::models::UploadUrlResponse**](UploadUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_uploads_recordings

> crate::models::UploadUrlResponse post_uploads_recordings(body)
Creates presigned url for uploading a recording file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UploadUrlRequest**](UploadUrlRequest.md) | query | [required] |

### Return type

[**crate::models::UploadUrlResponse**](UploadUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_uploads_workforcemanagement_historicaldata_csv

> crate::models::UploadUrlResponse post_uploads_workforcemanagement_historicaldata_csv(body)
Creates presigned url for uploading WFM historical data file. Requires data in csv format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UploadUrlRequest**](UploadUrlRequest.md) | query | [required] |

### Return type

[**crate::models::UploadUrlResponse**](UploadUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_uploads_workforcemanagement_historicaldata_json

> crate::models::UploadUrlResponse post_uploads_workforcemanagement_historicaldata_json(body)
Creates presigned url for uploading WFM historical data file. Requires data in json format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UploadUrlRequest**](UploadUrlRequest.md) | query | [required] |

### Return type

[**crate::models::UploadUrlResponse**](UploadUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

