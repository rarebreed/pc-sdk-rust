# MessagingCampaign

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**date_created** | Option<**String**> | Creation time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | Last modified time of the entity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | Required for updates, must match the version number of the most recent update | [optional]
**division** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**campaign_status** | Option<**String**> | The current status of the messaging campaign. A messaging campaign may be turned 'on' or 'off'. | [optional]
**callable_time_set** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**contact_list** | [**crate::models::DomainEntityRef**](DomainEntityRef.md) |  | 
**dnc_lists** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | The dnc lists to check before sending a message for this messaging campaign. | [optional]
**always_running** | Option<**bool**> | Whether this messaging campaign is always running | [optional]
**contact_sorts** | Option<[**Vec<crate::models::ContactSort>**](ContactSort.md)> | The order in which to sort contacts for dialing, based on up to four columns. | [optional]
**messages_per_minute** | **i32** | How many messages this messaging campaign will send per minute. | 
**contact_list_filters** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | The contact list filter to check before sending a message for this messaging campaign. | [optional]
**errors** | Option<[**Vec<crate::models::RestErrorDetail>**](RestErrorDetail.md)> | A list of current error conditions associated with this messaging campaign. | [optional]
**sms_config** | Option<[**crate::models::SmsConfig**](SmsConfig.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


