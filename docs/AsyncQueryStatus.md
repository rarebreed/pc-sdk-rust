# AsyncQueryStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | Option<**String**> | The current state of the asynchronous query | [optional]
**error_message** | Option<**String**> | The error associated with the current query, if the state is FAILED | [optional]
**expiration_date** | Option<**String**> | The time at which results for this query will expire. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**submission_date** | Option<**String**> | The time at which the query was submitted. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**completion_date** | Option<**String**> | The time at which the query completed. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


