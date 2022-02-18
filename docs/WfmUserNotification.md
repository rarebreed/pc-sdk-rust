# WfmUserNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The immutable globally unique identifier for the object. | 
**mutable_group_id** | **String** | The group ID of the notification (mutable, may change  on update) | 
**timestamp** | Option<**String**> | The timestamp for this notification. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**_type** | Option<**String**> | The type of this notification | [optional][readonly]
**shift_trade** | Option<[**crate::models::ShiftTradeNotification**](ShiftTradeNotification.md)> |  | [optional]
**time_off_request** | Option<[**crate::models::TimeOffRequestNotification**](TimeOffRequestNotification.md)> |  | [optional]
**marked_as_read** | **bool** | Whether this notification has been marked \"read\" | 
**agent_notification** | Option<**bool**> | Whether this notification is for an agent | [optional][readonly]
**other_notification_ids_in_group** | Option<**Vec<String>**> | Other notification IDs in group.  This field is only populated in real-time notifications | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


