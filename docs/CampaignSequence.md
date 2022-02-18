# CampaignSequence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**campaigns** | [**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md) | The ordered list of Campaigns that this CampaignSequence will run. | 
**current_campaign** | **i32** | A zero-based index indicating which Campaign this CampaignSequence is currently on. | [readonly]
**status** | **String** | The current status of the CampaignSequence. A CampaignSequence can be turned 'on' or 'off'. | 
**stop_message** | Option<**String**> | A message indicating if and why a CampaignSequence has stopped unexpectedly. | [optional][readonly]
**repeat** | Option<**bool**> | Indicates if a sequence should repeat from the beginning after the last campaign completes. Default is false. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


