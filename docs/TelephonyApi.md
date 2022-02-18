# \TelephonyApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_telephony_siptraces**](TelephonyApi.md#get_telephony_siptraces) | **GET** /api/v2/telephony/siptraces | Fetch SIP metadata
[**get_telephony_siptraces_download_download_id**](TelephonyApi.md#get_telephony_siptraces_download_download_id) | **GET** /api/v2/telephony/siptraces/download/{downloadId} | Get signed S3 URL for a pcap download
[**post_telephony_siptraces_download**](TelephonyApi.md#post_telephony_siptraces_download) | **POST** /api/v2/telephony/siptraces/download | Request a download of a pcap file to S3



## get_telephony_siptraces

> crate::models::SipSearchResult get_telephony_siptraces(date_start, date_end, call_id, to_user, from_user, conversation_id)
Fetch SIP metadata

Fetch SIP metadata that matches a given parameter. If exactMatch is passed as a parameter only sip records that have exactly that value will be returned. For example, some records contain conversationId but not all relevant records for that call may contain the conversationId so only a partial view of the call will be reflected

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date_start** | **String** | Start date of the search. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [required] |
**date_end** | **String** | End date of the search. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [required] |
**call_id** | Option<**String**> | unique identification of the placed call |  |
**to_user** | Option<**String**> | User to who the call was placed |  |
**from_user** | Option<**String**> | user who placed the call |  |
**conversation_id** | Option<**String**> | Unique identification of the conversation |  |

### Return type

[**crate::models::SipSearchResult**](SipSearchResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_telephony_siptraces_download_download_id

> crate::models::SignedUrlResponse get_telephony_siptraces_download_download_id(download_id)
Get signed S3 URL for a pcap download

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**download_id** | **String** | unique id for the downloaded file in S3 | [required] |

### Return type

[**crate::models::SignedUrlResponse**](SignedUrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telephony_siptraces_download

> crate::models::SipDownloadResponse post_telephony_siptraces_download(sip_search_public_request)
Request a download of a pcap file to S3

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sip_search_public_request** | [**SipSearchPublicRequest**](SipSearchPublicRequest.md) |  | [required] |

### Return type

[**crate::models::SipDownloadResponse**](SipDownloadResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

