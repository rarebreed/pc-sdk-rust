# AuditMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | AuditMessage ID. | 
**user** | Option<[**crate::models::AuditUser**](AuditUser.md)> |  | [optional]
**correlation_id** | Option<**String**> | Correlation ID. | [optional]
**transaction_id** | Option<**String**> | Transaction ID. | [optional]
**transaction_initiator** | Option<**bool**> | Whether or not this audit can be considered the initiator of the transaction it is a part of. | [optional]
**application** | Option<**String**> | The application through which the action of this AuditMessage was initiated. | [optional]
**service_name** | **String** | The name of the service which sent this AuditMessage. | 
**level** | **String** | The level of this audit. USER or SYSTEM. | 
**timestamp** | Option<**String**> | The time at which the action of this AuditMessage was initiated. | [optional]
**received_timestamp** | **String** | The time at which this AuditMessage was received. | 
**status** | **String** | The status of the action of this AuditMessage | 
**action_context** | Option<**String**> | The context of a system-level action | [optional]
**action** | Option<**String**> | A string representing the action that took place | [optional]
**changes** | Option<[**Vec<crate::models::Change>**](Change.md)> | Details about any changes that occurred in this audit | [optional]
**entity** | Option<[**crate::models::AuditEntity**](AuditEntity.md)> |  | [optional]
**service_context** | Option<[**crate::models::ServiceContext**](ServiceContext.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


