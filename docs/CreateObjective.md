# CreateObjective

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**template_id** | Option<**String**> | The id of this objective's base template | [optional]
**zones** | Option<[**Vec<crate::models::ObjectiveZone>**](ObjectiveZone.md)> | Objective zone specifies min,max points and values for the associated metric | [optional]
**enabled** | Option<**bool**> | A flag for whether this objective is enabled for the related metric | [optional]
**topic_ids** | Option<**Vec<String>**> | A list of topic ids for detected topic metrics | [optional]
**media_types** | Option<**Vec<String>**> | A list of media types for the metric | [optional]
**queue_ids** | Option<**Vec<String>**> | A list of queue ids for the metric | [optional]
**topic_ids_filter_type** | Option<**String**> | A filter type for topic Ids. It's only used for objectives with topicIds. Default filter behavior is \"or\". | [optional]
**date_start** | Option<[**String**](string.md)> | start date of the objective. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


