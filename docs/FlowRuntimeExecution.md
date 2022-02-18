# FlowRuntimeExecution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The flow execution ID | [optional]
**name** | Option<**String**> | The flow execution name. | [optional]
**flow_version** | [**crate::models::FlowVersion**](FlowVersion.md) |  | 
**date_launched** | **String** | The time the flow was launched. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | 
**status** | **String** | The flow's running status, which indicates whether the flow is running normally or completed, etc. | 
**date_completed** | Option<**String**> | The time the flow completed, if applicable. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**completion_reason** | Option<**String**> | The completion reason set at the flow completion time, if applicable. | [optional]
**flow_error_info** | Option<[**crate::models::ErrorBody**](ErrorBody.md)> |  | [optional]
**output_data** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | List of the flow's output variables, if any. Output variables are only supplied for Completed flows. | [optional]
**conversation** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


