# AnalyticsFlow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ending_language** | Option<**String**> | Flow ending language, e.g. en-us | [optional]
**entry_reason** | Option<**String**> | The particular entry reason for this flow, e.g. an address, userId, or flowId | [optional]
**entry_type** | Option<**String**> | The entry type for this flow, e.g. dnis, dialer, agent, flow, or direct | [optional]
**exit_reason** | Option<**String**> | The exit reason for this flow, e.g. DISCONNECT | [optional]
**flow_id** | Option<**String**> | The unique identifier of this flow | [optional]
**flow_name** | Option<**String**> | The name of this flow at the time of flow execution | [optional]
**flow_type** | Option<**String**> | The type of this flow | [optional]
**flow_version** | Option<**String**> | The version of this flow | [optional]
**issued_callback** | Option<**bool**> | Flag indicating whether the flow issued a callback | [optional]
**recognition_failure_reason** | Option<**String**> | The recognition failure reason causing to exit/disconnect | [optional]
**starting_language** | Option<**String**> | Flow starting language, e.g. en-us | [optional]
**transfer_target_address** | Option<**String**> | The address of a flow transfer target, e.g. a phone number, an email address, or a queueId | [optional]
**transfer_target_name** | Option<**String**> | The name of a flow transfer target | [optional]
**transfer_type** | Option<**String**> | The type of transfer for flows that ended with a transfer | [optional]
**outcomes** | Option<[**Vec<crate::models::AnalyticsFlowOutcome>**](AnalyticsFlowOutcome.md)> | Flow outcomes | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


