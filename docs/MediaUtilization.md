# MediaUtilization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**maximum_capacity** | Option<**i32**> | Defines the maximum number of conversations of this type that an agent can handle at one time. | [optional]
**interruptable_media_types** | Option<**Vec<String>**> | Defines the list of other media types that can interrupt a conversation of this media type.  Values include call, chat, email, callback, and message. | [optional]
**include_non_acd** | Option<**bool**> | If true, then track non-ACD conversations against utilization | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


