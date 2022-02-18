# AssessmentScoringSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_score** | Option<**f32**> | The total score of the answers | [optional][readonly]
**total_critical_score** | Option<**f32**> | The total score for the critical questions | [optional][readonly]
**total_non_critical_score** | Option<**f32**> | The total score for the non-critical questions | [optional][readonly]
**question_group_scores** | [**Vec<crate::models::AssessmentQuestionGroupScore>**](AssessmentQuestionGroupScore.md) | The individual scores for each question group | 
**failure_reasons** | Option<**Vec<String>**> | If the assessment was not passed, the reasons for failure. | [optional][readonly]
**comments** | Option<**String**> | Comments provided for these answers. | [optional]
**agent_comments** | Option<**String**> | Comments provided by agent. | [optional]
**is_passed** | Option<**bool**> | True if the assessment was passed | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


