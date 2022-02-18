# InboundMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**queue_id** | Option<**String**> | The ID of the queue to use for routing the email conversation. This field is mutually exclusive with flowId | [optional]
**flow_id** | Option<**String**> | The ID of the flow to use for routing email conversation. This field is mutually exclusive with queueId | [optional]
**provider** | **String** | The name of the provider that is sourcing the email such as Oracle, Salesforce, etc. | 
**skill_ids** | Option<**Vec<String>**> | The list of skill ID's to use for routing. | [optional]
**language_id** | Option<**String**> | The ID of the language to use for routing. | [optional]
**priority** | Option<**i32**> | The priority to assign to the conversation for routing. | [optional]
**attributes** | Option<**::std::collections::HashMap<String, String>**> | The list of attributes to associate with the customer participant. | [optional]
**to_address** | Option<**String**> | The email address of the recipient of the email. | [optional]
**to_name** | Option<**String**> | The name of the recipient of the email. | [optional]
**from_address** | Option<**String**> | The email address of the sender of the email. | [optional]
**from_name** | Option<**String**> | The name of the sender of the email. | [optional]
**subject** | Option<**String**> | The subject of the email | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


