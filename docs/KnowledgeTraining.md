# KnowledgeTraining

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**date_triggered** | Option<**String**> | Trigger date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_completed** | Option<**String**> | Training completed date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**status** | Option<**String**> | Training status. | [optional][readonly]
**language_code** | Option<**String**> | Language of the documents that are trained. | [optional][readonly]
**knowledge_base** | Option<[**crate::models::KnowledgeBase**](KnowledgeBase.md)> |  | [optional]
**error_message** | Option<**String**> | Any error message during the Training or Promote action. | [optional][readonly]
**knowledge_documents_state** | Option<**String**> | State of the Trained Documents, which can be one of these Draft, Active, Discarded, Archived. | [optional][readonly]
**date_promoted** | Option<**String**> | Trained Documents Promoted date-time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


