# ScimServiceProviderConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**schemas** | Option<**Vec<String>**> | The list of supported schemas. | [optional][readonly]
**documentation_uri** | Option<**String**> | The HTTP-addressable URL that points to the service provider's documentation. | [optional][readonly]
**patch** | Option<[**crate::models::ScimServiceProviderConfigSimpleFeature**](ScimServiceProviderConfigSimpleFeature.md)> |  | [optional]
**filter** | Option<[**crate::models::ScimServiceProviderConfigFilterFeature**](ScimServiceProviderConfigFilterFeature.md)> |  | [optional]
**etag** | Option<[**crate::models::ScimServiceProviderConfigSimpleFeature**](ScimServiceProviderConfigSimpleFeature.md)> |  | [optional]
**sort** | Option<[**crate::models::ScimServiceProviderConfigSimpleFeature**](ScimServiceProviderConfigSimpleFeature.md)> |  | [optional]
**bulk** | Option<[**crate::models::ScimServiceProviderConfigBulkFeature**](ScimServiceProviderConfigBulkFeature.md)> |  | [optional]
**change_password** | Option<[**crate::models::ScimServiceProviderConfigSimpleFeature**](ScimServiceProviderConfigSimpleFeature.md)> |  | [optional]
**authentication_schemes** | Option<[**Vec<crate::models::ScimServiceProviderConfigAuthenticationScheme>**](ScimServiceProviderConfigAuthenticationScheme.md)> | The list of supported authentication schemes. | [optional][readonly]
**meta** | Option<[**crate::models::ScimMetadata**](ScimMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


