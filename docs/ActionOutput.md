# ActionOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success_schema** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**success_schema_uri** | Option<**String**> | URI to retrieve success schema | [optional]
**error_schema** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**error_schema_uri** | Option<**String**> | URI to retrieve error schema | [optional]
**success_schema_flattened** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**error_schema_flattened** | Option<[**serde_json::Value**](.md)> | JSON schema that defines the body of response when request is not successful. The schema is transformed based on Architect's flattened format. If the 'flatten' query parameter is supplied as true, this field will be returned. Either errorSchema or errorSchemaFlattened will be returned, not both. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


