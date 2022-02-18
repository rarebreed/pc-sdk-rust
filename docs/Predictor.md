# Predictor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**queues** | [**Vec<crate::models::AddressableEntityRef>**](AddressableEntityRef.md) | The queue IDs associated with the predictor. | 
**kpi** | **String** | The KPI that the predictor attempts to maximize/minimize. | 
**routing_timeout_seconds** | Option<**i32**> | Number of seconds allocated to predictive routing before attempting a different routing method. This is a value between 12 and 900 seconds. | [optional]
**schedule** | Option<[**crate::models::PredictorSchedule**](PredictorSchedule.md)> |  | [optional]
**state** | Option<**String**> | The predictor state. | [optional][readonly]
**date_created** | Option<**String**> | DateTime indicating when the predictor was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | DateTime indicating when the predictor was last updated. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**workload_balancing_config** | Option<[**crate::models::PredictorWorkloadBalancing**](PredictorWorkloadBalancing.md)> |  | [optional]
**error_code** | Option<**String**> | Predictor error code - optional details on why the predictor went into error state. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


