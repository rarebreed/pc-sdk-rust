# IntegrationConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the integration, used to distinguish this integration from others of the same type. | 
**version** | **i32** | Version number required for updates. | 
**properties** | [**serde_json::Value**](.md) | Key-value configuration settings described by the schema in the propertiesSchemaUri field. | 
**advanced** | [**serde_json::Value**](.md) | Advanced configuration described by the schema in the advancedSchemaUri field. | 
**notes** | **String** | Notes about the integration. | 
**credentials** | [**::std::collections::HashMap<String, crate::models::CredentialInfo>**](CredentialInfo.md) | Credentials required by the integration. The required keys are indicated in the credentials property of the Integration Type | 
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


