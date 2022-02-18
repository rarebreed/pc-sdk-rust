# DraftTopics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Id for a topic. | 
**name** | Option<**String**> | Topic name. | [optional]
**miner** | Option<[**crate::models::Miner**](Miner.md)> |  | [optional]
**conversation_count** | Option<**i32**> | Number of conversations where a topic has occurred. | [optional][readonly]
**conversation_percent** | Option<**f32**> | Percentage of conversations where a topic has occurred. | [optional][readonly]
**utterance_count** | Option<**i32**> | Number of unique utterances where a topic has occurred. | [optional][readonly]
**phrase_count** | Option<**i32**> | Number of unique phrases (sub-utterances) where a topic has occurred. | [optional][readonly]
**phrases** | **Vec<String>** | The phrases that are extracted for a topic. | 
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


