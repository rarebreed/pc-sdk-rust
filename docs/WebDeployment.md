# WebDeployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The deployment ID | [optional][readonly]
**name** | **String** | The deployment name | 
**description** | Option<**String**> | The description of the config | [optional]
**configuration** | [**crate::models::WebDeploymentConfigurationVersionEntityRef**](WebDeploymentConfigurationVersionEntityRef.md) |  | 
**allow_all_domains** | Option<**bool**> | Property indicates whether all domains are allowed or not. allowedDomains must be empty when this is set as true. | [optional]
**allowed_domains** | Option<**Vec<String>**> | The list of domains that are approved to use this deployment; the list will be added to CORS headers for ease of web use. | [optional]
**snippet** | Option<**String**> | Javascript snippet used to load the config | [optional][readonly]
**date_created** | Option<**String**> | The date the deployment was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**date_modified** | Option<**String**> | The date the deployment was most recently modified. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional][readonly]
**last_modified_user** | Option<[**crate::models::AddressableEntityRef**](AddressableEntityRef.md)> |  | [optional]
**flow** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**status** | Option<**String**> | The current status of the deployment | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


