# TrustGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The group name. | 
**description** | Option<**String**> |  | [optional]
**date_modified** | Option<**String**> | Last modified date/time. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**member_count** | Option<**i64**> | Number of members. | [optional][readonly]
**state** | Option<**String**> | Active, inactive, or deleted state. | [optional][readonly]
**version** | Option<**i32**> | Current version for this resource. | [optional][readonly]
**_type** | **String** | Type of group. | 
**images** | Option<[**Vec<crate::models::UserImage>**](UserImage.md)> |  | [optional]
**addresses** | Option<[**Vec<crate::models::GroupContact>**](GroupContact.md)> |  | [optional]
**rules_visible** | **bool** | Are membership rules visible to the person requesting to view the group | 
**visibility** | **String** | Who can view this group | 
**owners** | Option<[**Vec<crate::models::User>**](User.md)> | Owners of the group | [optional]
**date_created** | Option<**String**> | The date on which the trusted group was added. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**created_by** | Option<[**crate::models::OrgUser**](OrgUser.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


