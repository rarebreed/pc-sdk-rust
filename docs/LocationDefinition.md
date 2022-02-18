# LocationDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**contact_user** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**emergency_number** | Option<[**crate::models::LocationEmergencyNumber**](LocationEmergencyNumber.md)> |  | [optional]
**address** | Option<[**crate::models::LocationAddress**](LocationAddress.md)> |  | [optional]
**state** | Option<**String**> | Current state of the location entity | [optional]
**notes** | Option<**String**> | Notes for the location entity | [optional]
**version** | Option<**i32**> | Current version of the location entity, value to be supplied should be retrieved by a GET or on create/update response | [optional]
**path** | Option<**Vec<String>**> | A list of ancestor IDs in order | [optional][readonly]
**profile_image** | Option<[**Vec<crate::models::LocationImage>**](LocationImage.md)> | Profile image of the location entity, retrieved with ?expand=images query parameter | [optional][readonly]
**floorplan_image** | Option<[**Vec<crate::models::LocationImage>**](LocationImage.md)> | Floorplan images of the location entity, retrieved with ?expand=images query parameter | [optional][readonly]
**address_verification_details** | Option<[**crate::models::LocationAddressVerificationDetails**](LocationAddressVerificationDetails.md)> |  | [optional]
**address_verified** | Option<**bool**> | Boolean field which states if the address has been verified as an actual address | [optional][readonly]
**address_stored** | Option<**bool**> | Boolean field which states if the address has been stored for E911 | [optional][readonly]
**images** | Option<**String**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


