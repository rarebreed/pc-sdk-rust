# \OrganizationApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_fieldconfig**](OrganizationApi.md#get_fieldconfig) | **GET** /api/v2/fieldconfig | Fetch field config for an entity type
[**get_organizations_embeddedintegration**](OrganizationApi.md#get_organizations_embeddedintegration) | **GET** /api/v2/organizations/embeddedintegration | Get the list of domains that will be allowed to embed PureCloud applications
[**get_organizations_ipaddressauthentication**](OrganizationApi.md#get_organizations_ipaddressauthentication) | **GET** /api/v2/organizations/ipaddressauthentication | Get organization IP address whitelist settings
[**get_organizations_limits_changerequest**](OrganizationApi.md#get_organizations_limits_changerequest) | **GET** /api/v2/organizations/limits/changerequests/{requestId} | Get a limit change request
[**get_organizations_limits_changerequests**](OrganizationApi.md#get_organizations_limits_changerequests) | **GET** /api/v2/organizations/limits/changerequests | Get the available limit change requests
[**get_organizations_limits_docs**](OrganizationApi.md#get_organizations_limits_docs) | **GET** /api/v2/organizations/limits/docs | Get a link to the limit documentation
[**get_organizations_limits_namespace**](OrganizationApi.md#get_organizations_limits_namespace) | **GET** /api/v2/organizations/limits/namespaces/{namespaceName} | Get the effective limits in a namespace for an organization
[**get_organizations_limits_namespace_defaults**](OrganizationApi.md#get_organizations_limits_namespace_defaults) | **GET** /api/v2/organizations/limits/namespaces/{namespaceName}/defaults | Get the default limits in a namespace for an organization
[**get_organizations_limits_namespaces**](OrganizationApi.md#get_organizations_limits_namespaces) | **GET** /api/v2/organizations/limits/namespaces | Get the available limit namespaces
[**get_organizations_me**](OrganizationApi.md#get_organizations_me) | **GET** /api/v2/organizations/me | Get organization.
[**get_organizations_whitelist**](OrganizationApi.md#get_organizations_whitelist) | **GET** /api/v2/organizations/whitelist | Use PUT /api/v2/organizations/embeddedintegration instead
[**patch_organizations_feature**](OrganizationApi.md#patch_organizations_feature) | **PATCH** /api/v2/organizations/features/{featureName} | Update organization
[**put_organizations_embeddedintegration**](OrganizationApi.md#put_organizations_embeddedintegration) | **PUT** /api/v2/organizations/embeddedintegration | Update the list of domains that will be allowed to embed PureCloud applications
[**put_organizations_ipaddressauthentication**](OrganizationApi.md#put_organizations_ipaddressauthentication) | **PUT** /api/v2/organizations/ipaddressauthentication | Update organization IP address whitelist settings
[**put_organizations_me**](OrganizationApi.md#put_organizations_me) | **PUT** /api/v2/organizations/me | Update organization.
[**put_organizations_whitelist**](OrganizationApi.md#put_organizations_whitelist) | **PUT** /api/v2/organizations/whitelist | Use PUT /api/v2/organizations/embeddedintegration instead



## get_fieldconfig

> crate::models::FieldConfig get_fieldconfig(_type)
Fetch field config for an entity type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** | Field type | [required] |

### Return type

[**crate::models::FieldConfig**](FieldConfig.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_embeddedintegration

> crate::models::EmbeddedIntegration get_organizations_embeddedintegration()
Get the list of domains that will be allowed to embed PureCloud applications

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EmbeddedIntegration**](EmbeddedIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_ipaddressauthentication

> crate::models::IpAddressAuthentication get_organizations_ipaddressauthentication()
Get organization IP address whitelist settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::IpAddressAuthentication**](IpAddressAuthentication.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_limits_changerequest

> crate::models::LimitChangeRequestDetails get_organizations_limits_changerequest(request_id)
Get a limit change request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** | Unique id for the limit change request | [required] |

### Return type

[**crate::models::LimitChangeRequestDetails**](LimitChangeRequestDetails.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_limits_changerequests

> crate::models::LimitChangeRequestsEntityListing get_organizations_limits_changerequests(after, before, status, page_size, expand)
Get the available limit change requests

Timestamp interval defaults to the last 365 days if both query parameters are omitted. If only one parameter is omitted, the interval will default to a 180 day range in the specified direction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**i64**> | Timestamp indicating the date to begin after when searching for requests. |  |
**before** | Option<**i64**> | Timestamp indicating the date to end before when searching for requests. |  |
**status** | Option<**String**> | Status of the request to be filtered by |  |
**page_size** | Option<**i32**> | Page Size |  |[default to 25]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::LimitChangeRequestsEntityListing**](LimitChangeRequestsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_limits_docs

> crate::models::UrlResponse get_organizations_limits_docs()
Get a link to the limit documentation

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UrlResponse**](UrlResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_limits_namespace

> crate::models::LimitsEntityListing get_organizations_limits_namespace(namespace_name)
Get the effective limits in a namespace for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_name** | **String** | The namespace to fetch limits for | [required] |

### Return type

[**crate::models::LimitsEntityListing**](LimitsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_limits_namespace_defaults

> crate::models::LimitsEntityListing get_organizations_limits_namespace_defaults(namespace_name)
Get the default limits in a namespace for an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace_name** | **String** | The namespace to fetch defaults limits for | [required] |

### Return type

[**crate::models::LimitsEntityListing**](LimitsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_limits_namespaces

> serde_json::Value get_organizations_limits_namespaces(page_size, page_number)
Get the available limit namespaces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 100]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_me

> crate::models::Organization get_organizations_me()
Get organization.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_whitelist

> crate::models::OrgWhitelistSettings get_organizations_whitelist()
Use PUT /api/v2/organizations/embeddedintegration instead

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OrgWhitelistSettings**](OrgWhitelistSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_organizations_feature

> crate::models::OrganizationFeatures patch_organizations_feature(feature_name, enabled)
Update organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_name** | **String** | Organization feature | [required] |
**enabled** | [**FeatureState**](FeatureState.md) | New state of feature | [required] |

### Return type

[**crate::models::OrganizationFeatures**](OrganizationFeatures.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_organizations_embeddedintegration

> crate::models::EmbeddedIntegration put_organizations_embeddedintegration(body)
Update the list of domains that will be allowed to embed PureCloud applications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EmbeddedIntegration**](EmbeddedIntegration.md) | Whitelist settings | [required] |

### Return type

[**crate::models::EmbeddedIntegration**](EmbeddedIntegration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_organizations_ipaddressauthentication

> crate::models::IpAddressAuthentication put_organizations_ipaddressauthentication(body)
Update organization IP address whitelist settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**IpAddressAuthentication**](IpAddressAuthentication.md) | IP address Whitelist settings | [required] |

### Return type

[**crate::models::IpAddressAuthentication**](IpAddressAuthentication.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_organizations_me

> crate::models::Organization put_organizations_me(body)
Update organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Organization**](Organization.md)> | Organization |  |

### Return type

[**crate::models::Organization**](Organization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_organizations_whitelist

> crate::models::OrgWhitelistSettings put_organizations_whitelist(body)
Use PUT /api/v2/organizations/embeddedintegration instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**OrgWhitelistSettings**](OrgWhitelistSettings.md) | Whitelist settings | [required] |

### Return type

[**crate::models::OrgWhitelistSettings**](OrgWhitelistSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

