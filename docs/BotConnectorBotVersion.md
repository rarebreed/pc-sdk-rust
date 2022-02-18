# BotConnectorBotVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **String** | The name of the version. This can be up to 100 characters long and must be comprised of displayable characters without leading or trailing whitespace | 
**supported_languages** | **Vec<String>** | The supported languages for this bot. EG 'en-us' or 'es', etc; These language codes are W3C language identification tags (ISO 639-1 for the language name and ISO 3166 for the country code) | 
**intents** | [**Vec<crate::models::BotIntent>**](BotIntent.md) | A list of potential intents this bot will return, limit of 50 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


