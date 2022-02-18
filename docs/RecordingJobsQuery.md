# RecordingJobsQuery

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** | Operation to perform bulk task | 
**action_date** | **String** | The date when the action will be performed. If the operation will cause the delete date of a recording to be older than the export date, the export date will be adjusted to the delete date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | 
**integration_id** | Option<**String**> | IntegrationId to Access AWS S3 bucket for bulk recording exports. This field is required and used only for EXPORT action. | [optional]
**include_screen_recordings** | Option<**bool**> | Include Screen recordings for export action, default value = true  | [optional]
**conversation_query** | [**crate::models::AsyncConversationQuery**](AsyncConversationQuery.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


