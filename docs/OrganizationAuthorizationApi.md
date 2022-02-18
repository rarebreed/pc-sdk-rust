# \OrganizationAuthorizationApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_orgauthorization_trustee**](OrganizationAuthorizationApi.md#delete_orgauthorization_trustee) | **DELETE** /api/v2/orgauthorization/trustees/{trusteeOrgId} | Delete Org Trust
[**delete_orgauthorization_trustee_cloneduser**](OrganizationAuthorizationApi.md#delete_orgauthorization_trustee_cloneduser) | **DELETE** /api/v2/orgauthorization/trustees/{trusteeOrgId}/clonedusers/{trusteeUserId} | Deletes cloned user
[**delete_orgauthorization_trustee_user**](OrganizationAuthorizationApi.md#delete_orgauthorization_trustee_user) | **DELETE** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users/{trusteeUserId} | Delete Trustee User
[**delete_orgauthorization_trustee_user_roles**](OrganizationAuthorizationApi.md#delete_orgauthorization_trustee_user_roles) | **DELETE** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users/{trusteeUserId}/roles | Delete Trustee User Roles
[**delete_orgauthorization_trustor**](OrganizationAuthorizationApi.md#delete_orgauthorization_trustor) | **DELETE** /api/v2/orgauthorization/trustors/{trustorOrgId} | Delete Org Trust
[**delete_orgauthorization_trustor_cloneduser**](OrganizationAuthorizationApi.md#delete_orgauthorization_trustor_cloneduser) | **DELETE** /api/v2/orgauthorization/trustors/{trustorOrgId}/clonedusers/{trusteeUserId} | Delete Cloned User
[**delete_orgauthorization_trustor_user**](OrganizationAuthorizationApi.md#delete_orgauthorization_trustor_user) | **DELETE** /api/v2/orgauthorization/trustors/{trustorOrgId}/users/{trusteeUserId} | Delete Trustee User
[**get_orgauthorization_pairing**](OrganizationAuthorizationApi.md#get_orgauthorization_pairing) | **GET** /api/v2/orgauthorization/pairings/{pairingId} | Get Pairing Info
[**get_orgauthorization_trustee**](OrganizationAuthorizationApi.md#get_orgauthorization_trustee) | **GET** /api/v2/orgauthorization/trustees/{trusteeOrgId} | Get Org Trust
[**get_orgauthorization_trustee_clonedusers**](OrganizationAuthorizationApi.md#get_orgauthorization_trustee_clonedusers) | **GET** /api/v2/orgauthorization/trustees/{trusteeOrgId}/clonedusers | The list of cloned users from the trustee organization (i.e. users with a native user record).
[**get_orgauthorization_trustee_user**](OrganizationAuthorizationApi.md#get_orgauthorization_trustee_user) | **GET** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users/{trusteeUserId} | Get Trustee User
[**get_orgauthorization_trustee_user_roles**](OrganizationAuthorizationApi.md#get_orgauthorization_trustee_user_roles) | **GET** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users/{trusteeUserId}/roles | Get Trustee User Roles
[**get_orgauthorization_trustee_users**](OrganizationAuthorizationApi.md#get_orgauthorization_trustee_users) | **GET** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users | The list of trustee users for this organization (i.e. users granted access to this organization).
[**get_orgauthorization_trustees**](OrganizationAuthorizationApi.md#get_orgauthorization_trustees) | **GET** /api/v2/orgauthorization/trustees | The list of trustees for this organization (i.e. organizations granted access to this organization).
[**get_orgauthorization_trustees_default**](OrganizationAuthorizationApi.md#get_orgauthorization_trustees_default) | **GET** /api/v2/orgauthorization/trustees/default | Get organization authorization trust with Customer Care, if one exists.
[**get_orgauthorization_trustor**](OrganizationAuthorizationApi.md#get_orgauthorization_trustor) | **GET** /api/v2/orgauthorization/trustors/{trustorOrgId} | Get Org Trust
[**get_orgauthorization_trustor_cloneduser**](OrganizationAuthorizationApi.md#get_orgauthorization_trustor_cloneduser) | **GET** /api/v2/orgauthorization/trustors/{trustorOrgId}/clonedusers/{trusteeUserId} | Get Cloned User
[**get_orgauthorization_trustor_clonedusers**](OrganizationAuthorizationApi.md#get_orgauthorization_trustor_clonedusers) | **GET** /api/v2/orgauthorization/trustors/{trustorOrgId}/clonedusers | The list of cloned users in the trustor organization (i.e. users with a native user record).
[**get_orgauthorization_trustor_user**](OrganizationAuthorizationApi.md#get_orgauthorization_trustor_user) | **GET** /api/v2/orgauthorization/trustors/{trustorOrgId}/users/{trusteeUserId} | Get Trustee User
[**get_orgauthorization_trustor_users**](OrganizationAuthorizationApi.md#get_orgauthorization_trustor_users) | **GET** /api/v2/orgauthorization/trustors/{trustorOrgId}/users | The list of users in the trustor organization (i.e. users granted access).
[**get_orgauthorization_trustors**](OrganizationAuthorizationApi.md#get_orgauthorization_trustors) | **GET** /api/v2/orgauthorization/trustors | The list of organizations that have authorized/trusted your organization.
[**post_orgauthorization_pairings**](OrganizationAuthorizationApi.md#post_orgauthorization_pairings) | **POST** /api/v2/orgauthorization/pairings | A pairing id is created by the trustee and given to the trustor to create a trust.
[**post_orgauthorization_trustee_users**](OrganizationAuthorizationApi.md#post_orgauthorization_trustee_users) | **POST** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users | Add a user to the trust.
[**post_orgauthorization_trustees**](OrganizationAuthorizationApi.md#post_orgauthorization_trustees) | **POST** /api/v2/orgauthorization/trustees | Create a new organization authorization trust. This is required to grant other organizations access to your organization.
[**post_orgauthorization_trustees_audits**](OrganizationAuthorizationApi.md#post_orgauthorization_trustees_audits) | **POST** /api/v2/orgauthorization/trustees/audits | Get Org Trustee Audits
[**post_orgauthorization_trustees_default**](OrganizationAuthorizationApi.md#post_orgauthorization_trustees_default) | **POST** /api/v2/orgauthorization/trustees/default | Create a new organization authorization trust with Customer Care. This is required to grant your regional Customer Care organization access to your organization.
[**post_orgauthorization_trustor_audits**](OrganizationAuthorizationApi.md#post_orgauthorization_trustor_audits) | **POST** /api/v2/orgauthorization/trustor/audits | Get Org Trustor Audits
[**put_orgauthorization_trustee**](OrganizationAuthorizationApi.md#put_orgauthorization_trustee) | **PUT** /api/v2/orgauthorization/trustees/{trusteeOrgId} | Update Org Trust
[**put_orgauthorization_trustee_user_roledivisions**](OrganizationAuthorizationApi.md#put_orgauthorization_trustee_user_roledivisions) | **PUT** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users/{trusteeUserId}/roledivisions | Update Trustee User Roles
[**put_orgauthorization_trustee_user_roles**](OrganizationAuthorizationApi.md#put_orgauthorization_trustee_user_roles) | **PUT** /api/v2/orgauthorization/trustees/{trusteeOrgId}/users/{trusteeUserId}/roles | Update Trustee User Roles
[**put_orgauthorization_trustor_cloneduser**](OrganizationAuthorizationApi.md#put_orgauthorization_trustor_cloneduser) | **PUT** /api/v2/orgauthorization/trustors/{trustorOrgId}/clonedusers/{trusteeUserId} | Creates a clone of the trustee user in the trustor org.
[**put_orgauthorization_trustor_user**](OrganizationAuthorizationApi.md#put_orgauthorization_trustor_user) | **PUT** /api/v2/orgauthorization/trustors/{trustorOrgId}/users/{trusteeUserId} | Add a Trustee user to the trust.



## delete_orgauthorization_trustee

> delete_orgauthorization_trustee(trustee_org_id)
Delete Org Trust

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_orgauthorization_trustee_cloneduser

> delete_orgauthorization_trustee_cloneduser(trustee_org_id, trustee_user_id)
Deletes cloned user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**trustee_user_id** | **String** | Id of the cloned user to delete | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_orgauthorization_trustee_user

> delete_orgauthorization_trustee_user(trustee_org_id, trustee_user_id)
Delete Trustee User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_orgauthorization_trustee_user_roles

> delete_orgauthorization_trustee_user_roles(trustee_org_id, trustee_user_id)
Delete Trustee User Roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_orgauthorization_trustor

> delete_orgauthorization_trustor(trustor_org_id)
Delete Org Trust

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_orgauthorization_trustor_cloneduser

> delete_orgauthorization_trustor_cloneduser(trustor_org_id, trustee_user_id)
Delete Cloned User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_orgauthorization_trustor_user

> delete_orgauthorization_trustor_user(trustor_org_id, trustee_user_id)
Delete Trustee User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_pairing

> crate::models::TrustRequest get_orgauthorization_pairing(pairing_id)
Get Pairing Info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pairing_id** | **String** | Pairing Id | [required] |

### Return type

[**crate::models::TrustRequest**](TrustRequest.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustee

> crate::models::Trustee get_orgauthorization_trustee(trustee_org_id)
Get Org Trust

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |

### Return type

[**crate::models::Trustee**](Trustee.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustee_clonedusers

> crate::models::ClonedUserEntityListing get_orgauthorization_trustee_clonedusers(trustee_org_id)
The list of cloned users from the trustee organization (i.e. users with a native user record).

There can be no more than 5 cloned users per organization, so results are represented as simple list and not paged

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |

### Return type

[**crate::models::ClonedUserEntityListing**](ClonedUserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustee_user

> crate::models::TrustUser get_orgauthorization_trustee_user(trustee_org_id, trustee_user_id)
Get Trustee User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

[**crate::models::TrustUser**](TrustUser.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustee_user_roles

> crate::models::UserAuthorization get_orgauthorization_trustee_user_roles(trustee_org_id, trustee_user_id)
Get Trustee User Roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

[**crate::models::UserAuthorization**](UserAuthorization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustee_users

> crate::models::TrustUserEntityListing get_orgauthorization_trustee_users(trustee_org_id, page_size, page_number)
The list of trustee users for this organization (i.e. users granted access to this organization).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::TrustUserEntityListing**](TrustUserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustees

> crate::models::TrustEntityListing get_orgauthorization_trustees(page_size, page_number)
The list of trustees for this organization (i.e. organizations granted access to this organization).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::TrustEntityListing**](TrustEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustees_default

> crate::models::Trustee get_orgauthorization_trustees_default()
Get organization authorization trust with Customer Care, if one exists.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Trustee**](Trustee.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustor

> crate::models::Trustor get_orgauthorization_trustor(trustor_org_id)
Get Org Trust

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |

### Return type

[**crate::models::Trustor**](Trustor.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustor_cloneduser

> crate::models::ClonedUser get_orgauthorization_trustor_cloneduser(trustor_org_id, trustee_user_id)
Get Cloned User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

[**crate::models::ClonedUser**](ClonedUser.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustor_clonedusers

> crate::models::ClonedUserEntityListing get_orgauthorization_trustor_clonedusers(trustor_org_id)
The list of cloned users in the trustor organization (i.e. users with a native user record).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |

### Return type

[**crate::models::ClonedUserEntityListing**](ClonedUserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustor_user

> crate::models::TrustUser get_orgauthorization_trustor_user(trustor_org_id, trustee_user_id)
Get Trustee User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

[**crate::models::TrustUser**](TrustUser.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustor_users

> crate::models::TrustUserEntityListing get_orgauthorization_trustor_users(trustor_org_id, page_size, page_number)
The list of users in the trustor organization (i.e. users granted access).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustee Organization Id | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::TrustUserEntityListing**](TrustUserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orgauthorization_trustors

> crate::models::TrustorEntityListing get_orgauthorization_trustors(page_size, page_number)
The list of organizations that have authorized/trusted your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::TrustorEntityListing**](TrustorEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_orgauthorization_pairings

> crate::models::TrustRequest post_orgauthorization_pairings(body)
A pairing id is created by the trustee and given to the trustor to create a trust.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TrustRequestCreate**](TrustRequestCreate.md) | Pairing Info | [required] |

### Return type

[**crate::models::TrustRequest**](TrustRequest.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_orgauthorization_trustee_users

> crate::models::TrustUser post_orgauthorization_trustee_users(trustee_org_id, body)
Add a user to the trust.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**body** | [**TrustMemberCreate**](TrustMemberCreate.md) | Trust | [required] |

### Return type

[**crate::models::TrustUser**](TrustUser.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_orgauthorization_trustees

> crate::models::Trustee post_orgauthorization_trustees(body)
Create a new organization authorization trust. This is required to grant other organizations access to your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TrustCreate**](TrustCreate.md) | Trust | [required] |

### Return type

[**crate::models::Trustee**](Trustee.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_orgauthorization_trustees_audits

> serde_json::Value post_orgauthorization_trustees_audits(body, page_size, page_number, sort_by, sort_order)
Get Org Trustee Audits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TrusteeAuditQueryRequest**](TrusteeAuditQueryRequest.md) | Values to scope the request. | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to timestamp]
**sort_order** | Option<**String**> | Sort order |  |[default to descending]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_orgauthorization_trustees_default

> crate::models::Trustee post_orgauthorization_trustees_default(assign_default_role, auto_expire)
Create a new organization authorization trust with Customer Care. This is required to grant your regional Customer Care organization access to your organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assign_default_role** | Option<**bool**> | Assign Admin role to default pairing with Customer Care |  |
**auto_expire** | Option<**bool**> | Automatically expire pairing after 30 days |  |

### Return type

[**crate::models::Trustee**](Trustee.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_orgauthorization_trustor_audits

> serde_json::Value post_orgauthorization_trustor_audits(body, page_size, page_number, sort_by, sort_order)
Get Org Trustor Audits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TrustorAuditQueryRequest**](TrustorAuditQueryRequest.md) | Values to scope the request. | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to timestamp]
**sort_order** | Option<**String**> | Sort order |  |[default to descending]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_orgauthorization_trustee

> crate::models::Trustee put_orgauthorization_trustee(trustee_org_id, body)
Update Org Trust

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**body** | [**TrustUpdate**](TrustUpdate.md) | Client | [required] |

### Return type

[**crate::models::Trustee**](Trustee.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_orgauthorization_trustee_user_roledivisions

> crate::models::UserAuthorization put_orgauthorization_trustee_user_roledivisions(trustee_org_id, trustee_user_id, body)
Update Trustee User Roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |
**body** | [**RoleDivisionGrants**](RoleDivisionGrants.md) | Set of roles with corresponding divisions to apply | [required] |

### Return type

[**crate::models::UserAuthorization**](UserAuthorization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_orgauthorization_trustee_user_roles

> crate::models::UserAuthorization put_orgauthorization_trustee_user_roles(trustee_org_id, trustee_user_id, body)
Update Trustee User Roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustee_org_id** | **String** | Trustee Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |
**body** | [**Vec<String>**](String.md) | List of roles | [required] |

### Return type

[**crate::models::UserAuthorization**](UserAuthorization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_orgauthorization_trustor_cloneduser

> crate::models::ClonedUser put_orgauthorization_trustor_cloneduser(trustor_org_id, trustee_user_id)
Creates a clone of the trustee user in the trustor org.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

[**crate::models::ClonedUser**](ClonedUser.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_orgauthorization_trustor_user

> crate::models::TrustUser put_orgauthorization_trustor_user(trustor_org_id, trustee_user_id)
Add a Trustee user to the trust.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**trustor_org_id** | **String** | Trustor Organization Id | [required] |
**trustee_user_id** | **String** | Trustee User Id | [required] |

### Return type

[**crate::models::TrustUser**](TrustUser.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

