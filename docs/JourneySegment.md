# JourneySegment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**is_active** | Option<**bool**> | Whether or not the segment is active. | [optional]
**display_name** | **String** | The display name of the segment. | 
**version** | Option<**i32**> | The version of the segment. | [optional]
**description** | Option<**String**> | A description of the segment. | [optional]
**color** | Option<**String**> | The hexadecimal color value of the segment. | [optional]
**scope** | Option<**String**> | The target entity that a segment applies to. | [optional]
**should_display_to_agent** | Option<**bool**> | Whether or not the segment should be displayed to agent/supervisor users. | [optional]
**context** | Option<[**crate::models::Context**](Context.md)> |  | [optional]
**journey** | Option<[**crate::models::Journey**](Journey.md)> |  | [optional]
**external_segment** | Option<[**crate::models::ExternalSegment**](ExternalSegment.md)> |  | [optional]
**assignment_expiration_days** | Option<**i32**> | Time, in days, from when the segment is assigned until it is automatically unassigned. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**created_date** | Option<**String**> | Timestamp indicating when the segment was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_date** | Option<**String**> | Timestamp indicating when the the segment was last updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


