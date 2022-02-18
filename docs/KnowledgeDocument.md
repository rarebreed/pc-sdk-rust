# KnowledgeDocument

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**language_code** | **String** | Language of the document | 
**_type** | **String** | Document type | 
**faq** | Option<[**crate::models::DocumentFaq**](DocumentFaq.md)> |  | [optional]
**date_created** | Option<**String**> | Document creation date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Document last modification date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**categories** | Option<[**Vec<crate::models::KnowledgeCategory>**](KnowledgeCategory.md)> | Document categories | [optional]
**knowledge_base** | Option<[**crate::models::KnowledgeBase**](KnowledgeBase.md)> |  | [optional]
**external_url** | Option<**String**> | External URL to the document | [optional]
**article** | Option<[**crate::models::DocumentArticle**](DocumentArticle.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


