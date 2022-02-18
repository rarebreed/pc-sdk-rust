# Campaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the Campaign. | 
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**contact_list** | [**crate::models::DomainEntityRef**](DomainEntityRef.md) |  | 
**queue** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**dialing_mode** | **String** | The strategy this Campaign will use for dialing. | 
**script** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**edge_group** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**site** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**campaign_status** | Option<**String**> | The current status of the Campaign. A Campaign may be turned 'on' or 'off'. Required for updates. | [optional]
**phone_columns** | [**Vec<crate::models::PhoneColumn>**](PhoneColumn.md) | The ContactPhoneNumberColumns on the ContactList that this Campaign should dial. | 
**abandon_rate** | Option<**f64**> | The targeted abandon rate percentage. Required for progressive, power, and predictive campaigns. | [optional]
**dnc_lists** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | DncLists for this Campaign to check before placing a call. | [optional]
**callable_time_set** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**call_analysis_response_set** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**errors** | Option<[**Vec<crate::models::RestErrorDetail>**](RestErrorDetail.md)> | A list of current error conditions associated with the campaign. | [optional][readonly]
**caller_name** | **String** | The caller id name to be displayed on the outbound call. | 
**caller_address** | **String** | The caller id phone number to be displayed on the outbound call. | 
**outbound_line_count** | Option<**i32**> | The number of outbound lines to be concurrently dialed. Only applicable to non-preview campaigns; only required for agentless. | [optional]
**rule_sets** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | Rule sets to be applied while this campaign is dialing. | [optional]
**skip_preview_disabled** | Option<**bool**> | Whether or not agents can skip previews without placing a call. Only applicable for preview campaigns. | [optional]
**preview_time_out_seconds** | Option<**i64**> | The number of seconds before a call will be automatically placed on a preview. A value of 0 indicates no automatic placement of calls. Only applicable to preview campaigns. | [optional]
**always_running** | Option<**bool**> | Indicates (when true) that the campaign will remain on after contacts are depleted, allowing additional contacts to be appended/added to the contact list and processed by the still-running campaign. The campaign can still be turned off manually. | [optional]
**contact_sort** | Option<[**crate::models::ContactSort**](ContactSort.md)> |  | [optional]
**contact_sorts** | Option<[**Vec<crate::models::ContactSort>**](ContactSort.md)> | The order in which to sort contacts for dialing, based on up to four columns. | [optional]
**no_answer_timeout** | Option<**i32**> | How long to wait before dispositioning a call as 'no-answer'. Default 30 seconds. Only applicable to non-preview campaigns. | [optional]
**call_analysis_language** | Option<**String**> | The language the edge will use to analyze the call. | [optional]
**priority** | Option<**i32**> | The priority of this campaign relative to other campaigns that are running on the same queue. 5 is the highest priority, 1 the lowest. | [optional]
**contact_list_filters** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | Filter to apply to the contact list before dialing. Currently a campaign can only have one filter applied. | [optional]
**division** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


