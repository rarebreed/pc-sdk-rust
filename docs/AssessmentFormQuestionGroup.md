# AssessmentFormQuestionGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the question group, | [optional]
**name** | **String** | The question group name | 
**_type** | **String** | The question group type | 
**default_answers_to_highest** | Option<**bool**> |  | [optional]
**default_answers_to_na** | Option<**bool**> |  | [optional]
**na_enabled** | Option<**bool**> |  | [optional]
**weight** | Option<**f32**> |  | [optional]
**manual_weight** | Option<**bool**> |  | [optional]
**questions** | [**Vec<crate::models::AssessmentFormQuestion>**](AssessmentFormQuestion.md) | The list of questions for this question group | 
**visibility_condition** | Option<[**crate::models::VisibilityCondition**](VisibilityCondition.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


