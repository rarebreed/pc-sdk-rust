# GroupUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> | The group name. | [optional]
**description** | Option<**String**> |  | [optional]
**state** | Option<**String**> | State of the group. | [optional]
**version** | **i32** | Current version for this resource. | 
**images** | Option<[**Vec<crate::models::UserImage>**](UserImage.md)> |  | [optional]
**addresses** | Option<[**Vec<crate::models::GroupContact>**](GroupContact.md)> |  | [optional]
**rules_visible** | Option<**bool**> | Are membership rules visible to the person requesting to view the group | [optional]
**visibility** | Option<**String**> | Who can view this group | [optional]
**owner_ids** | Option<**Vec<String>**> | Owners of the group | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


