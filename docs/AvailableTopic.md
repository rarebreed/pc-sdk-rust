# AvailableTopic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**permission_details** | Option<[**Vec<crate::models::PermissionDetails>**](PermissionDetails.md)> | Full detailed permissions required to subscribe to the topic | [optional]
**requires_permissions** | Option<**Vec<String>**> | Permissions required to subscribe to the topic | [optional]
**requires_division_permissions** | Option<**bool**> | True if the subscribing user must belong to the same division as the topic object ID | [optional]
**requires_any_validator** | Option<**bool**> | If multiple permissions are required for this topic, such as both requiresCurrentUser and requiresDivisionPermissions, then true here indicates that meeting any one condition will satisfy the requirements; false indicates all conditions must be met. | [optional]
**enforced** | Option<**bool**> | Whether or not the permissions on this topic are enforced | [optional]
**visibility** | Option<**String**> | Visibility of this topic (Public or Preview) | [optional]
**schema** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**requires_current_user** | Option<**bool**> | True if the topic user ID is required to match the subscribing user ID | [optional]
**requires_current_user_or_permission** | Option<**bool**> | True if permissions are only required when the topic user ID does not match the subscribing user ID | [optional]
**transports** | Option<**Vec<String>**> | Transports that support events for the topic | [optional]
**public_api_template_uri_paths** | Option<**Vec<String>**> |  | [optional]
**topic_parameters** | Option<**Vec<String>**> | Parameters in the topic name that can be substituted, in the order they appear in the topic name | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


