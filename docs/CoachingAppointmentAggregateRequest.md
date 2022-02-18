# CoachingAppointmentAggregateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | **String** | Interval to aggregate across. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**metrics** | Option<**Vec<String>**> | A list of metrics to aggregate.  If omitted, all metrics are returned. | [optional]
**group_by** | Option<**Vec<String>**> | An optional list of items by which to group the result data. | [optional]
**filter** | [**crate::models::QueryRequestFilter**](QueryRequestFilter.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


