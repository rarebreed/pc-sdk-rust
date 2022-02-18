# KnowledgeExtendedCategory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | Category name | 
**description** | Option<**String**> | Category description | [optional]
**knowledge_base** | Option<[**crate::models::KnowledgeBase**](KnowledgeBase.md)> |  | [optional]
**language_code** | Option<**String**> | Actual language of the category | [optional][readonly]
**date_created** | Option<**String**> | Category creation date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Category last modification date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**parent** | Option<[**crate::models::KnowledgeCategory**](KnowledgeCategory.md)> |  | [optional]
**children** | Option<[**Vec<crate::models::KnowledgeCategory>**](KnowledgeCategory.md)> | Category children | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


