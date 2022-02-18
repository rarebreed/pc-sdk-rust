# WorkdayPointsTrend

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_start_workday** | Option<[**String**](string.md)> | The start workday for the query range for the gamification points trend. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**date_end_workday** | Option<[**String**](string.md)> | The end workday for the query range for the gamification points trend. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**day_of_week** | Option<**String**> | Aggregated for same day comparison | [optional][readonly]
**average_points** | Option<**f64**> | The total average points | [optional][readonly]
**trend** | Option<[**Vec<crate::models::WorkdayPointsTrendItem>**](WorkdayPointsTrendItem.md)> | Daily points trends | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


