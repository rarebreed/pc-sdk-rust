# \LanguagesApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_language**](LanguagesApi.md#delete_language) | **DELETE** /api/v2/languages/{languageId} | Delete Language (Deprecated)
[**delete_routing_language**](LanguagesApi.md#delete_routing_language) | **DELETE** /api/v2/routing/languages/{languageId} | Delete Language
[**get_language**](LanguagesApi.md#get_language) | **GET** /api/v2/languages/{languageId} | Get language (Deprecated)
[**get_languages**](LanguagesApi.md#get_languages) | **GET** /api/v2/languages | Get the list of supported languages. (Deprecated)
[**get_languages_translations**](LanguagesApi.md#get_languages_translations) | **GET** /api/v2/languages/translations | Get all available languages for translation
[**get_languages_translations_builtin**](LanguagesApi.md#get_languages_translations_builtin) | **GET** /api/v2/languages/translations/builtin | Get the builtin translation for a language
[**get_languages_translations_organization**](LanguagesApi.md#get_languages_translations_organization) | **GET** /api/v2/languages/translations/organization | Get effective translation for an organization by language
[**get_languages_translations_user**](LanguagesApi.md#get_languages_translations_user) | **GET** /api/v2/languages/translations/users/{userId} | Get effective language translation for a user
[**get_routing_language**](LanguagesApi.md#get_routing_language) | **GET** /api/v2/routing/languages/{languageId} | Get language
[**post_languages**](LanguagesApi.md#post_languages) | **POST** /api/v2/languages | Create Language (Deprecated)



## delete_language

> delete_language(language_id)
Delete Language (Deprecated)

This endpoint is deprecated. It has been moved to /routing/languages/{languageId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language_id** | **String** | Language ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_language

> delete_routing_language(language_id)
Delete Language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language_id** | **String** | Language ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_language

> crate::models::Language get_language(language_id)
Get language (Deprecated)

This endpoint is deprecated. It has been moved to /routing/languages/{languageId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language_id** | **String** | Language ID | [required] |

### Return type

[**crate::models::Language**](Language.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages

> crate::models::LanguageEntityListing get_languages(page_size, page_number, sort_order, name)
Get the list of supported languages. (Deprecated)

This endpoint is deprecated. It has been moved to /routing/languages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]
**name** | Option<**String**> | Name |  |

### Return type

[**crate::models::LanguageEntityListing**](LanguageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages_translations

> crate::models::AvailableTranslations get_languages_translations()
Get all available languages for translation

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AvailableTranslations**](AvailableTranslations.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages_translations_builtin

> ::std::collections::HashMap<String, serde_json::Value> get_languages_translations_builtin(language)
Get the builtin translation for a language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | The language of the builtin translation to retrieve | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages_translations_organization

> ::std::collections::HashMap<String, serde_json::Value> get_languages_translations_organization(language)
Get effective translation for an organization by language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language** | **String** | The language of the translation to retrieve for the organization | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_languages_translations_user

> ::std::collections::HashMap<String, serde_json::Value> get_languages_translations_user(user_id)
Get effective language translation for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The user id | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_language

> crate::models::Language get_routing_language(language_id)
Get language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**language_id** | **String** | Language ID | [required] |

### Return type

[**crate::models::Language**](Language.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_languages

> crate::models::Language post_languages(body)
Create Language (Deprecated)

This endpoint is deprecated. It has been moved to /routing/languages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Language**](Language.md) | Language | [required] |

### Return type

[**crate::models::Language**](Language.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

