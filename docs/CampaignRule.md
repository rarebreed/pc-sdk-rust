# CampaignRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the CampaignRule. | 
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**campaign_rule_entities** | [**crate::models::CampaignRuleEntities**](CampaignRuleEntities.md) |  | 
**campaign_rule_conditions** | [**Vec<crate::models::CampaignRuleCondition>**](CampaignRuleCondition.md) | The list of conditions that are evaluated on the entities. | 
**campaign_rule_actions** | [**Vec<crate::models::CampaignRuleAction>**](CampaignRuleAction.md) | The list of actions that are executed if the conditions are satisfied. | 
**match_any_conditions** | Option<**bool**> |  | [optional]
**enabled** | Option<**bool**> | Whether or not this CampaignRule is currently enabled. Required on updates. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


