# Integration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> | The name of the integration, used to distinguish this integration from others of the same type. | [optional][readonly]
**integration_type** | Option<[**crate::models::IntegrationType**](IntegrationType.md)> |  | [optional]
**notes** | Option<**String**> | Notes about the integration. | [optional][readonly]
**intended_state** | **String** | Configured state of the integration. | 
**config** | Option<[**crate::models::IntegrationConfigurationInfo**](IntegrationConfigurationInfo.md)> |  | [optional]
**reported_state** | Option<[**crate::models::IntegrationStatusInfo**](IntegrationStatusInfo.md)> |  | [optional]
**attributes** | Option<**::std::collections::HashMap<String, String>**> | Read-only attributes for the integration. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


