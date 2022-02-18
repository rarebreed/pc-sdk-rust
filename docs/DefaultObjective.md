# DefaultObjective

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**template_id** | Option<**String**> | The id of this objective's base template | [optional]
**zones** | Option<[**Vec<crate::models::ObjectiveZone>**](ObjectiveZone.md)> | Objective zone specifies min,max points and values for the associated metric | [optional]
**enabled** | Option<**bool**> | A flag for whether this objective is enabled for the related metric | [optional]
**media_types** | Option<**Vec<String>**> | A list of media types for the metric | [optional]
**queues** | Option<[**Vec<crate::models::AddressableEntityRef>**](AddressableEntityRef.md)> | A list of queues for the metric | [optional]
**topics** | Option<[**Vec<crate::models::AddressableEntityRef>**](AddressableEntityRef.md)> | A list of topic ids for detected topic metrics | [optional]
**topic_ids_filter_type** | Option<**String**> | A filter type for topic Ids. It's only used for objectives with topicIds. Default filter behavior is \"or\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


