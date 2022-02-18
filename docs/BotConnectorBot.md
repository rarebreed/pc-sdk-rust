# BotConnectorBot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The Botconnector Bot Id - this is configurable by the user when put | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> | An optional description of the bot.  This can be up to 256 characters long and must be comprised of displayable characters without leading or trailing whitespace | [optional]
**versions** | [**Vec<crate::models::BotConnectorBotVersion>**](BotConnectorBotVersion.md) | This bots versions, limit of 50 per bot | 
**bot_composite_tag** | Option<**String**> | A system-generated string that contains metadata about this bot. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


