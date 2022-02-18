# PostTextResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_state** | **String** | The state of the bot after completion of the request | 
**reply_messages** | Option<[**Vec<crate::models::PostTextMessage>**](PostTextMessage.md)> | The list of messages to respond with, if any | [optional]
**intent_name** | Option<**String**> | The name of the intent the bot is either processing or has processed, this will be blank if no intent could be detected. | [optional]
**slots** | Option<**::std::collections::HashMap<String, String>**> | Data parameters detected and filled by the bot. | [optional]
**bot_correlation_id** | Option<**String**> | The optional ID specified in the request | [optional]
**amazon_lex** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Raw data response from AWS (if called) | [optional]
**google_dialog_flow** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Raw data response from Google Dialogflow (if called) | [optional]
**genesys_dialog_engine** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Raw data response from Genesys' Dialogengine (if called) | [optional]
**genesys_bot_connector** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Raw data response from Genesys' BotConnector (if called) | [optional]
**nuance_mix_dlg** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Raw data response from Nuance Mix Dlg (if called) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


