# WorkPlanConfigurationViolationMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_type** | Option<**String**> | Type of configuration violation message for this work plan | [optional]
**arguments** | Option<[**Vec<crate::models::WorkPlanValidationMessageArgument>**](WorkPlanValidationMessageArgument.md)> | Arguments of the message that provide information about the misconfigured value or the threshold that is exceeded by the misconfigured value | [optional]
**severity** | Option<**String**> | Severity of the message. A message with Error severity indicates the scheduler won't be able to produce schedules and thus the work plan is invalid. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


