# TextBotFlowTurnResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the bot flow turn. If additional turns are needed, supply this ID as the previous turn in your next turn request. | 
**previous_turn** | Option<[**crate::models::TextBotTurnReference**](TextBotTurnReference.md)> |  | [optional]
**prompts** | Option<[**crate::models::TextBotOutputPrompts**](TextBotOutputPrompts.md)> |  | [optional]
**next_action_type** | **String** | Indicates the suggested next action. If appropriate, the matching output event object includes additional information. | 
**next_action_disconnect** | Option<[**crate::models::TextBotDisconnectAction**](TextBotDisconnectAction.md)> |  | [optional]
**next_action_wait_for_input** | Option<[**crate::models::TextBotWaitForInputAction**](TextBotWaitForInputAction.md)> |  | [optional]
**next_action_exit** | Option<[**crate::models::TextBotExitAction**](TextBotExitAction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


