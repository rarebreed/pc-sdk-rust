# BuForecastModification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | **String** | The type of the modification | 
**start_interval_index** | Option<**i32**> | The number of 15 minute intervals past referenceStartDate representing the first interval to which to apply this modification. Must be null if values is populated | [optional]
**end_interval_index** | Option<**i32**> | The number of 15 minute intervals past referenceStartDate representing the last interval to which to apply this modification.  Must be null if values is populated | [optional]
**metric** | **String** | The metric to which this modification applies | 
**legacy_metric** | Option<**String**> | The legacy metric to which this modification applies if applicable | [optional][readonly]
**value** | Option<**f64**> | The value of the modification.  Must be null if \"values\" is populated | [optional]
**values** | Option<[**Vec<crate::models::WfmForecastModificationIntervalOffsetValue>**](WfmForecastModificationIntervalOffsetValue.md)> | The list of values to update.  Only applicable for grid-type modifications. Must be null if \"value\" is populated | [optional]
**display_granularity** | **String** | The client side display granularity of the modification, expressed in the ISO-8601 duration format. Periods are represented as an ISO-8601 string. For example: P1D or P1DT12H | 
**granularity** | Option<**String**> | The actual granularity of the modification as stored behind the scenes, expressed in the ISO-8601 duration format. Periods are represented as an ISO-8601 string. For example: P1D or P1DT12H | [optional]
**enabled** | **bool** | Whether the modification is enabled for the forecast | 
**planning_group_ids** | Option<**Vec<String>**> | The IDs of the planning groups to which this forecast modification applies.  Leave empty to apply to all | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


