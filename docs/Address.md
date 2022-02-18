# Address

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | This will be nameRaw if present, or a locality lookup of the address field otherwise. | [optional]
**name_raw** | Option<**String**> | The name as close to the bits on the wire as possible. | [optional]
**address_normalized** | Option<**String**> | The normalized address. This field is acquired from the Address Normalization Table.  The addressRaw could have gone through some transformations, such as only using the numeric portion, before being run through the Address Normalization Table. | [optional]
**address_raw** | Option<**String**> | The address as close to the bits on the wire as possible. | [optional]
**address_displayable** | Option<**String**> | The displayable address. This field is acquired from the Address Normalization Table.  The addressRaw could have gone through some transformations, such as only using the numeric portion, before being run through the Address Normalization Table. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


