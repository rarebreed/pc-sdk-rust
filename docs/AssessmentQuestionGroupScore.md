# AssessmentQuestionGroupScore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**question_group_id** | **String** | The ID of the question group | 
**total_score** | Option<**f32**> | The total score for the questions | [optional][readonly]
**max_total_score** | Option<**f32**> | The maximum total score for the questions | [optional][readonly]
**marked_na** | Option<**bool**> | True if this question group is marked NA | [optional]
**total_critical_score** | Option<**f32**> | The total score for the critical questions | [optional][readonly]
**max_total_critical_score** | Option<**f32**> | The maximum total score for the critical questions | [optional][readonly]
**total_non_critical_score** | Option<**f32**> | The total score for the non-critical questions | [optional][readonly]
**max_total_non_critical_score** | Option<**f32**> | The maximum total score for the non-critical questions | [optional][readonly]
**total_score_unweighted** | Option<**f32**> | The unweighted total score for this question group | [optional][readonly]
**max_total_score_unweighted** | Option<**f32**> | The maximum unweighted total score for this question group | [optional][readonly]
**total_critical_score_unweighted** | Option<**f32**> | The unweighted total score for the critical questions | [optional][readonly]
**max_total_critical_score_unweighted** | Option<**f32**> | The maximum unweighted total score for the critical questions | [optional][readonly]
**total_non_critical_score_unweighted** | Option<**f32**> | The total unweighted score for the non-critical questions | [optional][readonly]
**max_total_non_critical_score_unweighted** | Option<**f32**> | The maximum unweighted total score for the non-critical questions | [optional][readonly]
**question_scores** | Option<[**Vec<crate::models::AssessmentQuestionScore>**](AssessmentQuestionScore.md)> | The individual question scores | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


