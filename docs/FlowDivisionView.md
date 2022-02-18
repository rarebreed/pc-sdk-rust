# FlowDivisionView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The flow identifier | [optional]
**name** | **String** | The flow name | 
**division** | Option<[**crate::models::WritableDivision**](WritableDivision.md)> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**description** | Option<**String**> | the flow description | [optional]
**input_schema** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**output_schema** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**supported_languages** | Option<[**Vec<crate::models::SupportedLanguage>**](SupportedLanguage.md)> | List of supported languages for the published version of the flow. | [optional][readonly]
**published_version** | Option<[**crate::models::FlowVersion**](FlowVersion.md)> |  | [optional]
**debug_version** | Option<[**crate::models::FlowVersion**](FlowVersion.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


