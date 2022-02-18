# SmsAddress

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of this address. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**street** | Option<**String**> | The number and street address where this address is located. | [optional]
**city** | Option<**String**> | The city in which this address is in | [optional]
**region** | Option<**String**> | The state or region this address is in | [optional]
**postal_code** | Option<**String**> | The postal code this address is in | [optional]
**country_code** | Option<**String**> | The ISO country code of this address | [optional]
**validated** | Option<**bool**> | In some countries, addresses are validated to comply with local regulation. In those countries, if the address you provide does not pass validation, it will not be accepted as an Address. This value will be true if the Address has been validated, or false for countries that don't require validation or if the Address is non-compliant. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


