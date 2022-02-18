# SmsPhoneNumberProvision

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**phone_number** | **String** | A phone number to be used for SMS communications. E.g. +13175555555 or +34234234234 | 
**phone_number_type** | **String** | Type of the phone number provisioned. | 
**country_code** | **String** | The ISO 3166-1 alpha-2 country code of the country this phone number is associated with. | 
**address_id** | Option<**String**> | The id of an address added on your account. Due to regulatory requirements in some countries, an address may be required when provisioning a sms number. In those cases you should provide the provisioned sms address id here | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


