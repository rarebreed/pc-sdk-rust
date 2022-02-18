# IntegrationType

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the integration type. | 
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> | Description of the integration type. | [optional][readonly]
**provider** | Option<**String**> | PureCloud provider of the integration type. | [optional][readonly]
**category** | Option<**String**> | Category describing the integration type. | [optional][readonly]
**images** | Option<[**Vec<crate::models::UserImage>**](UserImage.md)> | Collection of logos. | [optional][readonly]
**config_properties_schema_uri** | Option<**String**> | URI of the schema describing the key-value properties needed to configure an integration of this type. | [optional][readonly]
**config_advanced_schema_uri** | Option<**String**> | URI of the schema describing the advanced JSON document needed to configure an integration of this type. | [optional][readonly]
**help_uri** | Option<**String**> | URI of a page with more information about the integration type | [optional][readonly]
**terms_of_service_uri** | Option<**String**> | URI of a page with terms and conditions for the integration type | [optional][readonly]
**vendor_name** | Option<**String**> | Name of the vendor of this integration type | [optional][readonly]
**vendor_website_uri** | Option<**String**> | URI of the vendor's website | [optional][readonly]
**marketplace_uri** | Option<**String**> | URI of the marketplace listing for this integration type | [optional][readonly]
**faq_uri** | Option<**String**> | URI of frequently asked questions about the integration type | [optional][readonly]
**privacy_policy_uri** | Option<**String**> | URI of a privacy policy for users of the integration type | [optional][readonly]
**support_contact_uri** | Option<**String**> | URI for vendor support | [optional][readonly]
**sales_contact_uri** | Option<**String**> | URI for vendor sales information | [optional][readonly]
**help_links** | Option<[**Vec<crate::models::HelpLink>**](HelpLink.md)> | List of links to additional help resources | [optional][readonly]
**credentials** | Option<[**::std::collections::HashMap<String, crate::models::CredentialSpecification>**](CredentialSpecification.md)> | Map of credentials for integrations of this type. The key is the name of a credential that can be provided in the credentials property of the integration configuration. | [optional][readonly]
**non_installable** | Option<**bool**> | Indicates if the integration type is installable or not. | [optional][readonly]
**max_instances** | Option<**i32**> | The maximum number of integration instances allowable for this integration type | [optional][readonly]
**user_permissions** | Option<**Vec<String>**> | List of permissions required to permit user access to the integration type. | [optional][readonly]
**vendor_o_auth_client_ids** | Option<**Vec<String>**> | List of OAuth Client IDs that must be authorized when the integration is created. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


