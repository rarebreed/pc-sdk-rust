# LocationCreateDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the Location. Required for creates, not required for updates | 
**version** | Option<**i32**> | Current version of the location | [optional]
**state** | Option<**String**> | Current activity status of the location. | [optional]
**path** | Option<**Vec<String>**> | A list of ancestor ids | [optional]
**notes** | Option<**String**> | Notes for the location | [optional]
**contact_user** | Option<**String**> | The user id of the location contact | [optional]
**emergency_number** | Option<[**crate::models::LocationEmergencyNumber**](LocationEmergencyNumber.md)> |  | [optional]
**address** | Option<[**crate::models::LocationAddress**](LocationAddress.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


