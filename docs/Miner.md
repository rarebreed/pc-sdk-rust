# Miner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | Chat Corpus Name. | 
**language** | Option<**String**> | Language Localization code. | [optional]
**date_created** | Option<**String**> | Date when the miner was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**status** | Option<**String**> | Status of the miner. | [optional][readonly]
**conversations_date_range_start** | Option<[**String**](string.md)> | Date from which the conversations need to be taken for mining. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**conversations_date_range_end** | Option<[**String**](string.md)> | Date till which the conversations need to be taken for mining. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**date_completed** | Option<**String**> | Date when the mining process was completed. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**message** | Option<**String**> | Mining message if present. | [optional][readonly]
**conversation_data_uploaded** | Option<**bool**> | Flag to indicate whether data file to be mined was uploaded. | [optional][readonly]
**media_type** | Option<**String**> | Media type for filtering conversations. | [optional][readonly]
**queue_ids** | Option<**Vec<String>**> | List of queue IDs for filtering conversations. | [optional][readonly]
**date_triggered** | Option<**String**> | Date when the miner started execution. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Date when the miner was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**latest_draft_version** | Option<[**crate::models::Draft**](Draft.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


