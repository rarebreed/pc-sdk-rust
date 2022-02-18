# QueueUtilizationDiagnostic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**users_in_queue** | Option<**i32**> | The number of users joined to the queue | [optional][readonly]
**active_users_in_queue** | Option<**i32**> | The number of users active on the queue | [optional][readonly]
**users_on_queue** | Option<**i32**> | The number of users with a status of on-queue | [optional][readonly]
**users_not_utilized** | Option<**i32**> | The number of users in the queue currently not engaged | [optional][readonly]
**users_on_queue_with_station** | Option<**i32**> | The number of users in the queue with a station | [optional][readonly]
**users_on_a_campaign_call** | Option<**i32**> | The number of users currently engaged in a campaign call | [optional][readonly]
**users_on_different_edge_group** | Option<**i32**> | The number of users whose station is homed to an edge different from the campaign | [optional][readonly]
**users_on_a_non_campaign_call** | Option<**i32**> | The number of users currently engaged in a communication that is not part of the campaign | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


