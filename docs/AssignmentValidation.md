# AssignmentValidation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**members_not_assigned** | Option<[**Vec<crate::models::UserReference>**](UserReference.md)> | The list of users that are not assigned to any custom performance profile | [optional]
**members_already_assigned** | Option<[**Vec<crate::models::UserReference>**](UserReference.md)> | The list of users that are already assigned to the requesting custom performance profile | [optional]
**members_already_assigned_to_other** | Option<[**Vec<crate::models::OtherProfileAssignment>**](OtherProfileAssignment.md)> | The list of users that are already assigned to other custom performance profiles | [optional]
**invalid_member_assignments** | Option<[**Vec<crate::models::InvalidAssignment>**](InvalidAssignment.md)> | The list of user id that are invalid for the gamfication service to handle | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


