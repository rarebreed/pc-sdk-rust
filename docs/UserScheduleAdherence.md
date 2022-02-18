# UserScheduleAdherence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**business_unit** | Option<[**crate::models::BusinessUnit**](BusinessUnit.md)> |  | [optional]
**management_unit** | Option<[**crate::models::ManagementUnit**](ManagementUnit.md)> |  | [optional]
**team** | Option<[**crate::models::Team**](Team.md)> |  | [optional]
**scheduled_activity_category** | Option<**String**> | Activity for which the user is scheduled | [optional][readonly]
**system_presence** | Option<**String**> | Actual underlying system presence value | [optional][readonly]
**organization_secondary_presence_id** | Option<**String**> | Organization Secondary Presence Id. | [optional][readonly]
**routing_status** | Option<**String**> | Actual underlying routing status, used to determine whether a user is actually in adherence when OnQueue | [optional][readonly]
**actual_activity_category** | Option<**String**> | Activity in which the user is actually engaged | [optional][readonly]
**is_out_of_office** | Option<**bool**> | Whether the user is marked OutOfOffice | [optional][readonly]
**adherence_state** | Option<**String**> | The user's current adherence state | [optional][readonly]
**impact** | Option<**String**> | The impact of the user's current adherenceState | [optional][readonly]
**time_of_adherence_change** | Option<**String**> | Time when the user entered the current adherenceState in ISO-8601 format | [optional][readonly]
**presence_update_time** | Option<**String**> | Time when presence was last updated.  Used to calculate time in current status. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**active_queues** | Option<[**Vec<crate::models::QueueReference>**](QueueReference.md)> | The list of queues to which this user is joined | [optional][readonly]
**active_queues_modified_time** | Option<**String**> | Time when the list of active queues for this user was last updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**removed_from_management_unit** | Option<**bool**> | For notification purposes. Used to indicate that a user was removed from the management unit | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


