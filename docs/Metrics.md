# Metrics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**order** | Option<**i32**> | The order of metric within a performance profile | [optional]
**metric_definition_name** | Option<**String**> | The name of associated metric definition | [optional]
**metric_definition_id** | Option<**String**> | The id of associated metric definition | [optional]
**external_metric_definition_id** | Option<**String**> | The id of associated external metric definition | [optional]
**unit_type** | Option<**String**> | Corresponding unit type for this metric | [optional]
**enabled** | Option<**bool**> | A flag for whether this metric is enabled for a performance profile | [optional]
**template_name** | Option<**String**> | The name of associated objective template | [optional]
**max_points** | Option<**i32**> | Achievable maximum points for this metric | [optional]
**performance_profile_id** | Option<**String**> | Performance profile id of this metric | [optional]
**linked_metric** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**date_created** | Option<**String**> | The created date of this metric. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_unlinked** | Option<[**String**](string.md)> | The unlinked workday for this metric if this metric was ever unlinked. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional][readonly]
**source_performance_profile** | Option<[**crate::models::PerformanceProfile**](PerformanceProfile.md)> |  | [optional]
**unit_definition** | Option<**String**> | Unit definition of linked external metric | [optional][readonly]
**precision** | Option<**i32**> | Precision of linked external metric | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


