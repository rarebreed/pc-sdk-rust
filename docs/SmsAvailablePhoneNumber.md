# SmsAvailablePhoneNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**phone_number** | Option<**String**> | A phone number available for provisioning in E.164 format. E.g. +13175555555 or +34234234234 | [optional]
**country_code** | Option<**String**> | The ISO 3166-1 alpha-2 country code of the country this phone number is associated with. | [optional]
**region** | Option<**String**> | The region/province/state the phone number is associated with. | [optional]
**city** | Option<**String**> | The city the phone number is associated with. | [optional]
**capabilities** | Option<**Vec<String>**> | The capabilities of the phone number available for provisioning. | [optional]
**phone_number_type** | Option<**String**> | The type of phone number available for provisioning. | [optional]
**address_requirement** | Option<**String**> | The address requirement needed for provisioning this number. If there is a requirement, the address must be the residence or place of business of the individual or entity using the phone number. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


