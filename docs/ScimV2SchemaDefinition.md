# ScimV2SchemaDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the SCIM resource. Set by the service provider. \"caseExact\" is set to \"true\". \"mutability\" is set to \"readOnly\". \"returned\" is set to \"always\". | [optional][readonly]
**name** | Option<**String**> | The name of the schema. | [optional][readonly]
**description** | Option<**String**> | The description of the schema. | [optional][readonly]
**attributes** | Option<[**Vec<crate::models::ScimV2SchemaAttribute>**](ScimV2SchemaAttribute.md)> | The list of service provider attributes. | [optional][readonly]
**meta** | Option<[**crate::models::ScimMetadata**](ScimMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


