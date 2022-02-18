# Leaderboard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**metric** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**date_start_workday** | Option<[**String**](string.md)> | Start workday used as the date range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**date_end_workday** | Option<[**String**](string.md)> | End workday used as the date range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**leaders** | Option<[**Vec<crate::models::LeaderboardItem>**](LeaderboardItem.md)> | The list of leaders generated. | [optional][readonly]
**user_rank** | Option<[**crate::models::LeaderboardItem**](LeaderboardItem.md)> |  | [optional]
**performance_profile** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


