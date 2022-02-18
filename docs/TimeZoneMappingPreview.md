# TimeZoneMappingPreview

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**contact_list** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**contacts_per_time_zone** | Option<**::std::collections::HashMap<String, i64>**> | The number of contacts per time zone that mapped to only that time zone | [optional]
**contacts_mapped_using_zip_code** | Option<**::std::collections::HashMap<String, i64>**> | The number of contacts per time zone that mapped to only that time zone and were mapped using the zip code column | [optional]
**contacts_mapped_to_a_single_zone** | Option<**i64**> | The total number of contacts that mapped to a single time zone | [optional]
**contacts_mapped_to_a_single_zone_using_zip_code** | Option<**i64**> | The total number of contacts that mapped to a single time zone and were mapped using the zip code column | [optional]
**contacts_mapped_to_multiple_zones** | Option<**i64**> | The total number of contacts that mapped to multiple time zones | [optional]
**contacts_mapped_to_multiple_zones_using_zip_code** | Option<**i64**> | The total number of contacts that mapped to multiple time zones and were mapped using the zip code column | [optional]
**contacts_in_default_window** | Option<**i64**> | The total number of contacts that will be dialed during the default window | [optional]
**contact_list_size** | Option<**i64**> | The total number of contacts in the contact list | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


