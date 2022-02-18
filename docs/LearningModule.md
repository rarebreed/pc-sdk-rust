# LearningModule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of learning module | 
**created_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_created** | Option<**String**> | The date/time learning module was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**modified_by** | Option<[**crate::models::UserReference**](UserReference.md)> |  | [optional]
**date_modified** | Option<**String**> | The date/time learning module was modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**version** | Option<**i32**> | The version of published learning module | [optional][readonly]
**external_id** | Option<**String**> | The external ID of the learning module | [optional][readonly]
**source** | Option<**String**> | The source of the learning module | [optional][readonly]
**rule** | Option<[**crate::models::LearningModuleRule**](LearningModuleRule.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]
**is_archived** | Option<**bool**> | If true, learning module is archived | [optional][readonly]
**is_published** | Option<**bool**> | If true, learning module is published | [optional][readonly]
**description** | Option<**String**> | The description of learning module | [optional]
**completion_time_in_days** | **i32** | The completion time of learning module in days | 
**_type** | Option<**String**> | The type for the learning module | [optional]
**inform_steps** | Option<[**Vec<crate::models::LearningModuleInformStep>**](LearningModuleInformStep.md)> | The list of inform steps in a learning module | [optional]
**assessment_form** | Option<[**crate::models::AssessmentForm**](AssessmentForm.md)> |  | [optional]
**summary_data** | Option<[**crate::models::LearningModuleSummary**](LearningModuleSummary.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


