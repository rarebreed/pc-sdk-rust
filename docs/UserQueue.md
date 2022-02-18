# UserQueue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**description** | Option<**String**> | The queue description. | [optional]
**date_created** | Option<**String**> | The date the queue was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | The date of the last modification to the queue. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_by** | Option<**String**> | The ID of the user that last modified the queue. | [optional]
**created_by** | Option<**String**> | The ID of the user that created the queue. | [optional]
**member_count** | Option<**i32**> | The total number of members in the queue. | [optional][readonly]
**user_member_count** | Option<**i32**> | The number of user members (i.e., non-group members) in the queue. | [optional][readonly]
**joined_member_count** | Option<**i32**> | The number of joined members in the queue. | [optional][readonly]
**media_settings** | Option<[**::std::collections::HashMap<String, crate::models::MediaSetting>**](MediaSetting.md)> | The media settings for the queue. Valid key values: CALL, CALLBACK, CHAT, EMAIL, MESSAGE, SOCIAL_EXPRESSION, VIDEO_COMM | [optional]
**routing_rules** | Option<[**Vec<crate::models::RoutingRule>**](RoutingRule.md)> | The routing rules for the queue, used for routing to known or preferred agents. | [optional]
**bullseye** | Option<[**crate::models::Bullseye**](Bullseye.md)> |  | [optional]
**acw_settings** | Option<[**crate::models::AcwSettings**](AcwSettings.md)> |  | [optional]
**skill_evaluation_method** | Option<**String**> | The skill evaluation method to use when routing conversations. | [optional]
**queue_flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**email_in_queue_flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**message_in_queue_flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**whisper_prompt** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**on_hold_prompt** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**enable_transcription** | Option<**bool**> | Indicates whether voice transcription is enabled for this queue. | [optional]
**enable_manual_assignment** | Option<**bool**> | Indicates whether manual assignment is enabled for this queue. | [optional]
**calling_party_name** | Option<**String**> | The name to use for caller identification for outbound calls from this queue. | [optional]
**calling_party_number** | Option<**String**> | The phone number to use for caller identification for outbound calls from this queue. | [optional]
**default_scripts** | Option<[**::std::collections::HashMap<String, crate::models::Script>**](Script.md)> | The default script Ids for the communication types. | [optional]
**outbound_messaging_addresses** | Option<[**crate::models::QueueMessagingAddresses**](QueueMessagingAddresses.md)> |  | [optional]
**outbound_email_address** | Option<[**crate::models::QueueEmailAddress**](QueueEmailAddress.md)> |  | [optional]
**joined** | Option<**bool**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


