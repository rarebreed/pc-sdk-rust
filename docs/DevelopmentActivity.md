# DevelopmentActivity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**date_completed** | Option<**String**> | Date that activity was completed. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**created_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_created** | Option<**String**> | Date activity was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**percentage_score** | Option<**f32**> | The user's percentage score for this activity | [optional][readonly]
**is_passed** | Option<**bool**> | True if the activity was passed | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**name** | Option<**String**> | The name of the activity | [optional]
**_type** | Option<**String**> | The type of activity | [optional]
**status** | Option<**String**> | The status of the activity | [optional]
**date_due** | Option<**String**> | Due date for completion of the activity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**facilitator** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**attendees** | Option<[**Vec<crate::models::UserReference>**](UserReference.md)> | List of users attending the activity | [optional]
**is_overdue** | Option<**bool**> | Indicates if the activity is overdue | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


