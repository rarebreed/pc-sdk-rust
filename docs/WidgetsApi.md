# \WidgetsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_widgets_deployment**](WidgetsApi.md#delete_widgets_deployment) | **DELETE** /api/v2/widgets/deployments/{deploymentId} | Delete a Widget deployment
[**get_widgets_deployment**](WidgetsApi.md#get_widgets_deployment) | **GET** /api/v2/widgets/deployments/{deploymentId} | Get a Widget deployment
[**get_widgets_deployments**](WidgetsApi.md#get_widgets_deployments) | **GET** /api/v2/widgets/deployments | List Widget deployments
[**post_widgets_deployments**](WidgetsApi.md#post_widgets_deployments) | **POST** /api/v2/widgets/deployments | Create Widget deployment
[**put_widgets_deployment**](WidgetsApi.md#put_widgets_deployment) | **PUT** /api/v2/widgets/deployments/{deploymentId} | Update a Widget deployment



## delete_widgets_deployment

> delete_widgets_deployment(deployment_id)
Delete a Widget deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Widget Config Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_widgets_deployment

> crate::models::WidgetDeployment get_widgets_deployment(deployment_id)
Get a Widget deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Widget Config Id | [required] |

### Return type

[**crate::models::WidgetDeployment**](WidgetDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_widgets_deployments

> crate::models::WidgetDeploymentEntityListing get_widgets_deployments()
List Widget deployments

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WidgetDeploymentEntityListing**](WidgetDeploymentEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_widgets_deployments

> crate::models::WidgetDeployment post_widgets_deployments(body)
Create Widget deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WidgetDeployment**](WidgetDeployment.md) | Deployment | [required] |

### Return type

[**crate::models::WidgetDeployment**](WidgetDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_widgets_deployment

> crate::models::WidgetDeployment put_widgets_deployment(deployment_id, body)
Update a Widget deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **String** | Widget Config Id | [required] |
**body** | [**WidgetDeployment**](WidgetDeployment.md) | Deployment | [required] |

### Return type

[**crate::models::WidgetDeployment**](WidgetDeployment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

