# LexBotAlias

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**bot** | Option<[**crate::models::LexBot**](LexBot.md)> |  | [optional]
**bot_version** | Option<**String**> | The version of the Lex bot this alias points at | [optional]
**status** | **String** | The status of the Lex bot alias | 
**failure_reason** | Option<**String**> | If the status is FAILED, Amazon Lex explains why it failed to build the bot | [optional]
**language** | Option<**String**> | The target language of the Lex bot | [optional]
**intents** | Option<[**Vec<crate::models::LexIntent>**](LexIntent.md)> | An array of Intents associated with this bot alias | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


