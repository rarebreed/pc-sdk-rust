# WfmHistoricalAdherenceQueryForUsers

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | **String** | Beginning of the date range to query in ISO-8601 format | 
**end_date** | Option<**String**> | End of the date range to query in ISO-8601 format. If it is not set, end date will be set to current time | [optional]
**time_zone** | Option<**String**> | The time zone to use for returned results in olson format. If it is not set, the business unit time zone will be used to compute adherence | [optional]
**user_ids** | **Vec<String>** | The userIds to report on | 
**include_exceptions** | Option<**bool**> | Whether user exceptions should be returned as part of the results | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


