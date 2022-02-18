# MessageMediaData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**url** | Option<**String**> | The location of the media, useful for retrieving it | [optional]
**media_type** | Option<**String**> | The detected internet media type of the the media object.  If null then the media type should be dictated by the url. | [optional]
**content_length_bytes** | Option<**i32**> | The optional content length of the the media object, in bytes. | [optional]
**upload_url** | Option<**String**> | The URL returned to upload an attachment | [optional]
**status** | Option<**String**> | The status of the media, indicates if the media is in the process of uploading. If the upload fails, the media becomes invalid | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


