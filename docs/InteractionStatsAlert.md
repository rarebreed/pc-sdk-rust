# InteractionStatsAlert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | Name of the rule that generated the alert | [readonly]
**dimension** | **String** | The dimension of concern. | [readonly]
**dimension_value** | **String** | The value of the dimension. | [readonly]
**metric** | **String** | The metric to be assessed. | [readonly]
**media_type** | **String** | The media type. | [readonly]
**numeric_range** | **String** | The comparison descriptor used against the metric's value. | [readonly]
**statistic** | **String** | The statistic of concern for the metric. | [readonly]
**value** | **f64** | The threshold value. | [readonly]
**rule_id** | **String** | The id of the rule. | [readonly]
**unread** | **bool** | Indicates if the alert has been read. | 
**start_date** | **String** | The date/time the alert was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [readonly]
**end_date** | Option<**String**> | The date/time the owning rule exiting in alarm status. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**notification_users** | [**Vec<crate::models::User>**](User.md) | The ids of users who were notified of alarm state change. | [readonly]
**alert_types** | **Vec<String>** | A collection of notification methods. | [readonly]
**rule_uri** | Option<**String**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


