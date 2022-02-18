# OrphanUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**archive_date** | Option<**String**> | The orphan recording's archive date. Must be greater than 1 day from now if set. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**delete_date** | Option<**String**> | The orphan recording's delete date. Must be greater than archiveDate and exportDate if set, otherwise one day from now. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**export_date** | Option<**String**> | The orphan recording's export date. Must be greater than 1 day from now if set. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**integration_id** | Option<**String**> | IntegrationId to access AWS S3 bucket for export. This field is required if exportDate is set. | [optional]
**conversation_id** | Option<**String**> | A conversation Id that this orphan's recording is to be attached to. If not present, the conversationId will be deduced from the recording media. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


