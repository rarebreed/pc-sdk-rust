# UpdateUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**chat** | Option<[**crate::models::Chat**](Chat.md)> |  | [optional]
**department** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**primary_contact_info** | Option<[**Vec<crate::models::Contact>**](Contact.md)> | The address(s) used for primary contact. Updates to the corresponding address in the addresses list will be reflected here. | [optional][readonly]
**addresses** | Option<[**Vec<crate::models::Contact>**](Contact.md)> | Email address, phone number, and/or extension for this user. One entry is allowed per media type | [optional]
**title** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**manager** | Option<**String**> |  | [optional]
**images** | Option<[**Vec<crate::models::UserImage>**](UserImage.md)> |  | [optional]
**version** | **i32** | This value should be the current version of the user. The current version can be obtained with a GET on the user before doing a PATCH. | 
**profile_skills** | Option<**Vec<String>**> | Profile skills possessed by the user | [optional]
**locations** | Option<[**Vec<crate::models::Location>**](Location.md)> | The user placement at each site location. | [optional]
**groups** | Option<[**Vec<crate::models::Group>**](Group.md)> | The groups the user is a member of | [optional]
**state** | Option<**String**> | The state of the user. This property can be used to restore a deleted user or transition between active and inactive. If specified, it is the only modifiable field. | [optional]
**acd_auto_answer** | Option<**bool**> | The value that denotes if acdAutoAnswer is set on the user | [optional]
**certifications** | Option<**Vec<String>**> |  | [optional]
**biography** | Option<[**crate::models::Biography**](Biography.md)> |  | [optional]
**employer_info** | Option<[**crate::models::EmployerInfo**](EmployerInfo.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


