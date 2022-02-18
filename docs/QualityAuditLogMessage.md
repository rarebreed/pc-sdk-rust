# QualityAuditLogMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Id of the audit message. | [optional]
**user_home_org_id** | Option<**String**> | Home Organization Id associated with this audit message. | [optional]
**user_trustee_org_id** | Option<**String**> | Trustee Organization Id if this audit message is from trustee access. | [optional]
**user** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**client** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**remote_ips** | Option<**Vec<String>**> | List of IP addresses of systems that originated or handled the request. | [optional]
**service_name** | Option<**String**> | Name of the service that logged this audit message. | [optional]
**level** | Option<**String**> | The level of this audit message. | [optional]
**status** | Option<**String**> | The status of the action of this audit message. | [optional]
**event_date** | Option<**String**> | Date and time of when the audit message was logged. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**message_info** | Option<[**crate::models::MessageInfo**](MessageInfo.md)> |  | [optional]
**action** | Option<**String**> | Action that took place. | [optional]
**entity** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**entity_type** | Option<**String**> | Type of the entity that was impacted. | [optional]
**property_changes** | Option<[**Vec<crate::models::PropertyChange>**](PropertyChange.md)> | List of properties that were changed and changes made to those properties. | [optional]
**context** | Option<**::std::collections::HashMap<String, String>**> | Additional context for this message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


