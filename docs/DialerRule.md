# DialerRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The identifier of the rule. | [optional][readonly]
**name** | **String** | The name of the rule. | 
**order** | Option<**i32**> | The ranked order of the rule. Rules are processed from lowest number to highest. | [optional]
**category** | **String** | The category of the rule. | 
**conditions** | [**Vec<crate::models::Condition>**](Condition.md) | A list of Conditions. All of the Conditions must evaluate to true to trigger the actions. | 
**actions** | Option<[**Vec<crate::models::DialerAction>**](DialerAction.md)> | The list of actions to be taken if the conditions are true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


