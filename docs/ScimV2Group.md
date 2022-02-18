# ScimV2Group

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the SCIM resource. Set by the service provider. \"caseExact\" is set to \"true\". \"mutability\" is set to \"readOnly\". \"returned\" is set to \"always\". | [optional][readonly]
**schemas** | Option<**Vec<String>**> | The list of supported schemas. | [optional][readonly]
**display_name** | **String** | The display name of the group. | [readonly]
**external_id** | Option<**String**> | The external ID of the group. Set by the provisioning client. \"caseExact\" is set to \"true\". \"mutability\" is set to \"readWrite\". | [optional]
**members** | Option<[**Vec<crate::models::ScimV2MemberReference>**](ScimV2MemberReference.md)> | The list of members in the group. | [optional]
**meta** | Option<[**crate::models::ScimMetadata**](ScimMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


