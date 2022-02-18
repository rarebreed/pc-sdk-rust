# SurveyForm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The survey form name | 
**modified_date** | Option<**String**> | Last modified date. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**published** | Option<**bool**> | Is this form published | [optional][readonly]
**disabled** | Option<**bool**> | Is this form disabled | [optional]
**context_id** | **String** | Unique Id for all versions of this form | [readonly]
**language** | **String** | Language for survey viewer localization. Currently localized languages: da, de, en-US, es, fi, fr, it, ja, ko, nl, no, pl, pt-BR, sv, th, tr, zh-CH, zh-TW | 
**header** | Option<**String**> | Markdown text for the top of the form. | [optional]
**footer** | Option<**String**> | Markdown text for the bottom of the form. | [optional]
**question_groups** | [**Vec<crate::models::SurveyQuestionGroup>**](SurveyQuestionGroup.md) | A list of question groups | 
**published_versions** | Option<[**crate::models::DomainEntityListingSurveyForm**](DomainEntityListingSurveyForm.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


