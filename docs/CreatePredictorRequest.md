# CreatePredictorRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_ids** | **Vec<String>** | The queue IDs associated with the predictor. | 
**kpi** | **String** | The KPI that the predictor attempts to maximize/minimize. | 
**routing_timeout_seconds** | Option<**i32**> | Number of seconds allocated to predictive routing before attempting a different routing method. This is a value between 12 and 900 seconds. | [optional]
**schedule** | Option<[**crate::models::PredictorSchedule**](PredictorSchedule.md)> |  | [optional]
**workload_balancing_config** | Option<[**crate::models::PredictorWorkloadBalancing**](PredictorWorkloadBalancing.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


