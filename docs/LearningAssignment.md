# LearningAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**assessment** | Option<[**crate::models::LearningAssessment**](LearningAssessment.md)> |  | [optional]
**created_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_created** | Option<**String**> | The date when the assignment was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**modified_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_modified** | Option<**String**> | The date when the assignment was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**is_overdue** | Option<**bool**> | True if the assignment is overdue | [optional][readonly]
**percentage_score** | Option<**f32**> | The user's percentage score for this assignment | [optional][readonly]
**is_rule** | Option<**bool**> | True if this assignment was created by a Rule | [optional][readonly]
**is_manual** | Option<**bool**> | True if this assignment was created manually | [optional][readonly]
**is_passed** | Option<**bool**> | True if the assessment was passed | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**state** | Option<**String**> | The Learning Assignment state | [optional]
**date_recommended_for_completion** | Option<**String**> | The recommended completion date of the assignment. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**version** | Option<**i32**> | The version of Learning module assigned | [optional]
**module** | Option<[**crate::models::LearningModule**](LearningModule.md)> |  | [optional]
**user** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**assessment_form** | Option<[**crate::models::AssessmentForm**](AssessmentForm.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


