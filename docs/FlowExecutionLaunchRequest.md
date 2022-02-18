# FlowExecutionLaunchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**flow_id** | **String** | ID of the flow to launch. | 
**flow_version** | Option<**String**> | The version of the flow to launch. Omit this value (or supply null/empty) to use the latest published version. | [optional]
**input_data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Input values to the flow. Valid values are defined by a flow's input JSON schema. | [optional]
**name** | Option<**String**> | A displayable name to assign to the new flow execution | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


