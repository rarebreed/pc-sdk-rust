# TrunkMetricsRegisters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**proxy_address** | Option<**String**> | Server proxy address that this registers array element represents. | [optional]
**register_state** | Option<**bool**> | True if last REGISTER message had positive response; false if error response or no response. | [optional]
**register_state_time** | Option<**String**> | ISO 8601 format UTC absolute date & time of the last change of the register state. | [optional]
**error_info** | Option<[**crate::models::TrunkErrorInfo**](TrunkErrorInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


