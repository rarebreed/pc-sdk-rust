# \KnowledgeApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_knowledge_knowledgebase**](KnowledgeApi.md#delete_knowledge_knowledgebase) | **DELETE** /api/v2/knowledge/knowledgebases/{knowledgeBaseId} | Delete knowledge base
[**delete_knowledge_knowledgebase_language_category**](KnowledgeApi.md#delete_knowledge_knowledgebase_language_category) | **DELETE** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/categories/{categoryId} | Delete category
[**delete_knowledge_knowledgebase_language_document**](KnowledgeApi.md#delete_knowledge_knowledgebase_language_document) | **DELETE** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents/{documentId} | Delete document
[**delete_knowledge_knowledgebase_language_documents_import**](KnowledgeApi.md#delete_knowledge_knowledgebase_language_documents_import) | **DELETE** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents/imports/{importId} | Delete import operation
[**get_knowledge_knowledgebase**](KnowledgeApi.md#get_knowledge_knowledgebase) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId} | Get knowledge base
[**get_knowledge_knowledgebase_language_categories**](KnowledgeApi.md#get_knowledge_knowledgebase_language_categories) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/categories | Get categories
[**get_knowledge_knowledgebase_language_category**](KnowledgeApi.md#get_knowledge_knowledgebase_language_category) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/categories/{categoryId} | Get category
[**get_knowledge_knowledgebase_language_document**](KnowledgeApi.md#get_knowledge_knowledgebase_language_document) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents/{documentId} | Get document
[**get_knowledge_knowledgebase_language_documents**](KnowledgeApi.md#get_knowledge_knowledgebase_language_documents) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents | Get documents
[**get_knowledge_knowledgebase_language_documents_import**](KnowledgeApi.md#get_knowledge_knowledgebase_language_documents_import) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents/imports/{importId} | Get import operation report
[**get_knowledge_knowledgebase_language_training**](KnowledgeApi.md#get_knowledge_knowledgebase_language_training) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/trainings/{trainingId} | Get training detail
[**get_knowledge_knowledgebase_language_trainings**](KnowledgeApi.md#get_knowledge_knowledgebase_language_trainings) | **GET** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/trainings | Get all trainings information for a knowledgebase
[**get_knowledge_knowledgebases**](KnowledgeApi.md#get_knowledge_knowledgebases) | **GET** /api/v2/knowledge/knowledgebases | Get knowledge bases
[**patch_knowledge_knowledgebase**](KnowledgeApi.md#patch_knowledge_knowledgebase) | **PATCH** /api/v2/knowledge/knowledgebases/{knowledgeBaseId} | Update knowledge base
[**patch_knowledge_knowledgebase_language_category**](KnowledgeApi.md#patch_knowledge_knowledgebase_language_category) | **PATCH** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/categories/{categoryId} | Update category
[**patch_knowledge_knowledgebase_language_document**](KnowledgeApi.md#patch_knowledge_knowledgebase_language_document) | **PATCH** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents/{documentId} | Update document
[**patch_knowledge_knowledgebase_language_documents**](KnowledgeApi.md#patch_knowledge_knowledgebase_language_documents) | **PATCH** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents | Update documents collection
[**patch_knowledge_knowledgebase_language_documents_import**](KnowledgeApi.md#patch_knowledge_knowledgebase_language_documents_import) | **PATCH** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents/imports/{importId} | Start import operation
[**post_knowledge_documentuploads**](KnowledgeApi.md#post_knowledge_documentuploads) | **POST** /api/v2/knowledge/documentuploads | Creates a presigned URL for uploading a knowledge import file with a set of documents
[**post_knowledge_knowledgebase_language_categories**](KnowledgeApi.md#post_knowledge_knowledgebase_language_categories) | **POST** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/categories | Create new category
[**post_knowledge_knowledgebase_language_documents**](KnowledgeApi.md#post_knowledge_knowledgebase_language_documents) | **POST** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents | Create document
[**post_knowledge_knowledgebase_language_documents_imports**](KnowledgeApi.md#post_knowledge_knowledgebase_language_documents_imports) | **POST** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/documents/imports | Create import operation
[**post_knowledge_knowledgebase_language_training_promote**](KnowledgeApi.md#post_knowledge_knowledgebase_language_training_promote) | **POST** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/trainings/{trainingId}/promote | Promote trained documents from draft state to active.
[**post_knowledge_knowledgebase_language_trainings**](KnowledgeApi.md#post_knowledge_knowledgebase_language_trainings) | **POST** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/languages/{languageCode}/trainings | Trigger training
[**post_knowledge_knowledgebase_search**](KnowledgeApi.md#post_knowledge_knowledgebase_search) | **POST** /api/v2/knowledge/knowledgebases/{knowledgeBaseId}/search | Search Documents
[**post_knowledge_knowledgebases**](KnowledgeApi.md#post_knowledge_knowledgebases) | **POST** /api/v2/knowledge/knowledgebases | Create new knowledge base



## delete_knowledge_knowledgebase

> crate::models::KnowledgeBase delete_knowledge_knowledgebase(knowledge_base_id)
Delete knowledge base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |

### Return type

[**crate::models::KnowledgeBase**](KnowledgeBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_knowledge_knowledgebase_language_category

> crate::models::KnowledgeCategory delete_knowledge_knowledgebase_language_category(category_id, knowledge_base_id, language_code)
Delete category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | Category ID | [required] |
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |

### Return type

[**crate::models::KnowledgeCategory**](KnowledgeCategory.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_knowledge_knowledgebase_language_document

> crate::models::KnowledgeDocument delete_knowledge_knowledgebase_language_document(document_id, knowledge_base_id, language_code)
Delete document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |

### Return type

[**crate::models::KnowledgeDocument**](KnowledgeDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_knowledge_knowledgebase_language_documents_import

> delete_knowledge_knowledgebase_language_documents_import(knowledge_base_id, language_code, import_id)
Delete import operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**import_id** | **String** | Import ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase

> crate::models::KnowledgeBase get_knowledge_knowledgebase(knowledge_base_id)
Get knowledge base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |

### Return type

[**crate::models::KnowledgeBase**](KnowledgeBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase_language_categories

> crate::models::CategoryListing get_knowledge_knowledgebase_language_categories(knowledge_base_id, language_code, before, after, limit, page_size, name)
Get categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**before** | Option<**String**> | The cursor that points to the start of the set of entities that has been returned. |  |
**after** | Option<**String**> | The cursor that points to the end of the set of entities that has been returned. |  |
**limit** | Option<**String**> | Number of entities to return. Maximum of 200. Deprecated in favour of pageSize, use CursorQueryParameters instead. |  |
**page_size** | Option<**String**> | Number of entities to return. Maximum of 200. |  |
**name** | Option<**String**> | Filter to return the categories that starts with the given category name. |  |

### Return type

[**crate::models::CategoryListing**](CategoryListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase_language_category

> crate::models::KnowledgeExtendedCategory get_knowledge_knowledgebase_language_category(category_id, knowledge_base_id, language_code)
Get category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | Category ID | [required] |
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |

### Return type

[**crate::models::KnowledgeExtendedCategory**](KnowledgeExtendedCategory.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase_language_document

> crate::models::KnowledgeDocument get_knowledge_knowledgebase_language_document(document_id, knowledge_base_id, language_code)
Get document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |

### Return type

[**crate::models::KnowledgeDocument**](KnowledgeDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase_language_documents

> crate::models::DocumentListing get_knowledge_knowledgebase_language_documents(knowledge_base_id, language_code, before, after, limit, page_size, categories, title, sort_by, sort_order, document_ids)
Get documents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**before** | Option<**String**> | The cursor that points to the start of the set of entities that has been returned. |  |
**after** | Option<**String**> | The cursor that points to the end of the set of entities that has been returned. |  |
**limit** | Option<**String**> | Number of entities to return. Maximum of 200. Deprecated in favour of pageSize, use CursorQueryParameters instead. |  |
**page_size** | Option<**String**> | Number of entities to return. Maximum of 200. |  |
**categories** | Option<**String**> | Filter by categories ids, comma separated values expected. |  |
**title** | Option<**String**> | Filter by document title. |  |
**sort_by** | Option<**String**> | Sort by. |  |
**sort_order** | Option<**String**> | Sort Order. |  |
**document_ids** | Option<[**Vec<String>**](String.md)> | Comma-separated list of document identifiers to fetch by. |  |

### Return type

[**crate::models::DocumentListing**](DocumentListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase_language_documents_import

> crate::models::KnowledgeImport get_knowledge_knowledgebase_language_documents_import(knowledge_base_id, language_code, import_id)
Get import operation report

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**import_id** | **String** | Import ID | [required] |

### Return type

[**crate::models::KnowledgeImport**](KnowledgeImport.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase_language_training

> crate::models::KnowledgeTraining get_knowledge_knowledgebase_language_training(knowledge_base_id, language_code, training_id)
Get training detail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**training_id** | **String** | Training ID | [required] |

### Return type

[**crate::models::KnowledgeTraining**](KnowledgeTraining.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebase_language_trainings

> crate::models::TrainingListing get_knowledge_knowledgebase_language_trainings(knowledge_base_id, language_code, before, after, limit, page_size, knowledge_documents_state)
Get all trainings information for a knowledgebase

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**before** | Option<**String**> | The cursor that points to the start of the set of entities that has been returned. |  |
**after** | Option<**String**> | The cursor that points to the end of the set of entities that has been returned. |  |
**limit** | Option<**String**> | Number of entities to return. Maximum of 200. Deprecated in favour of pageSize, use CursorQueryParameters instead. |  |
**page_size** | Option<**String**> | Number of entities to return. Maximum of 200. |  |
**knowledge_documents_state** | Option<**String**> | Return the training with the specified state of the trained documents. |  |

### Return type

[**crate::models::TrainingListing**](TrainingListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_knowledge_knowledgebases

> crate::models::KnowledgeBaseListing get_knowledge_knowledgebases(before, after, limit, page_size, name, core_language, published, sort_by, sort_order)
Get knowledge bases

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | The cursor that points to the start of the set of entities that has been returned. |  |
**after** | Option<**String**> | The cursor that points to the end of the set of entities that has been returned. |  |
**limit** | Option<**String**> | Number of entities to return. Maximum of 200. Deprecated in favour of pageSize, use CursorQueryParameters instead. |  |
**page_size** | Option<**String**> | Number of entities to return. Maximum of 200. |  |
**name** | Option<**String**> | Filter by Name. |  |
**core_language** | Option<**String**> | Filter by core language. |  |
**published** | Option<**bool**> | Filter by published status. |  |
**sort_by** | Option<**String**> | Sort by. |  |
**sort_order** | Option<**String**> | Sort Order. |  |

### Return type

[**crate::models::KnowledgeBaseListing**](KnowledgeBaseListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_knowledge_knowledgebase

> crate::models::KnowledgeBase patch_knowledge_knowledgebase(knowledge_base_id, body)
Update knowledge base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**body** | [**KnowledgeBase**](KnowledgeBase.md) |  | [required] |

### Return type

[**crate::models::KnowledgeBase**](KnowledgeBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_knowledge_knowledgebase_language_category

> crate::models::KnowledgeExtendedCategory patch_knowledge_knowledgebase_language_category(category_id, knowledge_base_id, language_code, body)
Update category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **String** | Category ID | [required] |
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**body** | [**KnowledgeCategoryRequest**](KnowledgeCategoryRequest.md) |  | [required] |

### Return type

[**crate::models::KnowledgeExtendedCategory**](KnowledgeExtendedCategory.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_knowledge_knowledgebase_language_document

> crate::models::KnowledgeDocument patch_knowledge_knowledgebase_language_document(document_id, knowledge_base_id, language_code, body)
Update document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**body** | [**KnowledgeDocumentRequest**](KnowledgeDocumentRequest.md) |  | [required] |

### Return type

[**crate::models::KnowledgeDocument**](KnowledgeDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_knowledge_knowledgebase_language_documents

> crate::models::DocumentListing patch_knowledge_knowledgebase_language_documents(knowledge_base_id, language_code, body)
Update documents collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**body** | [**Vec<crate::models::KnowledgeDocumentBulkRequest>**](KnowledgeDocumentBulkRequest.md) |  | [required] |

### Return type

[**crate::models::DocumentListing**](DocumentListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_knowledge_knowledgebase_language_documents_import

> crate::models::KnowledgeImport patch_knowledge_knowledgebase_language_documents_import(knowledge_base_id, language_code, import_id, body)
Start import operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**import_id** | **String** | Import ID | [required] |
**body** | [**ImportStatusRequest**](ImportStatusRequest.md) |  | [required] |

### Return type

[**crate::models::KnowledgeImport**](KnowledgeImport.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## post_knowledge_knowledgebase_language_categories

> crate::models::KnowledgeExtendedCategory post_knowledge_knowledgebase_language_categories(knowledge_base_id, language_code, body)
Create new category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**body** | [**KnowledgeCategoryRequest**](KnowledgeCategoryRequest.md) |  | [required] |

### Return type

[**crate::models::KnowledgeExtendedCategory**](KnowledgeExtendedCategory.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_knowledge_knowledgebase_language_documents

> crate::models::KnowledgeDocument post_knowledge_knowledgebase_language_documents(knowledge_base_id, language_code, body)
Create document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**body** | [**KnowledgeDocumentRequest**](KnowledgeDocumentRequest.md) |  | [required] |

### Return type

[**crate::models::KnowledgeDocument**](KnowledgeDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_knowledge_knowledgebase_language_documents_imports

> crate::models::KnowledgeImport post_knowledge_knowledgebase_language_documents_imports(knowledge_base_id, language_code, body)
Create import operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**body** | [**KnowledgeImport**](KnowledgeImport.md) |  | [required] |

### Return type

[**crate::models::KnowledgeImport**](KnowledgeImport.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_knowledge_knowledgebase_language_training_promote

> crate::models::KnowledgeTraining post_knowledge_knowledgebase_language_training_promote(knowledge_base_id, language_code, training_id)
Promote trained documents from draft state to active.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |
**training_id** | **String** | Training ID | [required] |

### Return type

[**crate::models::KnowledgeTraining**](KnowledgeTraining.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_knowledge_knowledgebase_language_trainings

> crate::models::KnowledgeTraining post_knowledge_knowledgebase_language_trainings(knowledge_base_id, language_code)
Trigger training

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**language_code** | **String** | Language code, format: iso2-LOCALE | [required] |

### Return type

[**crate::models::KnowledgeTraining**](KnowledgeTraining.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_knowledge_knowledgebase_search

> crate::models::KnowledgeSearchResponse post_knowledge_knowledgebase_search(knowledge_base_id, body)
Search Documents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**knowledge_base_id** | **String** | Knowledge base ID | [required] |
**body** | Option<[**KnowledgeSearchRequest**](KnowledgeSearchRequest.md)> |  |  |

### Return type

[**crate::models::KnowledgeSearchResponse**](KnowledgeSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_knowledge_knowledgebases

> crate::models::KnowledgeBase post_knowledge_knowledgebases(body)
Create new knowledge base

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**KnowledgeBase**](KnowledgeBase.md) |  | [required] |

### Return type

[**crate::models::KnowledgeBase**](KnowledgeBase.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

