# LearningAssignmentAggregateParam

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**interval** | **String** | Specifies the range of due dates to be used for filtering. Milliseconds will be truncated. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss | 
**metrics** | Option<**Vec<String>**> | The list of metrics to be returned. If omitted, all metrics are returned. | [optional]
**group_by** | Option<**Vec<String>**> | Specifies if the aggregated data is combined into a single set of metrics (groupBy is empty or not specified), or contains an element per attendeeId (groupBy is \"attendeeId\") | [optional]
**filter** | [**crate::models::LearningAssignmentAggregateQueryRequestFilter**](LearningAssignmentAggregateQueryRequestFilter.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


