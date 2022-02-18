# PolicyActions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**retain_recording** | Option<**bool**> | true to retain the recording associated with the conversation. Default = true | [optional]
**delete_recording** | Option<**bool**> | true to delete the recording associated with the conversation. If retainRecording = true, this will be ignored. Default = false | [optional]
**always_delete** | Option<**bool**> | true to delete the recording associated with the conversation regardless of the values of retainRecording or deleteRecording. Default = false | [optional]
**assign_evaluations** | Option<[**Vec<crate::models::EvaluationAssignment>**](EvaluationAssignment.md)> |  | [optional]
**assign_metered_evaluations** | Option<[**Vec<crate::models::MeteredEvaluationAssignment>**](MeteredEvaluationAssignment.md)> |  | [optional]
**assign_metered_assignment_by_agent** | Option<[**Vec<crate::models::MeteredAssignmentByAgent>**](MeteredAssignmentByAgent.md)> |  | [optional]
**assign_calibrations** | Option<[**Vec<crate::models::CalibrationAssignment>**](CalibrationAssignment.md)> |  | [optional]
**assign_surveys** | Option<[**Vec<crate::models::SurveyAssignment>**](SurveyAssignment.md)> |  | [optional]
**retention_duration** | Option<[**crate::models::RetentionDuration**](RetentionDuration.md)> |  | [optional]
**initiate_screen_recording** | Option<[**crate::models::InitiateScreenRecording**](InitiateScreenRecording.md)> |  | [optional]
**media_transcriptions** | Option<[**Vec<crate::models::MediaTranscription>**](MediaTranscription.md)> |  | [optional]
**integration_export** | Option<[**crate::models::IntegrationExport**](IntegrationExport.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


