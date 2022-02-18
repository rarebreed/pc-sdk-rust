# \ScriptsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_script**](ScriptsApi.md#get_script) | **GET** /api/v2/scripts/{scriptId} | Get a script
[**get_script_page**](ScriptsApi.md#get_script_page) | **GET** /api/v2/scripts/{scriptId}/pages/{pageId} | Get a page
[**get_script_pages**](ScriptsApi.md#get_script_pages) | **GET** /api/v2/scripts/{scriptId}/pages | Get the list of pages
[**get_scripts**](ScriptsApi.md#get_scripts) | **GET** /api/v2/scripts | Get the list of scripts
[**get_scripts_published**](ScriptsApi.md#get_scripts_published) | **GET** /api/v2/scripts/published | Get the published scripts.
[**get_scripts_published_script_id**](ScriptsApi.md#get_scripts_published_script_id) | **GET** /api/v2/scripts/published/{scriptId} | Get the published script.
[**get_scripts_published_script_id_page**](ScriptsApi.md#get_scripts_published_script_id_page) | **GET** /api/v2/scripts/published/{scriptId}/pages/{pageId} | Get the published page.
[**get_scripts_published_script_id_pages**](ScriptsApi.md#get_scripts_published_script_id_pages) | **GET** /api/v2/scripts/published/{scriptId}/pages | Get the list of published pages
[**get_scripts_published_script_id_variables**](ScriptsApi.md#get_scripts_published_script_id_variables) | **GET** /api/v2/scripts/published/{scriptId}/variables | Get the published variables
[**get_scripts_upload_status**](ScriptsApi.md#get_scripts_upload_status) | **GET** /api/v2/scripts/uploads/{uploadId}/status | Get the upload status of an imported script
[**post_script_export**](ScriptsApi.md#post_script_export) | **POST** /api/v2/scripts/{scriptId}/export | Export a script via download service.



## get_script

> crate::models::Script get_script(script_id)
Get a script

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |

### Return type

[**crate::models::Script**](Script.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_script_page

> crate::models::Page get_script_page(script_id, page_id, script_data_version)
Get a page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |
**page_id** | **String** | Page ID | [required] |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_script_pages

> Vec<crate::models::Page> get_script_pages(script_id, script_data_version)
Get the list of pages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**Vec<crate::models::Page>**](Page.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scripts

> crate::models::ScriptEntityListing get_scripts(page_size, page_number, expand, name, feature, flow_id, sort_by, sort_order, script_data_version)
Get the list of scripts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand |  |
**name** | Option<**String**> | Name filter |  |
**feature** | Option<**String**> | Feature filter |  |
**flow_id** | Option<**String**> | Secure flow id filter |  |
**sort_by** | Option<**String**> | SortBy |  |
**sort_order** | Option<**String**> | SortOrder |  |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**crate::models::ScriptEntityListing**](ScriptEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scripts_published

> crate::models::ScriptEntityListing get_scripts_published(page_size, page_number, expand, name, feature, flow_id, script_data_version)
Get the published scripts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<**String**> | Expand |  |
**name** | Option<**String**> | Name filter |  |
**feature** | Option<**String**> | Feature filter |  |
**flow_id** | Option<**String**> | Secure flow id filter |  |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**crate::models::ScriptEntityListing**](ScriptEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scripts_published_script_id

> crate::models::Script get_scripts_published_script_id(script_id, script_data_version)
Get the published script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**crate::models::Script**](Script.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scripts_published_script_id_page

> crate::models::Page get_scripts_published_script_id_page(script_id, page_id, script_data_version)
Get the published page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |
**page_id** | **String** | Page ID | [required] |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**crate::models::Page**](Page.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scripts_published_script_id_pages

> Vec<crate::models::Page> get_scripts_published_script_id_pages(script_id, script_data_version)
Get the list of published pages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**Vec<crate::models::Page>**](Page.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scripts_published_script_id_variables

> serde_json::Value get_scripts_published_script_id_variables(script_id, input, output, _type, script_data_version)
Get the published variables

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |
**input** | Option<**String**> | input |  |
**output** | Option<**String**> | output |  |
**_type** | Option<**String**> | type |  |
**script_data_version** | Option<**String**> | Advanced usage - controls the data version of the script |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scripts_upload_status

> crate::models::ImportScriptStatusResponse get_scripts_upload_status(upload_id, long_poll)
Get the upload status of an imported script

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**upload_id** | **String** | Upload ID | [required] |
**long_poll** | Option<**bool**> | Enable longPolling endpoint |  |[default to false]

### Return type

[**crate::models::ImportScriptStatusResponse**](ImportScriptStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_script_export

> crate::models::ExportScriptResponse post_script_export(script_id, body)
Export a script via download service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**script_id** | **String** | Script ID | [required] |
**body** | Option<[**ExportScriptRequest**](ExportScriptRequest.md)> |  |  |

### Return type

[**crate::models::ExportScriptResponse**](ExportScriptResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

