# PerformanceProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | A name for this performance profile | 
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**description** | **String** | A description about this performance profile | 
**metric_orders** | **Vec<String>** | Order of the associated metrics. The list should contain valid ids for metrics | 
**date_created** | Option<**String**> | Creation date for this performance profile. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**reporting_intervals** | Option<[**Vec<crate::models::ReportingInterval>**](ReportingInterval.md)> | The reporting interval periods for this performance profile | [optional]
**active** | Option<**bool**> | The flag for active profiles | [optional][readonly]
**member_count** | Option<**i32**> | The number of members in this performance profile | [optional][readonly]
**max_leaderboard_rank_size** | Option<**i32**> | The maximum rank size for the leaderboard. This counts the number of ranks can be retrieved in a leaderboard queries | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


