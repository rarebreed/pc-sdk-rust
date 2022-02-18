# Organization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**default_language** | Option<**String**> | The default language for this organization. Example: 'en' | [optional]
**default_country_code** | Option<**String**> | The default country code for this organization. Example: 'US' | [optional]
**third_party_org_name** | Option<**String**> | The short name for the organization. This field is globally unique and cannot be changed. | [optional][readonly]
**third_party_uri** | Option<**String**> |  | [optional]
**domain** | Option<**String**> |  | [optional]
**version** | **i32** | The current version of the organization. | 
**state** | Option<**String**> | The current state. Examples are active, inactive, deleted. | [optional]
**default_site_id** | Option<**String**> |  | [optional]
**support_uri** | Option<**String**> | Email address where support tickets are sent to. | [optional]
**voicemail_enabled** | Option<**bool**> |  | [optional]
**product_platform** | Option<**String**> | Organizations Originating Platform. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**features** | Option<**::std::collections::HashMap<String, bool>**> | The state of features available for the organization. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


