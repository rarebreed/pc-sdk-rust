# ScimConfigResourceType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the SCIM resource. Set by the service provider. \"caseExact\" is set to \"true\". \"mutability\" is set to \"readOnly\". \"returned\" is set to \"always\". | [optional][readonly]
**schemas** | Option<**Vec<String>**> | The list of supported schemas. | [optional][readonly]
**name** | Option<**String**> | The name of the resource type. | [optional][readonly]
**description** | Option<**String**> | The description of the resource type. | [optional][readonly]
**schema** | Option<**String**> | The URI of the primary or base schema for the resource type. | [optional][readonly]
**schema_extensions** | Option<[**Vec<crate::models::ScimConfigResourceTypeSchemaExtension>**](ScimConfigResourceTypeSchemaExtension.md)> | The list of schema extensions for the resource type. | [optional][readonly]
**endpoint** | Option<**String**> | The HTTP-addressable endpoint of the resource type. Appears after the base URL. | [optional][readonly]
**meta** | Option<[**crate::models::ScimMetadata**](ScimMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


