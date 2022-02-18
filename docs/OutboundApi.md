# \OutboundApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_outbound_attemptlimit**](OutboundApi.md#delete_outbound_attemptlimit) | **DELETE** /api/v2/outbound/attemptlimits/{attemptLimitsId} | Delete attempt limits
[**delete_outbound_callabletimeset**](OutboundApi.md#delete_outbound_callabletimeset) | **DELETE** /api/v2/outbound/callabletimesets/{callableTimeSetId} | Delete callable time set
[**delete_outbound_callanalysisresponseset**](OutboundApi.md#delete_outbound_callanalysisresponseset) | **DELETE** /api/v2/outbound/callanalysisresponsesets/{callAnalysisSetId} | Delete a dialer call analysis response set.
[**delete_outbound_campaign**](OutboundApi.md#delete_outbound_campaign) | **DELETE** /api/v2/outbound/campaigns/{campaignId} | Delete a campaign.
[**delete_outbound_campaign_progress**](OutboundApi.md#delete_outbound_campaign_progress) | **DELETE** /api/v2/outbound/campaigns/{campaignId}/progress | Reset campaign progress and recycle the campaign
[**delete_outbound_campaignrule**](OutboundApi.md#delete_outbound_campaignrule) | **DELETE** /api/v2/outbound/campaignrules/{campaignRuleId} | Delete Campaign Rule
[**delete_outbound_contactlist**](OutboundApi.md#delete_outbound_contactlist) | **DELETE** /api/v2/outbound/contactlists/{contactListId} | Delete a contact list.
[**delete_outbound_contactlist_contact**](OutboundApi.md#delete_outbound_contactlist_contact) | **DELETE** /api/v2/outbound/contactlists/{contactListId}/contacts/{contactId} | Delete a contact.
[**delete_outbound_contactlist_contacts**](OutboundApi.md#delete_outbound_contactlist_contacts) | **DELETE** /api/v2/outbound/contactlists/{contactListId}/contacts | Delete contacts from a contact list.
[**delete_outbound_contactlistfilter**](OutboundApi.md#delete_outbound_contactlistfilter) | **DELETE** /api/v2/outbound/contactlistfilters/{contactListFilterId} | Delete Contact List Filter
[**delete_outbound_contactlists**](OutboundApi.md#delete_outbound_contactlists) | **DELETE** /api/v2/outbound/contactlists | Delete multiple contact lists.
[**delete_outbound_dnclist**](OutboundApi.md#delete_outbound_dnclist) | **DELETE** /api/v2/outbound/dnclists/{dncListId} | Delete dialer DNC list
[**delete_outbound_messagingcampaign**](OutboundApi.md#delete_outbound_messagingcampaign) | **DELETE** /api/v2/outbound/messagingcampaigns/{messagingCampaignId} | Delete an Outbound Messaging Campaign
[**delete_outbound_ruleset**](OutboundApi.md#delete_outbound_ruleset) | **DELETE** /api/v2/outbound/rulesets/{ruleSetId} | Delete a Rule Set.
[**delete_outbound_schedules_campaign**](OutboundApi.md#delete_outbound_schedules_campaign) | **DELETE** /api/v2/outbound/schedules/campaigns/{campaignId} | Delete a dialer campaign schedule.
[**delete_outbound_schedules_sequence**](OutboundApi.md#delete_outbound_schedules_sequence) | **DELETE** /api/v2/outbound/schedules/sequences/{sequenceId} | Delete a dialer sequence schedule.
[**delete_outbound_sequence**](OutboundApi.md#delete_outbound_sequence) | **DELETE** /api/v2/outbound/sequences/{sequenceId} | Delete a dialer campaign sequence.
[**get_outbound_attemptlimit**](OutboundApi.md#get_outbound_attemptlimit) | **GET** /api/v2/outbound/attemptlimits/{attemptLimitsId} | Get attempt limits
[**get_outbound_attemptlimits**](OutboundApi.md#get_outbound_attemptlimits) | **GET** /api/v2/outbound/attemptlimits | Query attempt limits list
[**get_outbound_callabletimeset**](OutboundApi.md#get_outbound_callabletimeset) | **GET** /api/v2/outbound/callabletimesets/{callableTimeSetId} | Get callable time set
[**get_outbound_callabletimesets**](OutboundApi.md#get_outbound_callabletimesets) | **GET** /api/v2/outbound/callabletimesets | Query callable time set list
[**get_outbound_callanalysisresponseset**](OutboundApi.md#get_outbound_callanalysisresponseset) | **GET** /api/v2/outbound/callanalysisresponsesets/{callAnalysisSetId} | Get a dialer call analysis response set.
[**get_outbound_callanalysisresponsesets**](OutboundApi.md#get_outbound_callanalysisresponsesets) | **GET** /api/v2/outbound/callanalysisresponsesets | Query a list of dialer call analysis response sets.
[**get_outbound_campaign**](OutboundApi.md#get_outbound_campaign) | **GET** /api/v2/outbound/campaigns/{campaignId} | Get dialer campaign.
[**get_outbound_campaign_agentownedmappingpreview_results**](OutboundApi.md#get_outbound_campaign_agentownedmappingpreview_results) | **GET** /api/v2/outbound/campaigns/{campaignId}/agentownedmappingpreview/results | Get a preview of how agents will be mapped to this campaign's contact list.
[**get_outbound_campaign_diagnostics**](OutboundApi.md#get_outbound_campaign_diagnostics) | **GET** /api/v2/outbound/campaigns/{campaignId}/diagnostics | Get campaign diagnostics
[**get_outbound_campaign_interactions**](OutboundApi.md#get_outbound_campaign_interactions) | **GET** /api/v2/outbound/campaigns/{campaignId}/interactions | Get dialer campaign interactions.
[**get_outbound_campaign_progress**](OutboundApi.md#get_outbound_campaign_progress) | **GET** /api/v2/outbound/campaigns/{campaignId}/progress | Get campaign progress
[**get_outbound_campaign_stats**](OutboundApi.md#get_outbound_campaign_stats) | **GET** /api/v2/outbound/campaigns/{campaignId}/stats | Get statistics about a Dialer Campaign
[**get_outbound_campaignrule**](OutboundApi.md#get_outbound_campaignrule) | **GET** /api/v2/outbound/campaignrules/{campaignRuleId} | Get Campaign Rule
[**get_outbound_campaignrules**](OutboundApi.md#get_outbound_campaignrules) | **GET** /api/v2/outbound/campaignrules | Query Campaign Rule list
[**get_outbound_campaigns**](OutboundApi.md#get_outbound_campaigns) | **GET** /api/v2/outbound/campaigns | Query a list of dialer campaigns.
[**get_outbound_campaigns_all**](OutboundApi.md#get_outbound_campaigns_all) | **GET** /api/v2/outbound/campaigns/all | Query across all types of campaigns by division
[**get_outbound_campaigns_all_divisionviews**](OutboundApi.md#get_outbound_campaigns_all_divisionviews) | **GET** /api/v2/outbound/campaigns/all/divisionviews | Query across all types of campaigns
[**get_outbound_campaigns_divisionview**](OutboundApi.md#get_outbound_campaigns_divisionview) | **GET** /api/v2/outbound/campaigns/divisionviews/{campaignId} | Get a basic Campaign information object
[**get_outbound_campaigns_divisionviews**](OutboundApi.md#get_outbound_campaigns_divisionviews) | **GET** /api/v2/outbound/campaigns/divisionviews | Query a list of basic Campaign information objects
[**get_outbound_contactlist**](OutboundApi.md#get_outbound_contactlist) | **GET** /api/v2/outbound/contactlists/{contactListId} | Get a dialer contact list.
[**get_outbound_contactlist_contact**](OutboundApi.md#get_outbound_contactlist_contact) | **GET** /api/v2/outbound/contactlists/{contactListId}/contacts/{contactId} | Get a contact.
[**get_outbound_contactlist_export**](OutboundApi.md#get_outbound_contactlist_export) | **GET** /api/v2/outbound/contactlists/{contactListId}/export | Get the URI of a contact list export.
[**get_outbound_contactlist_importstatus**](OutboundApi.md#get_outbound_contactlist_importstatus) | **GET** /api/v2/outbound/contactlists/{contactListId}/importstatus | Get dialer contactList import status.
[**get_outbound_contactlist_timezonemappingpreview**](OutboundApi.md#get_outbound_contactlist_timezonemappingpreview) | **GET** /api/v2/outbound/contactlists/{contactListId}/timezonemappingpreview | Preview the result of applying Automatic Time Zone Mapping to a contact list
[**get_outbound_contactlistfilter**](OutboundApi.md#get_outbound_contactlistfilter) | **GET** /api/v2/outbound/contactlistfilters/{contactListFilterId} | Get Contact list filter
[**get_outbound_contactlistfilters**](OutboundApi.md#get_outbound_contactlistfilters) | **GET** /api/v2/outbound/contactlistfilters | Query Contact list filters
[**get_outbound_contactlists**](OutboundApi.md#get_outbound_contactlists) | **GET** /api/v2/outbound/contactlists | Query a list of contact lists.
[**get_outbound_contactlists_divisionview**](OutboundApi.md#get_outbound_contactlists_divisionview) | **GET** /api/v2/outbound/contactlists/divisionviews/{contactListId} | Get a basic ContactList information object
[**get_outbound_contactlists_divisionviews**](OutboundApi.md#get_outbound_contactlists_divisionviews) | **GET** /api/v2/outbound/contactlists/divisionviews | Query a list of simplified contact list objects.
[**get_outbound_dnclist**](OutboundApi.md#get_outbound_dnclist) | **GET** /api/v2/outbound/dnclists/{dncListId} | Get dialer DNC list
[**get_outbound_dnclist_export**](OutboundApi.md#get_outbound_dnclist_export) | **GET** /api/v2/outbound/dnclists/{dncListId}/export | Get the URI of a DNC list export.
[**get_outbound_dnclist_importstatus**](OutboundApi.md#get_outbound_dnclist_importstatus) | **GET** /api/v2/outbound/dnclists/{dncListId}/importstatus | Get dialer dncList import status.
[**get_outbound_dnclists**](OutboundApi.md#get_outbound_dnclists) | **GET** /api/v2/outbound/dnclists | Query dialer DNC lists
[**get_outbound_dnclists_divisionview**](OutboundApi.md#get_outbound_dnclists_divisionview) | **GET** /api/v2/outbound/dnclists/divisionviews/{dncListId} | Get a basic DncList information object
[**get_outbound_dnclists_divisionviews**](OutboundApi.md#get_outbound_dnclists_divisionviews) | **GET** /api/v2/outbound/dnclists/divisionviews | Query a list of simplified dnc list objects.
[**get_outbound_event**](OutboundApi.md#get_outbound_event) | **GET** /api/v2/outbound/events/{eventId} | Get Dialer Event
[**get_outbound_events**](OutboundApi.md#get_outbound_events) | **GET** /api/v2/outbound/events | Query Event Logs
[**get_outbound_messagingcampaign**](OutboundApi.md#get_outbound_messagingcampaign) | **GET** /api/v2/outbound/messagingcampaigns/{messagingCampaignId} | Get an Outbound Messaging Campaign
[**get_outbound_messagingcampaign_progress**](OutboundApi.md#get_outbound_messagingcampaign_progress) | **GET** /api/v2/outbound/messagingcampaigns/{messagingCampaignId}/progress | Get messaging campaign's progress
[**get_outbound_messagingcampaigns**](OutboundApi.md#get_outbound_messagingcampaigns) | **GET** /api/v2/outbound/messagingcampaigns | Query a list of Messaging Campaigns
[**get_outbound_messagingcampaigns_divisionview**](OutboundApi.md#get_outbound_messagingcampaigns_divisionview) | **GET** /api/v2/outbound/messagingcampaigns/divisionviews/{messagingCampaignId} | Get a basic Messaging Campaign information object
[**get_outbound_messagingcampaigns_divisionviews**](OutboundApi.md#get_outbound_messagingcampaigns_divisionviews) | **GET** /api/v2/outbound/messagingcampaigns/divisionviews | Query a list of basic Messaging Campaign information objects
[**get_outbound_ruleset**](OutboundApi.md#get_outbound_ruleset) | **GET** /api/v2/outbound/rulesets/{ruleSetId} | Get a Rule Set by ID.
[**get_outbound_rulesets**](OutboundApi.md#get_outbound_rulesets) | **GET** /api/v2/outbound/rulesets | Query a list of Rule Sets.
[**get_outbound_schedules_campaign**](OutboundApi.md#get_outbound_schedules_campaign) | **GET** /api/v2/outbound/schedules/campaigns/{campaignId} | Get a dialer campaign schedule.
[**get_outbound_schedules_campaigns**](OutboundApi.md#get_outbound_schedules_campaigns) | **GET** /api/v2/outbound/schedules/campaigns | Query for a list of dialer campaign schedules.
[**get_outbound_schedules_sequence**](OutboundApi.md#get_outbound_schedules_sequence) | **GET** /api/v2/outbound/schedules/sequences/{sequenceId} | Get a dialer sequence schedule.
[**get_outbound_schedules_sequences**](OutboundApi.md#get_outbound_schedules_sequences) | **GET** /api/v2/outbound/schedules/sequences | Query for a list of dialer sequence schedules.
[**get_outbound_sequence**](OutboundApi.md#get_outbound_sequence) | **GET** /api/v2/outbound/sequences/{sequenceId} | Get a dialer campaign sequence.
[**get_outbound_sequences**](OutboundApi.md#get_outbound_sequences) | **GET** /api/v2/outbound/sequences | Query a list of dialer campaign sequences.
[**get_outbound_settings**](OutboundApi.md#get_outbound_settings) | **GET** /api/v2/outbound/settings | Get the outbound settings for this organization
[**get_outbound_wrapupcodemappings**](OutboundApi.md#get_outbound_wrapupcodemappings) | **GET** /api/v2/outbound/wrapupcodemappings | Get the Dialer wrap up code mapping.
[**patch_outbound_settings**](OutboundApi.md#patch_outbound_settings) | **PATCH** /api/v2/outbound/settings | Update the outbound settings for this organization
[**post_outbound_attemptlimits**](OutboundApi.md#post_outbound_attemptlimits) | **POST** /api/v2/outbound/attemptlimits | Create attempt limits
[**post_outbound_audits**](OutboundApi.md#post_outbound_audits) | **POST** /api/v2/outbound/audits | Retrieves audits for dialer.
[**post_outbound_callabletimesets**](OutboundApi.md#post_outbound_callabletimesets) | **POST** /api/v2/outbound/callabletimesets | Create callable time set
[**post_outbound_callanalysisresponsesets**](OutboundApi.md#post_outbound_callanalysisresponsesets) | **POST** /api/v2/outbound/callanalysisresponsesets | Create a dialer call analysis response set.
[**post_outbound_campaign_agentownedmappingpreview**](OutboundApi.md#post_outbound_campaign_agentownedmappingpreview) | **POST** /api/v2/outbound/campaigns/{campaignId}/agentownedmappingpreview | Initiate request for a preview of how agents will be mapped to this campaign's contact list.
[**post_outbound_campaign_callback_schedule**](OutboundApi.md#post_outbound_campaign_callback_schedule) | **POST** /api/v2/outbound/campaigns/{campaignId}/callback/schedule | Schedule a Callback for a Dialer Campaign (Deprecated)
[**post_outbound_campaignrules**](OutboundApi.md#post_outbound_campaignrules) | **POST** /api/v2/outbound/campaignrules | Create Campaign Rule
[**post_outbound_campaigns**](OutboundApi.md#post_outbound_campaigns) | **POST** /api/v2/outbound/campaigns | Create a campaign.
[**post_outbound_campaigns_progress**](OutboundApi.md#post_outbound_campaigns_progress) | **POST** /api/v2/outbound/campaigns/progress | Get progress for a list of campaigns
[**post_outbound_contactlist_clear**](OutboundApi.md#post_outbound_contactlist_clear) | **POST** /api/v2/outbound/contactlists/{contactListId}/clear | Deletes all contacts out of a list. All outstanding recalls or rule-scheduled callbacks for non-preview campaigns configured with the contactlist will be cancelled.
[**post_outbound_contactlist_contacts**](OutboundApi.md#post_outbound_contactlist_contacts) | **POST** /api/v2/outbound/contactlists/{contactListId}/contacts | Add contacts to a contact list.
[**post_outbound_contactlist_contacts_bulk**](OutboundApi.md#post_outbound_contactlist_contacts_bulk) | **POST** /api/v2/outbound/contactlists/{contactListId}/contacts/bulk | Get contacts from a contact list.
[**post_outbound_contactlist_export**](OutboundApi.md#post_outbound_contactlist_export) | **POST** /api/v2/outbound/contactlists/{contactListId}/export | Initiate the export of a contact list.
[**post_outbound_contactlistfilters**](OutboundApi.md#post_outbound_contactlistfilters) | **POST** /api/v2/outbound/contactlistfilters | Create Contact List Filter
[**post_outbound_contactlistfilters_preview**](OutboundApi.md#post_outbound_contactlistfilters_preview) | **POST** /api/v2/outbound/contactlistfilters/preview | Get a preview of the output of a contact list filter
[**post_outbound_contactlists**](OutboundApi.md#post_outbound_contactlists) | **POST** /api/v2/outbound/contactlists | Create a contact List.
[**post_outbound_conversation_dnc**](OutboundApi.md#post_outbound_conversation_dnc) | **POST** /api/v2/outbound/conversations/{conversationId}/dnc | Add phone numbers to a Dialer DNC list.
[**post_outbound_dnclist_export**](OutboundApi.md#post_outbound_dnclist_export) | **POST** /api/v2/outbound/dnclists/{dncListId}/export | Initiate the export of a dnc list.
[**post_outbound_dnclist_phonenumbers**](OutboundApi.md#post_outbound_dnclist_phonenumbers) | **POST** /api/v2/outbound/dnclists/{dncListId}/phonenumbers | Add phone numbers to a DNC list.
[**post_outbound_dnclists**](OutboundApi.md#post_outbound_dnclists) | **POST** /api/v2/outbound/dnclists | Create dialer DNC list
[**post_outbound_messagingcampaigns**](OutboundApi.md#post_outbound_messagingcampaigns) | **POST** /api/v2/outbound/messagingcampaigns | Create a Messaging Campaign
[**post_outbound_messagingcampaigns_progress**](OutboundApi.md#post_outbound_messagingcampaigns_progress) | **POST** /api/v2/outbound/messagingcampaigns/progress | Get progress for a list of messaging campaigns
[**post_outbound_rulesets**](OutboundApi.md#post_outbound_rulesets) | **POST** /api/v2/outbound/rulesets | Create a Rule Set.
[**post_outbound_sequences**](OutboundApi.md#post_outbound_sequences) | **POST** /api/v2/outbound/sequences | Create a new campaign sequence.
[**put_outbound_attemptlimit**](OutboundApi.md#put_outbound_attemptlimit) | **PUT** /api/v2/outbound/attemptlimits/{attemptLimitsId} | Update attempt limits
[**put_outbound_callabletimeset**](OutboundApi.md#put_outbound_callabletimeset) | **PUT** /api/v2/outbound/callabletimesets/{callableTimeSetId} | Update callable time set
[**put_outbound_callanalysisresponseset**](OutboundApi.md#put_outbound_callanalysisresponseset) | **PUT** /api/v2/outbound/callanalysisresponsesets/{callAnalysisSetId} | Update a dialer call analysis response set.
[**put_outbound_campaign**](OutboundApi.md#put_outbound_campaign) | **PUT** /api/v2/outbound/campaigns/{campaignId} | Update a campaign.
[**put_outbound_campaign_agent**](OutboundApi.md#put_outbound_campaign_agent) | **PUT** /api/v2/outbound/campaigns/{campaignId}/agents/{userId} | Send notification that an agent's state changed 
[**put_outbound_campaignrule**](OutboundApi.md#put_outbound_campaignrule) | **PUT** /api/v2/outbound/campaignrules/{campaignRuleId} | Update Campaign Rule
[**put_outbound_contactlist**](OutboundApi.md#put_outbound_contactlist) | **PUT** /api/v2/outbound/contactlists/{contactListId} | Update a contact list.
[**put_outbound_contactlist_contact**](OutboundApi.md#put_outbound_contactlist_contact) | **PUT** /api/v2/outbound/contactlists/{contactListId}/contacts/{contactId} | Update a contact.
[**put_outbound_contactlistfilter**](OutboundApi.md#put_outbound_contactlistfilter) | **PUT** /api/v2/outbound/contactlistfilters/{contactListFilterId} | Update Contact List Filter
[**put_outbound_dnclist**](OutboundApi.md#put_outbound_dnclist) | **PUT** /api/v2/outbound/dnclists/{dncListId} | Update dialer DNC list
[**put_outbound_messagingcampaign**](OutboundApi.md#put_outbound_messagingcampaign) | **PUT** /api/v2/outbound/messagingcampaigns/{messagingCampaignId} | Update an Outbound Messaging Campaign
[**put_outbound_ruleset**](OutboundApi.md#put_outbound_ruleset) | **PUT** /api/v2/outbound/rulesets/{ruleSetId} | Update a Rule Set.
[**put_outbound_schedules_campaign**](OutboundApi.md#put_outbound_schedules_campaign) | **PUT** /api/v2/outbound/schedules/campaigns/{campaignId} | Update a new campaign schedule.
[**put_outbound_schedules_sequence**](OutboundApi.md#put_outbound_schedules_sequence) | **PUT** /api/v2/outbound/schedules/sequences/{sequenceId} | Update a new sequence schedule.
[**put_outbound_sequence**](OutboundApi.md#put_outbound_sequence) | **PUT** /api/v2/outbound/sequences/{sequenceId} | Update a new campaign sequence.
[**put_outbound_wrapupcodemappings**](OutboundApi.md#put_outbound_wrapupcodemappings) | **PUT** /api/v2/outbound/wrapupcodemappings | Update the Dialer wrap up code mapping.



## delete_outbound_attemptlimit

> delete_outbound_attemptlimit(attempt_limits_id)
Delete attempt limits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attempt_limits_id** | **String** | Attempt limits ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_callabletimeset

> delete_outbound_callabletimeset(callable_time_set_id)
Delete callable time set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callable_time_set_id** | **String** | Callable Time Set ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_callanalysisresponseset

> delete_outbound_callanalysisresponseset(call_analysis_set_id)
Delete a dialer call analysis response set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_analysis_set_id** | **String** | Call Analysis Response Set ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_campaign

> crate::models::Campaign delete_outbound_campaign(campaign_id)
Delete a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::Campaign**](Campaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_campaign_progress

> delete_outbound_campaign_progress(campaign_id)
Reset campaign progress and recycle the campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_campaignrule

> delete_outbound_campaignrule(campaign_rule_id)
Delete Campaign Rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_rule_id** | **String** | Campaign Rule ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_contactlist

> delete_outbound_contactlist(contact_list_id)
Delete a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | ContactList ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_contactlist_contact

> delete_outbound_contactlist_contact(contact_list_id, contact_id)
Delete a contact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contact List ID | [required] |
**contact_id** | **String** | Contact ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_contactlist_contacts

> delete_outbound_contactlist_contacts(contact_list_id, contact_ids)
Delete contacts from a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contact List ID | [required] |
**contact_ids** | [**Vec<String>**](String.md) | ContactIds to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_contactlistfilter

> delete_outbound_contactlistfilter(contact_list_filter_id)
Delete Contact List Filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_filter_id** | **String** | Contact List Filter ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_contactlists

> delete_outbound_contactlists(id)
Delete multiple contact lists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | contact list id(s) to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_dnclist

> delete_outbound_dnclist(dnc_list_id)
Delete dialer DNC list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | DncList ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_messagingcampaign

> crate::models::MessagingCampaign delete_outbound_messagingcampaign(messaging_campaign_id)
Delete an Outbound Messaging Campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_campaign_id** | **String** | The Messaging Campaign ID | [required] |

### Return type

[**crate::models::MessagingCampaign**](MessagingCampaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_ruleset

> delete_outbound_ruleset(rule_set_id)
Delete a Rule Set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_set_id** | **String** | Rule Set ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_schedules_campaign

> delete_outbound_schedules_campaign(campaign_id)
Delete a dialer campaign schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_schedules_sequence

> delete_outbound_schedules_sequence(sequence_id)
Delete a dialer sequence schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_id** | **String** | Sequence ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outbound_sequence

> delete_outbound_sequence(sequence_id)
Delete a dialer campaign sequence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_id** | **String** | Campaign Sequence ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_attemptlimit

> crate::models::AttemptLimits get_outbound_attemptlimit(attempt_limits_id)
Get attempt limits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attempt_limits_id** | **String** | Attempt limits ID | [required] |

### Return type

[**crate::models::AttemptLimits**](AttemptLimits.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_attemptlimits

> crate::models::AttemptLimitsEntityListing get_outbound_attemptlimits(page_size, page_number, allow_empty_result, filter_type, name, sort_by, sort_order)
Query attempt limits list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::AttemptLimitsEntityListing**](AttemptLimitsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_callabletimeset

> crate::models::CallableTimeSet get_outbound_callabletimeset(callable_time_set_id)
Get callable time set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callable_time_set_id** | **String** | Callable Time Set ID | [required] |

### Return type

[**crate::models::CallableTimeSet**](CallableTimeSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_callabletimesets

> crate::models::CallableTimeSetEntityListing get_outbound_callabletimesets(page_size, page_number, allow_empty_result, filter_type, name, sort_by, sort_order)
Query callable time set list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::CallableTimeSetEntityListing**](CallableTimeSetEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_callanalysisresponseset

> crate::models::ResponseSet get_outbound_callanalysisresponseset(call_analysis_set_id)
Get a dialer call analysis response set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_analysis_set_id** | **String** | Call Analysis Response Set ID | [required] |

### Return type

[**crate::models::ResponseSet**](ResponseSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_callanalysisresponsesets

> crate::models::ResponseSetEntityListing get_outbound_callanalysisresponsesets(page_size, page_number, allow_empty_result, filter_type, name, sort_by, sort_order)
Query a list of dialer call analysis response sets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::ResponseSetEntityListing**](ResponseSetEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaign

> crate::models::Campaign get_outbound_campaign(campaign_id)
Get dialer campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::Campaign**](Campaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaign_agentownedmappingpreview_results

> crate::models::AgentOwnedMappingPreviewListing get_outbound_campaign_agentownedmappingpreview_results(campaign_id)
Get a preview of how agents will be mapped to this campaign's contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::AgentOwnedMappingPreviewListing**](AgentOwnedMappingPreviewListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaign_diagnostics

> crate::models::CampaignDiagnostics get_outbound_campaign_diagnostics(campaign_id)
Get campaign diagnostics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::CampaignDiagnostics**](CampaignDiagnostics.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaign_interactions

> crate::models::CampaignInteractions get_outbound_campaign_interactions(campaign_id)
Get dialer campaign interactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::CampaignInteractions**](CampaignInteractions.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaign_progress

> crate::models::CampaignProgress get_outbound_campaign_progress(campaign_id)
Get campaign progress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::CampaignProgress**](CampaignProgress.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaign_stats

> crate::models::CampaignStats get_outbound_campaign_stats(campaign_id)
Get statistics about a Dialer Campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::CampaignStats**](CampaignStats.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaignrule

> crate::models::CampaignRule get_outbound_campaignrule(campaign_rule_id)
Get Campaign Rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_rule_id** | **String** | Campaign Rule ID | [required] |

### Return type

[**crate::models::CampaignRule**](CampaignRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaignrules

> crate::models::CampaignRuleEntityListing get_outbound_campaignrules(page_size, page_number, allow_empty_result, filter_type, name, sort_by, sort_order)
Query Campaign Rule list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::CampaignRuleEntityListing**](CampaignRuleEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaigns

> crate::models::CampaignEntityListing get_outbound_campaigns(page_size, page_number, filter_type, name, id, contact_list_id, dnc_list_ids, distribution_queue_id, edge_group_id, call_analysis_response_set_id, division_id, sort_by, sort_order)
Query a list of dialer campaigns.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**contact_list_id** | Option<**String**> | Contact List ID |  |
**dnc_list_ids** | Option<**String**> | DNC list ID |  |
**distribution_queue_id** | Option<**String**> | Distribution queue ID |  |
**edge_group_id** | Option<**String**> | Edge group ID |  |
**call_analysis_response_set_id** | Option<**String**> | Call analysis response set ID |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::CampaignEntityListing**](CampaignEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaigns_all

> crate::models::CommonCampaignEntityListing get_outbound_campaigns_all(page_size, page_number, id, name, division_id, media_type, sort_order)
Query across all types of campaigns by division

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**id** | Option<[**Vec<String>**](String.md)> | Campaign ID(s) |  |
**name** | Option<**String**> | Campaign name(s) |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |
**media_type** | Option<[**Vec<String>**](String.md)> | Media type(s) |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::CommonCampaignEntityListing**](CommonCampaignEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaigns_all_divisionviews

> crate::models::CommonCampaignDivisionViewEntityListing get_outbound_campaigns_all_divisionviews(page_size, page_number, id, name, division_id, media_type, sort_order)
Query across all types of campaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**id** | Option<[**Vec<String>**](String.md)> | Campaign ID(s) |  |
**name** | Option<**String**> | Campaign name(s) |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |
**media_type** | Option<[**Vec<String>**](String.md)> | Media type(s) |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::CommonCampaignDivisionViewEntityListing**](CommonCampaignDivisionViewEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaigns_divisionview

> crate::models::CampaignDivisionView get_outbound_campaigns_divisionview(campaign_id)
Get a basic Campaign information object

This returns a simplified version of a Campaign, consisting of name and division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::CampaignDivisionView**](CampaignDivisionView.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_campaigns_divisionviews

> crate::models::CampaignDivisionViewListing get_outbound_campaigns_divisionviews(page_size, page_number, filter_type, name, id, sort_by, sort_order)
Query a list of basic Campaign information objects

This returns a simplified version of a Campaign, consisting of name and division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::CampaignDivisionViewListing**](CampaignDivisionViewListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlist

> crate::models::ContactList get_outbound_contactlist(contact_list_id, include_import_status, include_size)
Get a dialer contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | ContactList ID | [required] |
**include_import_status** | Option<**bool**> | Import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]

### Return type

[**crate::models::ContactList**](ContactList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlist_contact

> crate::models::DialerContact get_outbound_contactlist_contact(contact_list_id, contact_id)
Get a contact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contact List ID | [required] |
**contact_id** | **String** | Contact ID | [required] |

### Return type

[**crate::models::DialerContact**](DialerContact.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlist_export

> crate::models::ExportUri get_outbound_contactlist_export(contact_list_id, download)
Get the URI of a contact list export.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | ContactList ID | [required] |
**download** | Option<**String**> | Redirect to download uri |  |[default to false]

### Return type

[**crate::models::ExportUri**](ExportUri.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlist_importstatus

> crate::models::ImportStatus get_outbound_contactlist_importstatus(contact_list_id)
Get dialer contactList import status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | ContactList ID | [required] |

### Return type

[**crate::models::ImportStatus**](ImportStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlist_timezonemappingpreview

> crate::models::TimeZoneMappingPreview get_outbound_contactlist_timezonemappingpreview(contact_list_id)
Preview the result of applying Automatic Time Zone Mapping to a contact list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | ContactList ID | [required] |

### Return type

[**crate::models::TimeZoneMappingPreview**](TimeZoneMappingPreview.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlistfilter

> crate::models::ContactListFilter get_outbound_contactlistfilter(contact_list_filter_id)
Get Contact list filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_filter_id** | **String** | Contact List Filter ID | [required] |

### Return type

[**crate::models::ContactListFilter**](ContactListFilter.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlistfilters

> crate::models::ContactListFilterEntityListing get_outbound_contactlistfilters(page_size, page_number, allow_empty_result, filter_type, name, sort_by, sort_order, contact_list_id)
Query Contact list filters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]
**contact_list_id** | Option<**String**> | Contact List ID |  |

### Return type

[**crate::models::ContactListFilterEntityListing**](ContactListFilterEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlists

> crate::models::ContactListEntityListing get_outbound_contactlists(include_import_status, include_size, page_size, page_number, allow_empty_result, filter_type, name, id, division_id, sort_by, sort_order)
Query a list of contact lists.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_import_status** | Option<**bool**> | Include import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::ContactListEntityListing**](ContactListEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlists_divisionview

> crate::models::ContactListDivisionView get_outbound_contactlists_divisionview(contact_list_id, include_import_status, include_size)
Get a basic ContactList information object

This returns a simplified version of a ContactList, consisting of the name, division, column names, phone columns, import status, and size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contactlist ID | [required] |
**include_import_status** | Option<**bool**> | Include import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]

### Return type

[**crate::models::ContactListDivisionView**](ContactListDivisionView.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_contactlists_divisionviews

> crate::models::ContactListDivisionViewListing get_outbound_contactlists_divisionviews(include_import_status, include_size, page_size, page_number, filter_type, name, id, sort_by, sort_order)
Query a list of simplified contact list objects.

This return a simplified version of contact lists, consisting of the name, division, column names, phone columns, import status, and size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_import_status** | Option<**bool**> | Include import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::ContactListDivisionViewListing**](ContactListDivisionViewListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_dnclist

> crate::models::DncList get_outbound_dnclist(dnc_list_id, include_import_status, include_size)
Get dialer DNC list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | DncList ID | [required] |
**include_import_status** | Option<**bool**> | Import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]

### Return type

[**crate::models::DncList**](DncList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_dnclist_export

> crate::models::ExportUri get_outbound_dnclist_export(dnc_list_id, download)
Get the URI of a DNC list export.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | DncList ID | [required] |
**download** | Option<**String**> | Redirect to download uri |  |[default to false]

### Return type

[**crate::models::ExportUri**](ExportUri.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_dnclist_importstatus

> crate::models::ImportStatus get_outbound_dnclist_importstatus(dnc_list_id)
Get dialer dncList import status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | DncList ID | [required] |

### Return type

[**crate::models::ImportStatus**](ImportStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_dnclists

> crate::models::DncListEntityListing get_outbound_dnclists(include_import_status, include_size, page_size, page_number, allow_empty_result, filter_type, name, dnc_source_type, division_id, sort_by, sort_order)
Query dialer DNC lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_import_status** | Option<**bool**> | Import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**dnc_source_type** | Option<**String**> | DncSourceType |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |

### Return type

[**crate::models::DncListEntityListing**](DncListEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_dnclists_divisionview

> crate::models::DncListDivisionView get_outbound_dnclists_divisionview(dnc_list_id, include_import_status, include_size)
Get a basic DncList information object

This returns a simplified version of a DncList, consisting of the name, division, import status, and size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | Dnclist ID | [required] |
**include_import_status** | Option<**bool**> | Include import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]

### Return type

[**crate::models::DncListDivisionView**](DncListDivisionView.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_dnclists_divisionviews

> crate::models::DncListDivisionViewListing get_outbound_dnclists_divisionviews(include_import_status, include_size, page_size, page_number, filter_type, name, dnc_source_type, id, sort_by, sort_order)
Query a list of simplified dnc list objects.

This return a simplified version of dnc lists, consisting of the name, division, import status, and size.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_import_status** | Option<**bool**> | Include import status |  |[default to false]
**include_size** | Option<**bool**> | Include size |  |[default to false]
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**dnc_source_type** | Option<**String**> | DncSourceType |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::DncListDivisionViewListing**](DncListDivisionViewListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_event

> crate::models::EventLog get_outbound_event(event_id)
Get Dialer Event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event Log ID | [required] |

### Return type

[**crate::models::EventLog**](EventLog.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_events

> crate::models::DialerEventEntityListing get_outbound_events(page_size, page_number, filter_type, category, level, sort_by, sort_order)
Query Event Logs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**category** | Option<**String**> | Category |  |
**level** | Option<**String**> | Level |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::DialerEventEntityListing**](DialerEventEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_messagingcampaign

> crate::models::MessagingCampaign get_outbound_messagingcampaign(messaging_campaign_id)
Get an Outbound Messaging Campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_campaign_id** | **String** | The Messaging Campaign ID | [required] |

### Return type

[**crate::models::MessagingCampaign**](MessagingCampaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_messagingcampaign_progress

> crate::models::CampaignProgress get_outbound_messagingcampaign_progress(messaging_campaign_id)
Get messaging campaign's progress

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_campaign_id** | **String** | The Messaging Campaign ID | [required] |

### Return type

[**crate::models::CampaignProgress**](CampaignProgress.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_messagingcampaigns

> crate::models::MessagingCampaignEntityListing get_outbound_messagingcampaigns(page_size, page_number, sort_by, sort_order, name, contact_list_id, division_id, _type, sender_sms_phone_number, id)
Query a list of Messaging Campaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | The field to sort by |  |[default to name]
**sort_order** | Option<**String**> | The direction to sort |  |[default to ascending]
**name** | Option<**String**> | Name |  |
**contact_list_id** | Option<**String**> | Contact List ID |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |
**_type** | Option<**String**> | Campaign Type |  |
**sender_sms_phone_number** | Option<**String**> | Sender SMS Phone Number |  |
**id** | Option<[**Vec<String>**](String.md)> | A list of messaging campaign ids to bulk fetch |  |

### Return type

[**crate::models::MessagingCampaignEntityListing**](MessagingCampaignEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_messagingcampaigns_divisionview

> crate::models::MessagingCampaignDivisionView get_outbound_messagingcampaigns_divisionview(messaging_campaign_id)
Get a basic Messaging Campaign information object

This returns a simplified version of a Messaging Campaign, consisting of id, name, and division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_campaign_id** | **String** | The Messaging Campaign ID | [required] |

### Return type

[**crate::models::MessagingCampaignDivisionView**](MessagingCampaignDivisionView.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_messagingcampaigns_divisionviews

> crate::models::MessagingCampaignDivisionViewEntityListing get_outbound_messagingcampaigns_divisionviews(page_size, page_number, sort_order, name, _type, id, sender_sms_phone_number)
Query a list of basic Messaging Campaign information objects

This returns a listing of simplified Messaging Campaigns, each consisting of id, name, and division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | The direction to sort |  |[default to a]
**name** | Option<**String**> | Name |  |
**_type** | Option<**String**> | Campaign Type |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**sender_sms_phone_number** | Option<**String**> | Sender SMS Phone Number |  |

### Return type

[**crate::models::MessagingCampaignDivisionViewEntityListing**](MessagingCampaignDivisionViewEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_ruleset

> crate::models::RuleSet get_outbound_ruleset(rule_set_id)
Get a Rule Set by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_set_id** | **String** | Rule Set ID | [required] |

### Return type

[**crate::models::RuleSet**](RuleSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_rulesets

> crate::models::RuleSetEntityListing get_outbound_rulesets(page_size, page_number, allow_empty_result, filter_type, name, sort_by, sort_order)
Query a list of Rule Sets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::RuleSetEntityListing**](RuleSetEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_schedules_campaign

> crate::models::CampaignSchedule get_outbound_schedules_campaign(campaign_id)
Get a dialer campaign schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**crate::models::CampaignSchedule**](CampaignSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_schedules_campaigns

> Vec<crate::models::CampaignSchedule> get_outbound_schedules_campaigns()
Query for a list of dialer campaign schedules.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CampaignSchedule>**](CampaignSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_schedules_sequence

> crate::models::SequenceSchedule get_outbound_schedules_sequence(sequence_id)
Get a dialer sequence schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_id** | **String** | Sequence ID | [required] |

### Return type

[**crate::models::SequenceSchedule**](SequenceSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_schedules_sequences

> Vec<crate::models::SequenceSchedule> get_outbound_schedules_sequences()
Query for a list of dialer sequence schedules.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SequenceSchedule>**](SequenceSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_sequence

> crate::models::CampaignSequence get_outbound_sequence(sequence_id)
Get a dialer campaign sequence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_id** | **String** | Campaign Sequence ID | [required] |

### Return type

[**crate::models::CampaignSequence**](CampaignSequence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_sequences

> crate::models::CampaignSequenceEntityListing get_outbound_sequences(page_size, page_number, allow_empty_result, filter_type, name, sort_by, sort_order)
Query a list of dialer campaign sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size. The max that will be returned is 100. |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**allow_empty_result** | Option<**bool**> | Whether to return an empty page when there are no results for that page |  |[default to false]
**filter_type** | Option<**String**> | Filter type |  |[default to Prefix]
**name** | Option<**String**> | Name |  |
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to a]

### Return type

[**crate::models::CampaignSequenceEntityListing**](CampaignSequenceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_settings

> crate::models::OutboundSettings get_outbound_settings()
Get the outbound settings for this organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OutboundSettings**](OutboundSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_outbound_wrapupcodemappings

> crate::models::WrapUpCodeMapping get_outbound_wrapupcodemappings()
Get the Dialer wrap up code mapping.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WrapUpCodeMapping**](WrapUpCodeMapping.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_outbound_settings

> patch_outbound_settings(body)
Update the outbound settings for this organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OutboundSettings**](OutboundSettings.md) | outboundSettings | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_attemptlimits

> crate::models::AttemptLimits post_outbound_attemptlimits(body)
Create attempt limits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AttemptLimits**](AttemptLimits.md) | AttemptLimits | [required] |

### Return type

[**crate::models::AttemptLimits**](AttemptLimits.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_audits

> crate::models::AuditSearchResult post_outbound_audits(body, page_size, page_number, sort_by, sort_order, facets_only)
Retrieves audits for dialer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DialerAuditRequest**](DialerAuditRequest.md) | AuditSearch | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to entity.name]
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]
**facets_only** | Option<**bool**> | Facets only |  |[default to false]

### Return type

[**crate::models::AuditSearchResult**](AuditSearchResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_callabletimesets

> crate::models::CallableTimeSet post_outbound_callabletimesets(body)
Create callable time set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CallableTimeSet**](CallableTimeSet.md) | DialerCallableTimeSet | [required] |

### Return type

[**crate::models::CallableTimeSet**](CallableTimeSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_callanalysisresponsesets

> crate::models::ResponseSet post_outbound_callanalysisresponsesets(body)
Create a dialer call analysis response set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ResponseSet**](ResponseSet.md) | ResponseSet | [required] |

### Return type

[**crate::models::ResponseSet**](ResponseSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_campaign_agentownedmappingpreview

> serde_json::Value post_outbound_campaign_agentownedmappingpreview(campaign_id)
Initiate request for a preview of how agents will be mapped to this campaign's contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_campaign_callback_schedule

> crate::models::ContactCallbackRequest post_outbound_campaign_callback_schedule(campaign_id, body)
Schedule a Callback for a Dialer Campaign (Deprecated)

This endpoint is deprecated and may have unexpected results. Please use \"/conversations/{conversationId}/participants/{participantId}/callbacks instead.\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |
**body** | [**ContactCallbackRequest**](ContactCallbackRequest.md) | ContactCallbackRequest | [required] |

### Return type

[**crate::models::ContactCallbackRequest**](ContactCallbackRequest.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_campaignrules

> crate::models::CampaignRule post_outbound_campaignrules(body)
Create Campaign Rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CampaignRule**](CampaignRule.md) | CampaignRule | [required] |

### Return type

[**crate::models::CampaignRule**](CampaignRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_campaigns

> crate::models::Campaign post_outbound_campaigns(body)
Create a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Campaign**](Campaign.md) | Campaign | [required] |

### Return type

[**crate::models::Campaign**](Campaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_campaigns_progress

> Vec<crate::models::CampaignProgress> post_outbound_campaigns_progress(body)
Get progress for a list of campaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<String>**](String.md) | Campaign IDs | [required] |

### Return type

[**Vec<crate::models::CampaignProgress>**](CampaignProgress.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_contactlist_clear

> post_outbound_contactlist_clear(contact_list_id)
Deletes all contacts out of a list. All outstanding recalls or rule-scheduled callbacks for non-preview campaigns configured with the contactlist will be cancelled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contact List ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_contactlist_contacts

> Vec<crate::models::DialerContact> post_outbound_contactlist_contacts(contact_list_id, body, priority, clear_system_data, do_not_queue)
Add contacts to a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contact List ID | [required] |
**body** | [**Vec<crate::models::WritableDialerContact>**](WritableDialerContact.md) | Contact | [required] |
**priority** | Option<**bool**> | Contact priority. True means the contact(s) will be dialed next; false means the contact will go to the end of the contact queue. |  |
**clear_system_data** | Option<**bool**> | Clear system data. True means the system columns (attempts, callable status, etc) stored on the contact will be cleared if the contact already exists; false means they won't. |  |
**do_not_queue** | Option<**bool**> | Do not queue. True means that updated contacts will not have their positions in the queue altered, so contacts that have already been dialed will not be redialed. For new contacts, this parameter has no effect; False means that updated contacts will be re-queued, according to the 'priority' parameter. |  |

### Return type

[**Vec<crate::models::DialerContact>**](DialerContact.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_contactlist_contacts_bulk

> Vec<crate::models::DialerContact> post_outbound_contactlist_contacts_bulk(contact_list_id, body)
Get contacts from a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contact List ID | [required] |
**body** | [**Vec<String>**](String.md) | ContactIds to get. | [required] |

### Return type

[**Vec<crate::models::DialerContact>**](DialerContact.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_contactlist_export

> crate::models::DomainEntityRef post_outbound_contactlist_export(contact_list_id)
Initiate the export of a contact list.

Returns 200 if received OK.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | ContactList ID | [required] |

### Return type

[**crate::models::DomainEntityRef**](DomainEntityRef.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_contactlistfilters

> crate::models::ContactListFilter post_outbound_contactlistfilters(body)
Create Contact List Filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContactListFilter**](ContactListFilter.md) | ContactListFilter | [required] |

### Return type

[**crate::models::ContactListFilter**](ContactListFilter.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_contactlistfilters_preview

> crate::models::FilterPreviewResponse post_outbound_contactlistfilters_preview(body)
Get a preview of the output of a contact list filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContactListFilter**](ContactListFilter.md) | ContactListFilter | [required] |

### Return type

[**crate::models::FilterPreviewResponse**](FilterPreviewResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_contactlists

> crate::models::ContactList post_outbound_contactlists(body)
Create a contact List.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContactList**](ContactList.md) | ContactList | [required] |

### Return type

[**crate::models::ContactList**](ContactList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_conversation_dnc

> post_outbound_conversation_dnc(conversation_id)
Add phone numbers to a Dialer DNC list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_dnclist_export

> crate::models::DomainEntityRef post_outbound_dnclist_export(dnc_list_id)
Initiate the export of a dnc list.

Returns 200 if received OK.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | DncList ID | [required] |

### Return type

[**crate::models::DomainEntityRef**](DomainEntityRef.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_dnclist_phonenumbers

> post_outbound_dnclist_phonenumbers(dnc_list_id, body)
Add phone numbers to a DNC list.

Only Internal DNC lists may be appended to

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | DncList ID | [required] |
**body** | [**Vec<String>**](String.md) | DNC Phone Numbers | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_dnclists

> crate::models::DncList post_outbound_dnclists(body)
Create dialer DNC list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DncListCreate**](DncListCreate.md) | DncList | [required] |

### Return type

[**crate::models::DncList**](DncList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_messagingcampaigns

> crate::models::MessagingCampaign post_outbound_messagingcampaigns(body)
Create a Messaging Campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**MessagingCampaign**](MessagingCampaign.md) | Messaging Campaign | [required] |

### Return type

[**crate::models::MessagingCampaign**](MessagingCampaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_messagingcampaigns_progress

> Vec<crate::models::CampaignProgress> post_outbound_messagingcampaigns_progress(body)
Get progress for a list of messaging campaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<String>**](String.md) | Messaging Campaign IDs | [required] |

### Return type

[**Vec<crate::models::CampaignProgress>**](CampaignProgress.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_rulesets

> crate::models::RuleSet post_outbound_rulesets(body)
Create a Rule Set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RuleSet**](RuleSet.md) | RuleSet | [required] |

### Return type

[**crate::models::RuleSet**](RuleSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_outbound_sequences

> crate::models::CampaignSequence post_outbound_sequences(body)
Create a new campaign sequence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CampaignSequence**](CampaignSequence.md) | Organization | [required] |

### Return type

[**crate::models::CampaignSequence**](CampaignSequence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_attemptlimit

> crate::models::AttemptLimits put_outbound_attemptlimit(attempt_limits_id, body)
Update attempt limits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**attempt_limits_id** | **String** | Attempt limits ID | [required] |
**body** | [**AttemptLimits**](AttemptLimits.md) | AttemptLimits | [required] |

### Return type

[**crate::models::AttemptLimits**](AttemptLimits.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_callabletimeset

> crate::models::CallableTimeSet put_outbound_callabletimeset(callable_time_set_id, body)
Update callable time set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callable_time_set_id** | **String** | Callable Time Set ID | [required] |
**body** | [**CallableTimeSet**](CallableTimeSet.md) | DialerCallableTimeSet | [required] |

### Return type

[**crate::models::CallableTimeSet**](CallableTimeSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_callanalysisresponseset

> crate::models::ResponseSet put_outbound_callanalysisresponseset(call_analysis_set_id, body)
Update a dialer call analysis response set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**call_analysis_set_id** | **String** | Call Analysis Response Set ID | [required] |
**body** | [**ResponseSet**](ResponseSet.md) | ResponseSet | [required] |

### Return type

[**crate::models::ResponseSet**](ResponseSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_campaign

> crate::models::Campaign put_outbound_campaign(campaign_id, body)
Update a campaign.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |
**body** | [**Campaign**](Campaign.md) | Campaign | [required] |

### Return type

[**crate::models::Campaign**](Campaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_campaign_agent

> String put_outbound_campaign_agent(campaign_id, user_id, body)
Send notification that an agent's state changed 

New agent state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |
**user_id** | **String** | Agent's user ID | [required] |
**body** | [**Agent**](Agent.md) | agent | [required] |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_campaignrule

> crate::models::CampaignRule put_outbound_campaignrule(campaign_rule_id, body)
Update Campaign Rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_rule_id** | **String** | Campaign Rule ID | [required] |
**body** | [**CampaignRule**](CampaignRule.md) | CampaignRule | [required] |

### Return type

[**crate::models::CampaignRule**](CampaignRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_contactlist

> crate::models::ContactList put_outbound_contactlist(contact_list_id, body)
Update a contact list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | ContactList ID | [required] |
**body** | [**ContactList**](ContactList.md) | ContactList | [required] |

### Return type

[**crate::models::ContactList**](ContactList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_contactlist_contact

> crate::models::DialerContact put_outbound_contactlist_contact(contact_list_id, contact_id, body)
Update a contact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_id** | **String** | Contact List ID | [required] |
**contact_id** | **String** | Contact ID | [required] |
**body** | [**DialerContact**](DialerContact.md) | Contact | [required] |

### Return type

[**crate::models::DialerContact**](DialerContact.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_contactlistfilter

> crate::models::ContactListFilter put_outbound_contactlistfilter(contact_list_filter_id, body)
Update Contact List Filter

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_list_filter_id** | **String** | Contact List Filter ID | [required] |
**body** | [**ContactListFilter**](ContactListFilter.md) | ContactListFilter | [required] |

### Return type

[**crate::models::ContactListFilter**](ContactListFilter.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_dnclist

> crate::models::DncList put_outbound_dnclist(dnc_list_id, body)
Update dialer DNC list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dnc_list_id** | **String** | DncList ID | [required] |
**body** | [**DncList**](DncList.md) | DncList | [required] |

### Return type

[**crate::models::DncList**](DncList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_messagingcampaign

> crate::models::MessagingCampaign put_outbound_messagingcampaign(messaging_campaign_id, body)
Update an Outbound Messaging Campaign

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messaging_campaign_id** | **String** | The Messaging Campaign ID | [required] |
**body** | [**MessagingCampaign**](MessagingCampaign.md) | MessagingCampaign | [required] |

### Return type

[**crate::models::MessagingCampaign**](MessagingCampaign.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_ruleset

> crate::models::RuleSet put_outbound_ruleset(rule_set_id, body)
Update a Rule Set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_set_id** | **String** | Rule Set ID | [required] |
**body** | [**RuleSet**](RuleSet.md) | RuleSet | [required] |

### Return type

[**crate::models::RuleSet**](RuleSet.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_schedules_campaign

> crate::models::CampaignSchedule put_outbound_schedules_campaign(campaign_id, body)
Update a new campaign schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign_id** | **String** | Campaign ID | [required] |
**body** | [**CampaignSchedule**](CampaignSchedule.md) | CampaignSchedule | [required] |

### Return type

[**crate::models::CampaignSchedule**](CampaignSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_schedules_sequence

> crate::models::SequenceSchedule put_outbound_schedules_sequence(sequence_id, body)
Update a new sequence schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_id** | **String** | Sequence ID | [required] |
**body** | [**SequenceSchedule**](SequenceSchedule.md) | SequenceSchedule | [required] |

### Return type

[**crate::models::SequenceSchedule**](SequenceSchedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_sequence

> crate::models::CampaignSequence put_outbound_sequence(sequence_id, body)
Update a new campaign sequence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_id** | **String** | Campaign Sequence ID | [required] |
**body** | [**CampaignSequence**](CampaignSequence.md) | Organization | [required] |

### Return type

[**crate::models::CampaignSequence**](CampaignSequence.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_outbound_wrapupcodemappings

> crate::models::WrapUpCodeMapping put_outbound_wrapupcodemappings(body)
Update the Dialer wrap up code mapping.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WrapUpCodeMapping**](WrapUpCodeMapping.md) | wrapUpCodeMapping | [required] |

### Return type

[**crate::models::WrapUpCodeMapping**](WrapUpCodeMapping.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

