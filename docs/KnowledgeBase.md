# KnowledgeBase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> | Knowledge base description | [optional]
**core_language** | **String** | Core language for knowledge base in which initial content must be created first | 
**date_created** | Option<**String**> | Knowledge base creation date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Knowledge base last modification date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**faq_count** | Option<**i32**> | The count representing the number of documents of type FAQ in the KnowledgeBase | [optional][readonly]
**date_document_last_modified** | Option<**String**> | The date representing when the last document is modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**article_count** | Option<**i32**> | The count representing the number of documents of type Article in the KnowledgeBase | [optional][readonly]
**published** | Option<**bool**> | Flag that indicates the knowledge base is published | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


