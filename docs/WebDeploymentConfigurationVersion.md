# WebDeploymentConfigurationVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The configuration version ID | [optional][readonly]
**name** | **String** | The configuration version name | 
**version** | Option<**String**> | The version of the configuration | [optional][readonly]
**description** | Option<**String**> | The description of the configuration | [optional]
**languages** | Option<**Vec<String>**> | A list of languages supported on the configuration | [optional]
**default_language** | Option<**String**> | The default language to use for the configuration | [optional]
**messenger** | Option<[**crate::models::MessengerSettings**](MessengerSettings.md)> |  | [optional]
**position** | Option<[**crate::models::PositionSettings**](PositionSettings.md)> |  | [optional]
**support_center** | Option<[**crate::models::SupportCenterSettings**](SupportCenterSettings.md)> |  | [optional]
**cobrowse** | Option<[**crate::models::CobrowseSettings**](CobrowseSettings.md)> |  | [optional]
**journey_events** | Option<[**crate::models::JourneyEventsSettings**](JourneyEventsSettings.md)> |  | [optional]
**authentication_settings** | Option<[**crate::models::AuthenticationSettings**](AuthenticationSettings.md)> |  | [optional]
**date_created** | Option<**String**> | The date the configuration version was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | The date the configuration version was most recently modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_published** | Option<**String**> | The date the configuration version was most recently published. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**last_modified_user** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**created_user** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**published_user** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**status** | Option<**String**> | The current status of the configuration version | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


