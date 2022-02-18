# WidgetDeployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> | A human-readable description of this Deployment. | [optional]
**authentication_required** | Option<**bool**> | When true, the customer members starting a chat must be authenticated by supplying their JWT to the create operation. | [optional]
**disabled** | Option<**bool**> | When true, all create chat operations using this Deployment will be rejected. | [optional]
**flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**allowed_domains** | Option<**Vec<String>**> | The list of domains that are approved to use this Deployment; the list will be added to CORS headers for ease of web use. | [optional]
**client_type** | Option<**String**> | The type of display widget for which this Deployment is configured, which controls the administrator settings shown. | [optional]
**client_config** | Option<[**crate::models::WidgetClientConfig**](WidgetClientConfig.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


