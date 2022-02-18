# \WebDeploymentsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_webdeployments_configuration**](WebDeploymentsApi.md#delete_webdeployments_configuration) | **DELETE** /api/v2/webdeployments/configurations/{configurationId} | Delete all versions of a configuration
[**delete_webdeployments_deployment**](WebDeploymentsApi.md#delete_webdeployments_deployment) | **DELETE** /api/v2/webdeployments/deployments/{deploymentId} | Delete a deployment
[**get_webdeployments_configuration_version**](WebDeploymentsApi.md#get_webdeployments_configuration_version) | **GET** /api/v2/webdeployments/configurations/{configurationId}/versions/{versionId} | Get a configuration version
[**get_webdeployments_configuration_versions**](WebDeploymentsApi.md#get_webdeployments_configuration_versions) | **GET** /api/v2/webdeployments/configurations/{configurationId}/versions | Get the versions of a configuration
[**get_webdeployments_configuration_versions_draft**](WebDeploymentsApi.md#get_webdeployments_configuration_versions_draft) | **GET** /api/v2/webdeployments/configurations/{configurationId}/versions/draft | Get the configuration draft
[**get_webdeployments_configurations**](WebDeploymentsApi.md#get_webdeployments_configurations) | **GET** /api/v2/webdeployments/configurations | View configuration drafts
[**get_webdeployments_deployment**](WebDeploymentsApi.md#get_webdeployments_deployment) | **GET** /api/v2/webdeployments/deployments/{deploymentId} | Get a deployment
[**get_webdeployments_deployments**](WebDeploymentsApi.md#get_webdeployments_deployments) | **GET** /api/v2/webdeployments/deployments | Get deployments
[**post_webdeployments_configuration_versions_draft_publish**](WebDeploymentsApi.md#post_webdeployments_configuration_versions_draft_publish) | **POST** /api/v2/webdeployments/configurations/{configurationId}/versions/draft/publish | Publish the configuration draft and create a new version
[**post_webdeployments_configurations**](WebDeploymentsApi.md#post_webdeployments_configurations) | **POST** /api/v2/webdeployments/configurations | Create a configuration draft
[**post_webdeployments_deployments**](WebDeploymentsApi.md#post_webdeployments_deployments) | **POST** /api/v2/webdeployments/deployments | Create a deployment
[**put_webdeployments_configuration_versions_draft**](WebDeploymentsApi.md#put_webdeployments_configuration_versions_draft) | **PUT** /api/v2/webdeployments/configurations/{configurationId}/versions/draft | Update the configuration draft
[**put_webdeployments_deployment**](WebDeploymentsApi.md#put_webdeployments_deployment) | **PUT** /api/v2/webdeployments/deployments/{deploymentId} | Update a deployment



## delete_webdeployments_configuration

> delete_webdeployments_configuration(configuration_id)
Delete all versions of a configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | The configuration version ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webdeployments_deployment

> delete_webdeployments_deployment(deployment_id)
Delete a deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | The deployment ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webdeployments_configuration_version

> crate::models::WebDeploymentConfigurationVersion get_webdeployments_configuration_version(configuration_id, version_id)
Get a configuration version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | The configuration version ID | [required] |
**version_id** | **String** | The version of the configuration to get | [required] |

### Return type

[**crate::models::WebDeploymentConfigurationVersion**](WebDeploymentConfigurationVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webdeployments_configuration_versions

> crate::models::WebDeploymentConfigurationVersionEntityListing get_webdeployments_configuration_versions(configuration_id)
Get the versions of a configuration

This returns the 50 most recent versions for this configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | The configuration version ID | [required] |

### Return type

[**crate::models::WebDeploymentConfigurationVersionEntityListing**](WebDeploymentConfigurationVersionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webdeployments_configuration_versions_draft

> crate::models::WebDeploymentConfigurationVersion get_webdeployments_configuration_versions_draft(configuration_id)
Get the configuration draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | The configuration version ID | [required] |

### Return type

[**crate::models::WebDeploymentConfigurationVersion**](WebDeploymentConfigurationVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webdeployments_configurations

> crate::models::WebDeploymentConfigurationVersionEntityListing get_webdeployments_configurations(show_only_published)
View configuration drafts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_only_published** | Option<**bool**> | Get only configuration drafts with published versions |  |[default to false]

### Return type

[**crate::models::WebDeploymentConfigurationVersionEntityListing**](WebDeploymentConfigurationVersionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webdeployments_deployment

> crate::models::WebDeployment get_webdeployments_deployment(deployment_id)
Get a deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | The deployment ID | [required] |

### Return type

[**crate::models::WebDeployment**](WebDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webdeployments_deployments

> crate::models::WebDeploymentEntityListing get_webdeployments_deployments()
Get deployments

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WebDeploymentEntityListing**](WebDeploymentEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webdeployments_configuration_versions_draft_publish

> crate::models::WebDeploymentConfigurationVersion post_webdeployments_configuration_versions_draft_publish(configuration_id)
Publish the configuration draft and create a new version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | The configuration version ID | [required] |

### Return type

[**crate::models::WebDeploymentConfigurationVersion**](WebDeploymentConfigurationVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webdeployments_configurations

> crate::models::WebDeploymentConfigurationVersion post_webdeployments_configurations(configuration_version)
Create a configuration draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_version** | [**WebDeploymentConfigurationVersion**](WebDeploymentConfigurationVersion.md) |  | [required] |

### Return type

[**crate::models::WebDeploymentConfigurationVersion**](WebDeploymentConfigurationVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_webdeployments_deployments

> crate::models::WebDeployment post_webdeployments_deployments(deployment)
Create a deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment** | [**WebDeployment**](WebDeployment.md) |  | [required] |

### Return type

[**crate::models::WebDeployment**](WebDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_webdeployments_configuration_versions_draft

> crate::models::WebDeploymentConfigurationVersion put_webdeployments_configuration_versions_draft(configuration_id, configuration_version)
Update the configuration draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**configuration_id** | **String** | The configuration version ID | [required] |
**configuration_version** | [**WebDeploymentConfigurationVersion**](WebDeploymentConfigurationVersion.md) |  | [required] |

### Return type

[**crate::models::WebDeploymentConfigurationVersion**](WebDeploymentConfigurationVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_webdeployments_deployment

> crate::models::WebDeployment put_webdeployments_deployment(deployment_id, deployment)
Update a deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | The deployment ID | [required] |
**deployment** | [**WebDeployment**](WebDeployment.md) |  | [required] |

### Return type

[**crate::models::WebDeployment**](WebDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

