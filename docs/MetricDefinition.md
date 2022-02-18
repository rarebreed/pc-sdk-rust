# MetricDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**unit_type** | Option<**String**> | The type of associated metric unit | [optional]
**short_name** | Option<**String**> | An alternate name for this metric definition, often abbreviation | [optional]
**dividend_metrics** | Option<**Vec<String>**> | Metric names used as dividend | [optional]
**divisor_metrics** | Option<**Vec<String>**> | Metric names used as divisor | [optional]
**default_objective** | Option<[**crate::models::DefaultObjective**](DefaultObjective.md)> |  | [optional]
**lock_template_id** | Option<**String**> | An optional field to specify if this metric definition is locked to certain template. e.g. punctuality | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


