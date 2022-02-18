# VoicemailGroupPolicy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**group** | Option<[**crate::models::Group**](Group.md)> |  | [optional]
**enabled** | Option<**bool**> | Whether voicemail is enabled for the group | [optional]
**send_email_notifications** | Option<**bool**> | Whether email notifications are sent to group members when a new voicemail is received | [optional]
**disable_email_pii** | Option<**bool**> | Removes any PII from group emails. This is overridden by the analogous organization configuration value. This is always true if HIPAA is enabled or unknown for an organization. | [optional]
**rotate_calls_secs** | Option<**i32**> | How many seconds to ring before rotating to the next member in the group | [optional]
**stop_ringing_after_rotations** | Option<**i32**> | How many rotations to go through | [optional]
**overflow_group_id** | Option<**String**> | A fallback group to contact when all of the members in this group did not answer the call. | [optional]
**group_alert_type** | Option<**String**> | Specifies if the members in this group should be contacted randomly, in a specific order, or by round-robin. | [optional]
**interactive_response_prompt_id** | Option<**String**> | The prompt to use when connecting a user to a Group Ring call | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


