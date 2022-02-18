# VoicemailOrganizationPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled** | Option<**bool**> | Whether voicemail is enabled for this organization | [optional][readonly]
**alert_timeout_seconds** | Option<**i32**> | The organization's default number of seconds to ring a user's phone before a call is transferred to voicemail | [optional]
**pin_configuration** | Option<[**crate::models::PinConfiguration**](PINConfiguration.md)> |  | [optional]
**voicemail_extension** | Option<**String**> | The extension for voicemail retrieval.  The default value is *86. | [optional]
**pin_required** | Option<**bool**> | If this is true, a PIN is required when accessing a user's voicemail from a phone. | [optional]
**interactive_response_required** | Option<**bool**> | Whether user should be prompted with a confirmation prompt when connecting to a Group Ring call | [optional]
**send_email_notifications** | Option<**bool**> | Whether email notifications are sent for new voicemails in the organization. If false, new voicemail email notifications are not be sent for the organization overriding any user or group setting. | [optional]
**disable_email_pii** | Option<**bool**> | Removes any PII from emails. This overrides any analogous group configuration value. This is always true if HIPAA is enabled or unknown for an organization. | [optional]
**modified_date** | Option<**String**> | The date the policy was last modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


