# CoachingNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> | The name of the appointment for this notification. | [optional][readonly]
**marked_as_read** | Option<**bool**> | Indicates if notification is read or unread | [optional]
**action_type** | Option<**String**> | Action causing the notification. | [optional][readonly]
**relationship** | Option<**String**> | The relationship of this user to this notification's appointment | [optional][readonly]
**date_start** | Option<**String**> | The start time of the appointment relating to this notification. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**length_in_minutes** | Option<**i32**> | The duration of the appointment on this notification | [optional][readonly]
**status** | Option<**String**> | The status of the appointment for this notification | [optional][readonly]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**appointment** | Option<[**crate::models::CoachingAppointmentResponse**](CoachingAppointmentResponse.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


