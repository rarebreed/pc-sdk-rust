# InboundRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**pattern** | **String** | The search pattern that the mailbox name should match. | 
**queue** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**priority** | Option<**i32**> | The priority to use for routing. | [optional]
**skills** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | The skills to use for routing. | [optional]
**language** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**from_name** | **String** | The sender name to use for outgoing replies. | 
**from_email** | Option<**String**> | The sender email to use for outgoing replies. | [optional]
**flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**reply_email_address** | Option<[**crate::models::QueueEmailAddress**](QueueEmailAddress.md)> |  | [optional]
**auto_bcc** | Option<[**Vec<crate::models::EmailAddress>**](EmailAddress.md)> | The recipients that should be  automatically blind copied on outbound emails associated with this InboundRoute. | [optional]
**spam_flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


