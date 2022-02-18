# FlowVersion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The flow version identifier | [optional]
**name** | Option<**String**> |  | [optional]
**commit_version** | Option<**String**> |  | [optional]
**configuration_version** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**secure** | Option<**bool**> |  | [optional]
**debug** | Option<**bool**> |  | [optional]
**created_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**created_by_client** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**configuration_uri** | Option<**String**> |  | [optional]
**date_created** | Option<**i64**> |  | [optional]
**generation_id** | Option<**String**> |  | [optional]
**publish_result_uri** | Option<**String**> |  | [optional]
**input_schema** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**output_schema** | Option<[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)> |  | [optional]
**nlu_info** | Option<[**crate::models::NluInfo**](NluInfo.md)> |  | [optional]
**supported_languages** | Option<[**Vec<crate::models::SupportedLanguage>**](SupportedLanguage.md)> | List of supported languages for this version of the flow | [optional][readonly]
**compatible_flow_types** | Option<**Vec<String>**> | Compatible flow types designate which flow types are allowed to embed a flow’s configuration within their own flow configuration.  Currently the only flows that can be embedded are Common Module flows and the embedding flow can invoke them using the Call Common Module action. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


