# CampaignRuleActionEntities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**campaigns** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | The list of campaigns for a CampaignRule to monitor. Required if the CampaignRule has any conditions that run on a campaign. | [optional]
**sequences** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | The list of sequences for a CampaignRule to monitor. Required if the CampaignRule has any conditions that run on a sequence. | [optional]
**use_triggering_entity** | Option<**bool**> | If true, the CampaignRuleAction will apply to the same entity that triggered the CampaignRuleCondition. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


