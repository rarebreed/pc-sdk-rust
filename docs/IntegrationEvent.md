# IntegrationEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique ID for this event | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**correlation_id** | Option<**String**> | Correlation ID for the event | [optional][readonly]
**timestamp** | Option<**String**> | Time the event occurred. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**level** | Option<**String**> | Indicates the severity of the event. | [optional][readonly]
**event_code** | Option<**String**> | A classification for the event. Suitable for programmatic searching, sorting, or filtering | [optional][readonly]
**message** | Option<[**crate::models::MessageInfo**](MessageInfo.md)> |  | [optional]
**entities** | Option<[**Vec<crate::models::EventEntity>**](EventEntity.md)> | Collection of entities affected by or pertaining to the event (e.g. a list of Integrations or Bridge connectors) | [optional][readonly]
**context_attributes** | Option<**::std::collections::HashMap<String, String>**> | Map of context attributes specific to this event. | [optional][readonly]
**detail_message** | Option<[**crate::models::MessageInfo**](MessageInfo.md)> |  | [optional]
**user** | Option<[**crate::models::User**](User.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


