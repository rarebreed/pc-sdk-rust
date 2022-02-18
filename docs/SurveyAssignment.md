# SurveyAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**survey_form** | Option<[**crate::models::PublishedSurveyFormReference**](PublishedSurveyFormReference.md)> |  | [optional]
**flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**invite_time_interval** | Option<**String**> | An ISO 8601 repeated interval consisting of the number of repetitions, the start datetime, and the interval (e.g. R2/2018-03-01T13:00:00Z/P1M10DT2H30M). Total duration must not exceed 90 days. | [optional]
**sending_user** | Option<**String**> | User together with sendingDomain used to send email, null to use no-reply | [optional]
**sending_domain** | **String** | Validated email domain, required | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


