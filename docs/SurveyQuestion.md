# SurveyQuestion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**text** | Option<**String**> |  | [optional]
**help_text** | Option<**String**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**na_enabled** | Option<**bool**> |  | [optional]
**visibility_condition** | Option<[**crate::models::VisibilityCondition**](VisibilityCondition.md)> |  | [optional]
**answer_options** | Option<[**Vec<crate::models::AnswerOption>**](AnswerOption.md)> | Options from which to choose an answer for this question. Only used by Multiple Choice type questions. | [optional]
**max_response_characters** | Option<**i32**> | How many characters are allowed in the text response to this question. Used by NPS and Free Text question types. | [optional]
**explanation_prompt** | Option<**String**> | Prompt for details explaining the chosen NPS score. Used by NPS questions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


