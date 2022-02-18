# EvaluationForm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The evaluation form name | 
**modified_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**published** | Option<**bool**> |  | [optional]
**context_id** | Option<**String**> |  | [optional]
**question_groups** | [**Vec<crate::models::EvaluationQuestionGroup>**](EvaluationQuestionGroup.md) | A list of question groups | 
**published_versions** | Option<[**crate::models::DomainEntityListingEvaluationForm**](DomainEntityListingEvaluationForm.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


