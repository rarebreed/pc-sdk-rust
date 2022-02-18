# SentimentFeedback

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**phrase** | **String** | The phrase for which sentiment feedback is provided | 
**dialect** | **String** | The dialect for the given phrase, dialect format is {language}-{country} where language follows ISO 639-1 standard and country follows ISO 3166-1 alpha 2 standard | 
**feedback_value** | **String** | The sentiment feedback value for the given phrase | 
**date_created** | Option<**String**> | The Timestamp when sentiment feedback created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**created_by** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


