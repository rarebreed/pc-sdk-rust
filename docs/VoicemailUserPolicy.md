# VoicemailUserPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether the user has voicemail enabled | [optional][readonly]
**alert_timeout_seconds** | Option<**i32**> | The number of seconds to ring the user's phone before a call is transfered to voicemail | [optional]
**pin** | Option<**String**> | The user's PIN to access their voicemail. This property is only used for updates and never provided otherwise to ensure security | [optional]
**modified_date** | Option<**String**> | The date the policy was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**send_email_notifications** | Option<**bool**> | Whether email notifications are sent to the user when a new voicemail is received | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


