# ImportStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | **String** | current status of the import | [readonly]
**total_records** | **i64** | total number of records to be imported | [readonly]
**completed_records** | **i64** | number of records finished importing | [readonly]
**percent_complete** | **i32** | percentage of records finished importing | [readonly]
**failure_reason** | Option<**String**> | if the import has failed, the reason for the failure | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


