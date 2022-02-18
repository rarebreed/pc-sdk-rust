# EffectiveConfiguration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**properties** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Key-value configuration settings described by the schema in the propertiesSchemaUri field. | 
**advanced** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Advanced configuration described by the schema in the advancedSchemaUri field. | 
**name** | **String** | The name of the integration, used to distinguish this integration from others of the same type. | 
**notes** | **String** | Notes about the integration. | 
**credentials** | [**::std::collections::HashMap<String, crate::models::CredentialInfo>**](CredentialInfo.md) | Credentials required by the integration. The required keys are indicated in the credentials property of the Integration Type | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


