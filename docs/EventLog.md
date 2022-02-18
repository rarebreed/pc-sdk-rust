# EventLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**error_entity** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**related_entity** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**timestamp** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**level** | Option<**String**> |  | [optional]
**category** | Option<**String**> |  | [optional]
**correlation_id** | Option<**String**> |  | [optional]
**event_message** | Option<[**crate::models::EventMessage**](EventMessage.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


