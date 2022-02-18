# PostTextRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**bot_id** | **String** | ID of the bot to send the text to. | 
**bot_alias** | Option<**String**> | Alias/Version of the bot | [optional]
**integration_id** | **String** | the integration service id for the bot's credentials | 
**bot_session_id** | **String** | GUID for this bot's session | 
**post_text_message** | [**crate::models::PostTextMessage**](PostTextMessage.md) |  | 
**language_code** | Option<**String**> | The launguage code the bot will run under | [optional]
**bot_session_timeout_minutes** | Option<**i32**> | Override timeout for the bot session. This should be greater than 10 minutes. | [optional]
**bot_channels** | Option<**Vec<String>**> | The channels this bot is utilizing | [optional]
**bot_correlation_id** | Option<**String**> | Id for tracking the activity - this will be returned in the response | [optional]
**messaging_platform_type** | Option<**String**> | If the channels list contains a 'Messaging' item and the messaging platform is known, include it here to get accurate analytics | [optional]
**amazon_lex_request** | Option<[**crate::models::AmazonLexRequest**](AmazonLexRequest.md)> |  | [optional]
**google_dialogflow** | Option<[**crate::models::GoogleDialogflowCustomSettings**](GoogleDialogflowCustomSettings.md)> |  | [optional]
**genesys_bot_connector** | Option<[**crate::models::GenesysBotConnector**](GenesysBotConnector.md)> |  | [optional]
**nuance_mix_dlg** | Option<[**crate::models::NuanceMixDlgSettings**](NuanceMixDlgSettings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


