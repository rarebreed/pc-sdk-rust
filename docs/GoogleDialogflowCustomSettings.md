# GoogleDialogflowCustomSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment** | Option<**String**> | If set this environment will be used to initiate the dialogflow bot, otherwise the default configuration will be used.  See https://cloud.google.com/dialogflow/docs/agents-versions | [optional]
**event_name** | Option<**String**> | If set this eventName will be used to initiate the dialogflow bot rather than language processing on the input text.  See https://cloud.google.com/dialogflow/es/docs/events-overview | [optional]
**webhook_query_parameters** | Option<**::std::collections::HashMap<String, String>**> | Parameters passed to the fulfillment webhook of the bot (if any). | [optional]
**event_input_parameters** | Option<**::std::collections::HashMap<String, String>**> | Parameters passed to the event input of the bot. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


