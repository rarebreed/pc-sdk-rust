# Flow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The flow identifier | [optional]
**name** | **String** | The flow name | 
**division** | Option<[**crate::models::WritableDivision**](WritableDivision.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**locked_user** | Option<[**crate::models::User**](User.md)> |  | [optional]
**locked_client** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**active** | Option<**bool**> |  | [optional]
**system** | Option<**bool**> |  | [optional]
**deleted** | Option<**bool**> |  | [optional]
**published_version** | Option<[**crate::models::FlowVersion**](FlowVersion.md)> |  | [optional]
**saved_version** | Option<[**crate::models::FlowVersion**](FlowVersion.md)> |  | [optional]
**input_schema** | Option<[**serde_json::Value**](.md)> | json schema describing the inputs for the flow | [optional]
**output_schema** | Option<[**serde_json::Value**](.md)> | json schema describing the outputs for the flow | [optional]
**checked_in_version** | Option<[**crate::models::FlowVersion**](FlowVersion.md)> |  | [optional]
**debug_version** | Option<[**crate::models::FlowVersion**](FlowVersion.md)> |  | [optional]
**published_by** | Option<[**crate::models::User**](User.md)> |  | [optional]
**current_operation** | Option<[**crate::models::Operation**](Operation.md)> |  | [optional]
**nlu_info** | Option<[**crate::models::NluInfo**](NluInfo.md)> |  | [optional]
**supported_languages** | Option<[**Vec<crate::models::SupportedLanguage>**](SupportedLanguage.md)> | List of supported languages for the published version of the flow. | [optional][readonly]
**compatible_flow_types** | Option<**Vec<String>**> | Compatible flow types designate which flow types are allowed to embed a flow’s configuration within their own flow configuration.  Currently the only flows that can be embedded are Common Module flows and the embedding flow can invoke them using the Call Common Module action. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


