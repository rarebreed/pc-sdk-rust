# NluFeedbackResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**text** | Option<**String**> | The feedback text. | [optional]
**intents** | Option<[**Vec<crate::models::IntentFeedback>**](IntentFeedback.md)> | Detected intent of the utterance | [optional]
**version** | Option<[**crate::models::NluDomainVersion**](NluDomainVersion.md)> |  | [optional]
**date_created** | Option<**String**> | The date when the feedback was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


