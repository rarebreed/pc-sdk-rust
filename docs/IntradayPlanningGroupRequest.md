# IntradayPlanningGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**business_unit_date** | [**String**](string.md) | Requested date in yyyy-MM-dd format. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | 
**categories** | **Vec<String>** | The metric categories | 
**planning_group_ids** | Option<**Vec<String>**> | The IDs of the planning groups for which to fetch data.  Omitting or passing an empty list will return all available planning groups | [optional]
**interval_length_minutes** | Option<**i32**> | The period/interval in minutes for which to aggregate the data. Required, defaults to 15 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


