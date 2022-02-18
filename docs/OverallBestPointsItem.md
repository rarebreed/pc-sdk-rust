# OverallBestPointsItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**granularity_type** | Option<**String**> | Best points aggregation interval granularity | [optional][readonly]
**users** | Option<[**Vec<crate::models::UserReference>**](UserReference.md)> | List of associated users with the equal points. | [optional][readonly]
**count** | Option<**i32**> | The count of the user IDs in the list | [optional][readonly]
**points** | Option<**i32**> | Gamification points | [optional][readonly]
**date_start_workday** | Option<[**String**](string.md)> | Start workday of the best points aggregation interval. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**date_end_workday** | Option<[**String**](string.md)> | End workday of the best points aggregation interval. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


