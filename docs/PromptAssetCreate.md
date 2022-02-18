# PromptAssetCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**prompt_id** | Option<**String**> | Associated prompt ID | [optional][readonly]
**language** | **String** | The prompt language. | 
**media_uri** | Option<**String**> | URI of the resource audio | [optional][readonly]
**tts_string** | Option<**String**> | Text to speech of the resource | [optional]
**text** | Option<**String**> | Text of the resource | [optional]
**upload_status** | Option<**String**> | Audio upload status | [optional][readonly]
**upload_uri** | Option<**String**> | Upload URI for the resource audio | [optional][readonly]
**language_default** | Option<**bool**> | Whether or not this resource locale is the default for the language | [optional][readonly]
**tags** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> |  | [optional]
**duration_seconds** | Option<**f64**> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


