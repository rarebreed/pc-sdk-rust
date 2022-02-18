# ReportingTurn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_input** | Option<**String**> | The chosen user input associated with this reporting turn. | [optional]
**bot_prompts** | Option<**Vec<String>**> | The bot prompts associated with this reporting turn. | [optional]
**session_id** | Option<**String**> | The bot session ID that this reporting turn is grouped under. | [optional]
**ask_action** | Option<[**crate::models::ReportingTurnAction**](ReportingTurnAction.md)> |  | [optional]
**intent** | Option<[**crate::models::ReportingTurnIntent**](ReportingTurnIntent.md)> |  | [optional]
**knowledge** | Option<[**crate::models::ReportingTurnKnowledge**](ReportingTurnKnowledge.md)> |  | [optional]
**date_created** | Option<**String**> | Timestamp indicating when the original turn was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**ask_action_result** | Option<**String**> | Result of the bot flow 'ask' action. | [optional]
**conversation** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


