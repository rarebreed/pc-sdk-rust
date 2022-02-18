# CreateUser

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | User's full name | 
**department** | Option<**String**> |  | [optional]
**email** | **String** | User's email and username | 
**addresses** | Option<[**Vec<crate::models::Contact>**](Contact.md)> | Email addresses and phone numbers for this user | [optional]
**title** | Option<**String**> |  | [optional]
**password** | **String** | User's password | 
**division_id** | **String** | The division to which this user will belong | 
**state** | Option<**String**> | Optional initialized state of the user. If not specified, state will be Active if invites are sent, otherwise Inactive. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


