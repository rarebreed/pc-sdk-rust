# KnowledgeImport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Id of the import operation | [optional][readonly]
**name** | Option<**String**> | Name of the import operation | [optional]
**upload_key** | **String** | Upload key | 
**file_type** | **String** | file type of the document | 
**ignore_headers** | Option<**bool**> | Ignore headers for the specified file | [optional]
**status** | Option<**String**> | Status of the operation | [optional][readonly]
**report** | Option<[**crate::models::ImportReport**](ImportReport.md)> |  | [optional]
**knowledge_base** | Option<[**crate::models::KnowledgeBase**](KnowledgeBase.md)> |  | [optional]
**language_code** | Option<**String**> | Language code | [optional][readonly]
**date_created** | Option<**String**> | Created date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


