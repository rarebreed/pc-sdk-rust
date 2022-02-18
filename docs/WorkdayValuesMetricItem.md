# WorkdayValuesMetricItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**metric** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**metric_definition** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**average** | Option<**f64**> | The average value of the metric | [optional][readonly]
**unit_type** | Option<**String**> | The unit type of the metric value | [optional][readonly]
**trend** | Option<[**Vec<crate::models::WorkdayValuesTrendItem>**](WorkdayValuesTrendItem.md)> | The metric value trend | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


