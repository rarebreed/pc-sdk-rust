# EvaluationScoringSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_score** | Option<**f32**> | Score of all questions | [optional]
**total_critical_score** | Option<**f32**> | Score of only the critical questions | [optional]
**total_non_critical_score** | Option<**f32**> | Score of only the non-critical questions | [optional]
**question_group_scores** | Option<[**Vec<crate::models::EvaluationQuestionGroupScore>**](EvaluationQuestionGroupScore.md)> |  | [optional]
**any_failed_kill_questions** | Option<**bool**> | Indicates that at least one fatal question was answered without having the highest score available for the question | [optional]
**comments** | Option<**String**> | Overall comments from the evaluator | [optional]
**agent_comments** | Option<**String**> | Comments from the agent while reviewing evaluation results | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


