# Evaluation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**conversation** | Option<[**crate::models::Conversation**](Conversation.md)> |  | [optional]
**evaluation_form** | Option<[**crate::models::EvaluationForm**](EvaluationForm.md)> |  | [optional]
**evaluator** | Option<[**crate::models::User**](User.md)> |  | [optional]
**agent** | Option<[**crate::models::User**](User.md)> |  | [optional]
**calibration** | Option<[**crate::models::Calibration**](Calibration.md)> |  | [optional]
**status** | Option<**String**> |  | [optional]
**answers** | Option<[**crate::models::EvaluationScoringSet**](EvaluationScoringSet.md)> |  | [optional]
**agent_has_read** | Option<**bool**> |  | [optional]
**release_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**assigned_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**changed_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**queue** | Option<[**crate::models::Queue**](Queue.md)> |  | [optional]
**media_type** | Option<**Vec<String>**> | List of different communication types used in conversation. | [optional]
**rescore** | Option<**bool**> | Is only true when evaluation is re-scored. | [optional]
**conversation_date** | Option<**String**> | Date of conversation. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**conversation_end_date** | Option<**String**> | End date of conversation if it had completed before evaluation creation. Null if created before the conversation ended. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**never_release** | Option<**bool**> | Signifies if the evaluation is never to be released. This cannot be set true if release date is also set. | [optional]
**resource_id** | Option<**String**> | Only used for email evaluations. Will be null for all other evaluations. | [optional]
**resource_type** | Option<**String**> | The type of resource. Only used for email evaluations. Will be null for evaluations on all other resources. | [optional]
**redacted** | Option<**bool**> | Is only true when the user making the request does not have sufficient permissions to see evaluation | [optional]
**is_scoring_index** | Option<**bool**> |  | [optional]
**authorized_actions** | Option<**Vec<String>**> | List of user authorized actions on evaluation. Possible values: edit, editScore, editAgentSignoff, delete, viewAudit | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


