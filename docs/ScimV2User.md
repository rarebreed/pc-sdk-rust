# ScimV2User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the SCIM resource. Set by the service provider. \"caseExact\" is set to \"true\". \"mutability\" is set to \"readOnly\". \"returned\" is set to \"always\". | [optional][readonly]
**schemas** | Option<**Vec<String>**> | The list of supported schemas. | [optional][readonly]
**active** | Option<**bool**> | Indicates whether the user's administrative status is active. | [optional]
**user_name** | Option<**String**> | The user's Genesys Cloud email address. Must be unique. | [optional]
**display_name** | Option<**String**> | The display name of the user. | [optional]
**password** | Option<**String**> | The new password for the Genesys Cloud user. Does not return an existing password. When creating a user, if a password is not supplied, then a password will be randomly generated that is 40 characters in length and contains five characters from each of the password policy groups. | [optional]
**title** | Option<**String**> | The user's title. | [optional]
**phone_numbers** | Option<[**Vec<crate::models::ScimPhoneNumber>**](ScimPhoneNumber.md)> | The list of the user's phone numbers. | [optional]
**emails** | Option<[**Vec<crate::models::ScimEmail>**](ScimEmail.md)> | The list of the user's email addresses. | [optional]
**external_id** | Option<**String**> | The external ID of the user. Set by the provisioning client. \"caseExact\" is set to \"true\". \"mutability\" is set to \"readWrite\". | [optional]
**groups** | Option<[**Vec<crate::models::ScimV2GroupReference>**](ScimV2GroupReference.md)> | The list of groups that the user is a member of. | [optional]
**roles** | Option<[**Vec<crate::models::ScimUserRole>**](ScimUserRole.md)> | The list of roles assigned to the user. | [optional]
**urnietfparamsscimschemasextensionenterprise2_0_user** | Option<[**crate::models::ScimV2EnterpriseUser**](ScimV2EnterpriseUser.md)> |  | [optional]
**urnietfparamsscimschemasextensiongenesyspurecloud2_0_user** | Option<[**crate::models::ScimUserExtensions**](ScimUserExtensions.md)> |  | [optional]
**meta** | Option<[**crate::models::ScimMetadata**](ScimMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


