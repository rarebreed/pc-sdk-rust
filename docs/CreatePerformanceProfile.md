# CreatePerformanceProfile

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | A name for this performance profile | 
**division** | [**crate::models::WritableDivision**](WritableDivision.md) |  | 
**description** | **String** | A description about this performance profile | 
**metric_orders** | Option<**Vec<String>**> | Order of the associated metrics. The list should contain valid ids for metrics | [optional][readonly]
**date_created** | Option<**String**> | Creation date for this performance profile. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**reporting_intervals** | [**Vec<crate::models::ReportingInterval>**](ReportingInterval.md) | The reporting interval periods for this performance profile | 
**active** | **bool** | The flag for active profiles | 
**member_count** | Option<**i32**> | The number of members in this performance profile | [optional][readonly]
**max_leaderboard_rank_size** | **i32** | The maximum rank size for the leaderboard. This counts the number of ranks can be retrieved in a leaderboard queries | 
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


