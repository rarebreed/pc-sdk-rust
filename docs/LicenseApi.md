# \LicenseApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_license_definition**](LicenseApi.md#get_license_definition) | **GET** /api/v2/license/definitions/{licenseId} | Get PureCloud license definition.
[**get_license_definitions**](LicenseApi.md#get_license_definitions) | **GET** /api/v2/license/definitions | Get all PureCloud license definitions available for the organization.
[**get_license_toggle**](LicenseApi.md#get_license_toggle) | **GET** /api/v2/license/toggles/{featureName} | Get PureCloud license feature toggle value.
[**get_license_user**](LicenseApi.md#get_license_user) | **GET** /api/v2/license/users/{userId} | Get licenses for specified user.
[**get_license_users**](LicenseApi.md#get_license_users) | **GET** /api/v2/license/users | Get a page of users and their licenses
[**post_license_infer**](LicenseApi.md#post_license_infer) | **POST** /api/v2/license/infer | Get a list of licenses inferred based on a list of roleIds
[**post_license_organization**](LicenseApi.md#post_license_organization) | **POST** /api/v2/license/organization | Update the organization's license assignments in a batch.
[**post_license_toggle**](LicenseApi.md#post_license_toggle) | **POST** /api/v2/license/toggles/{featureName} | Switch PureCloud license feature toggle value.
[**post_license_users**](LicenseApi.md#post_license_users) | **POST** /api/v2/license/users | Fetch user licenses in a batch.



## get_license_definition

> crate::models::LicenseDefinition get_license_definition(license_id)
Get PureCloud license definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_id** | **String** | ID | [required] |

### Return type

[**crate::models::LicenseDefinition**](LicenseDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_definitions

> Vec<crate::models::LicenseDefinition> get_license_definitions()
Get all PureCloud license definitions available for the organization.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LicenseDefinition>**](LicenseDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_toggle

> crate::models::LicenseOrgToggle get_license_toggle(feature_name)
Get PureCloud license feature toggle value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_name** | **String** | featureName | [required] |

### Return type

[**crate::models::LicenseOrgToggle**](LicenseOrgToggle.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_user

> crate::models::LicenseUser get_license_user(user_id)
Get licenses for specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID | [required] |

### Return type

[**crate::models::LicenseUser**](LicenseUser.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_license_users

> crate::models::UserLicensesEntityListing get_license_users(page_size, page_number)
Get a page of users and their licenses

Retrieve a page of users in an organization along with the licenses they possess.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::UserLicensesEntityListing**](UserLicensesEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_license_infer

> Vec<String> post_license_infer(body)
Get a list of licenses inferred based on a list of roleIds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Vec<String>**](String.md)> | The roleIds to use while inferring licenses |  |

### Return type

**Vec<String>**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_license_organization

> Vec<crate::models::LicenseUpdateStatus> post_license_organization(body)
Update the organization's license assignments in a batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**LicenseBatchAssignmentRequest**](LicenseBatchAssignmentRequest.md)> | The license assignments to update. |  |

### Return type

[**Vec<crate::models::LicenseUpdateStatus>**](LicenseUpdateStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_license_toggle

> crate::models::LicenseOrgToggle post_license_toggle(feature_name)
Switch PureCloud license feature toggle value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**feature_name** | **String** | featureName | [required] |

### Return type

[**crate::models::LicenseOrgToggle**](LicenseOrgToggle.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_license_users

> ::std::collections::HashMap<String, serde_json::Value> post_license_users(body)
Fetch user licenses in a batch.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Vec<String>**](String.md)> | The user IDs to fetch. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

