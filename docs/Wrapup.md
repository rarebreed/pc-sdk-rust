# Wrapup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**String**> | The user configured wrap up code id. | [optional]
**name** | Option<**String**> | The user configured wrap up code name. | [optional]
**notes** | Option<**String**> | Text entered by the agent to describe the call or disposition. | [optional]
**tags** | Option<**Vec<String>**> | List of tags selected by the agent to describe the call or disposition. | [optional]
**duration_seconds** | Option<**i32**> | The length of time in seconds that the agent spent doing after call work. | [optional]
**end_time** | Option<**String**> | The timestamp when the wrapup was finished. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**provisional** | Option<**bool**> | Indicates if this is a pending save and should not require a code to be specified.  This allows someone to save some temporary wrapup that will be used later. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


