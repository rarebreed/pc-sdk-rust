# DocumentAudit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**user** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**workspace** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**transaction_id** | Option<**String**> |  | [optional]
**transaction_initiator** | Option<**bool**> |  | [optional]
**application** | Option<**String**> |  | [optional]
**service_name** | Option<**String**> |  | [optional]
**level** | Option<**String**> |  | [optional]
**timestamp** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**status** | Option<**String**> |  | [optional]
**action_context** | Option<**String**> |  | [optional]
**action** | Option<**String**> |  | [optional]
**entity** | Option<[**crate::models::AuditEntityReference**](AuditEntityReference.md)> |  | [optional]
**changes** | Option<[**Vec<crate::models::AuditChange>**](AuditChange.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


