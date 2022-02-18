# AssessmentForm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**date_modified** | Option<**String**> | Last modified date of the assessment form. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**context_id** | Option<**String**> | The unique Id for all versions of this assessment form | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**published** | Option<**bool**> | If true, assessment form is published | [optional][readonly]
**pass_percent** | **i32** | The pass percent for the assessment form | 
**question_groups** | [**Vec<crate::models::AssessmentFormQuestionGroup>**](AssessmentFormQuestionGroup.md) | A list of question groups | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


