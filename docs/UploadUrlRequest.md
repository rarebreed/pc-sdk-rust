# UploadUrlRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file_name** | Option<**String**> | Name of the file to upload. It must not start with a dot and not end with a forward slash. Whitespace and the following characters are not allowed: \\{^}%`]\">[~<#| | [optional]
**content_md5** | Option<**String**> | Content MD-5 of the file to upload | [optional]
**signed_url_timeout_seconds** | Option<**i32**> | The number of seconds the presigned URL is valid for (from 1 to 604800 seconds). If none provided, defaults to 600 seconds | [optional]
**server_side_encryption** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


