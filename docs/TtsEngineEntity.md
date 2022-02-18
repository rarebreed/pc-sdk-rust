# TtsEngineEntity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**languages** | **Vec<String>** | The set of languages the TTS engine supports | 
**output_formats** | **Vec<String>** | The set of output formats the TTS engine can produce | 
**voices** | Option<[**Vec<crate::models::TtsVoiceEntity>**](TtsVoiceEntity.md)> | The set of voices the TTS engine supports | [optional]
**is_default** | Option<**bool**> | The TTS engine is the global default engine | [optional]
**is_secure** | Option<**bool**> | The TTS engine can be used in a secure call flow | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


