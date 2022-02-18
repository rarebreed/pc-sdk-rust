# AssessmentFormQuestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**text** | **String** | The question text | 
**help_text** | Option<**String**> |  | [optional]
**na_enabled** | Option<**bool**> |  | [optional]
**comments_required** | Option<**bool**> |  | [optional]
**visibility_condition** | Option<[**crate::models::VisibilityCondition**](VisibilityCondition.md)> |  | [optional]
**answer_options** | Option<[**Vec<crate::models::AnswerOption>**](AnswerOption.md)> | Options from which to choose an answer for this question. Only used by Multiple Choice type questions. | [optional]
**max_response_characters** | Option<**i32**> | How many characters are allowed in the text response to this question. Used by Free Text question types. | [optional]
**is_kill** | Option<**bool**> | Does an incorrect answer to this question mark the form as having a failed kill question. Only used by Multiple Choice type questions. | [optional]
**is_critical** | Option<**bool**> | Does this question contribute to the critical score. Only used by Multiple Choice type questions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


