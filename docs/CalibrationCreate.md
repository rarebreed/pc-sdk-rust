# CalibrationCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**calibrator** | Option<[**crate::models::User**](User.md)> |  | [optional]
**agent** | Option<[**crate::models::User**](User.md)> |  | [optional]
**conversation** | [**crate::models::Conversation**](Conversation.md) |  | 
**evaluation_form** | Option<[**crate::models::EvaluationForm**](EvaluationForm.md)> |  | [optional]
**context_id** | Option<**String**> |  | [optional]
**average_score** | Option<**i32**> |  | [optional]
**high_score** | Option<**i32**> |  | [optional]
**low_score** | Option<**i32**> |  | [optional]
**created_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**evaluations** | Option<[**Vec<crate::models::Evaluation>**](Evaluation.md)> |  | [optional]
**evaluators** | Option<[**Vec<crate::models::User>**](User.md)> |  | [optional]
**scoring_index** | Option<[**crate::models::Evaluation**](Evaluation.md)> |  | [optional]
**expert_evaluator** | Option<[**crate::models::User**](User.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


