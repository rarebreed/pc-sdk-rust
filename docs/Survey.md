# Survey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**conversation** | Option<[**crate::models::Conversation**](Conversation.md)> |  | [optional]
**survey_form** | Option<[**crate::models::SurveyForm**](SurveyForm.md)> |  | [optional]
**agent** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**queue** | Option<[**crate::models::QueueReference**](QueueReference.md)> |  | [optional]
**answers** | Option<[**crate::models::SurveyScoringSet**](SurveyScoringSet.md)> |  | [optional]
**completed_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**survey_error_details** | Option<[**crate::models::SurveyErrorDetails**](SurveyErrorDetails.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


