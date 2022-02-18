# PatchActionMap

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**version** | Option<**i32**> | The version of the action map. | [optional]
**is_active** | Option<**bool**> | Whether the action map is active. | [optional]
**display_name** | **String** | Display name of the action map. | 
**trigger_with_segments** | **Vec<String>** | Trigger action map if any segment in the list is assigned to a given customer. | 
**trigger_with_event_conditions** | Option<[**Vec<crate::models::EventCondition>**](EventCondition.md)> | List of event conditions that must be satisfied to trigger the action map. | [optional]
**trigger_with_outcome_probability_conditions** | Option<[**Vec<crate::models::OutcomeProbabilityCondition>**](OutcomeProbabilityCondition.md)> | Probability conditions for outcomes that must be satisfied to trigger the action map. | [optional]
**page_url_conditions** | [**Vec<crate::models::UrlCondition>**](UrlCondition.md) | URL conditions that a page must match for web actions to be displayable. | 
**activation** | Option<[**crate::models::Activation**](Activation.md)> |  | [optional]
**weight** | Option<**i32**> | Weight of the action map with higher number denoting higher weight. | [optional]
**action** | Option<[**crate::models::PatchAction**](PatchAction.md)> |  | [optional]
**action_map_schedule_groups** | Option<[**crate::models::PatchActionMapScheduleGroups**](PatchActionMapScheduleGroups.md)> |  | [optional]
**ignore_frequency_cap** | Option<**bool**> | Override organization-level frequency cap and always offer web engagements from this action map. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**created_date** | Option<**String**> | Timestamp indicating when the action map was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_date** | Option<**String**> | Timestamp indicating when the action map was last updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**start_date** | Option<**String**> | Timestamp at which the action map is scheduled to start firing. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_date** | Option<**String**> | Timestamp at which the action map is scheduled to stop firing. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


