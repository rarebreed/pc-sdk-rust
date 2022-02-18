# ComparisonPeriod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**kpi** | Option<**String**> | Key Performance Indicator optimised during the comparison period. | [optional][readonly]
**date_started** | Option<**String**> | Start date of the comparison period. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_ended** | Option<**String**> | End date of the comparison period. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**kpi_total_on** | Option<**i64**> | Absolute metric (in which the KPI is based) total for the interactions handled by predictive routing (GPR was on) | [optional][readonly]
**kpi_total_off** | Option<**i64**> | Absolute metric (in which the KPI is based) total for the interactions not routed by predictive routing (GPR was off) | [optional][readonly]
**interaction_count_on** | Option<**i64**> | Total interactions handled by predictive routing (GPR was on) | [optional][readonly]
**interaction_count_off** | Option<**i64**> | Total interactions not routed by predictive routing (GPR was off) | [optional][readonly]
**kpi_results** | Option<[**Vec<crate::models::KpiResult>**](KpiResult.md)> | KPI results for each metric | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


