# CreateWebChatRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_id** | **String** | The ID of the queue to use for routing the chat conversation. | 
**provider** | **String** | The name of the provider that is sourcing the web chat. | 
**skill_ids** | Option<**Vec<String>**> | The list of skill ID's to use for routing. | [optional]
**language_id** | Option<**String**> | The ID of the langauge to use for routing. | [optional]
**priority** | Option<**i64**> | The priority to assign to the conversation for routing. | [optional]
**attributes** | Option<**::std::collections::HashMap<String, String>**> | The list of attributes to associate with the customer participant. | [optional]
**customer_name** | Option<**String**> | The name of the customer participating in the web chat. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


