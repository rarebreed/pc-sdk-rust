# InteractionStatsRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | Name of the rule | 
**dimension** | **String** | The dimension of concern. | 
**dimension_value** | **String** | The value of the dimension. | 
**metric** | **String** | The metric to be assessed. | 
**media_type** | **String** | The media type. | 
**numeric_range** | **String** | The comparison descriptor used against the metric's value. | 
**statistic** | **String** | The statistic of concern for the metric. | 
**value** | **f64** | The threshold value. | 
**enabled** | **bool** | Indicates if the rule is enabled. | 
**in_alarm** | Option<**bool**> | Indicates if the rule is in alarm state. | [optional][readonly]
**notification_users** | [**Vec<crate::models::User>**](User.md) | The ids of users who will be notified of alarm state change. | 
**alert_types** | **Vec<String>** | A collection of notification methods. | 
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


