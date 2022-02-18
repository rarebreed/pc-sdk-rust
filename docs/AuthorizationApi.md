# \AuthorizationApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_authorization_division**](AuthorizationApi.md#delete_authorization_division) | **DELETE** /api/v2/authorization/divisions/{divisionId} | Delete a division.
[**delete_authorization_role**](AuthorizationApi.md#delete_authorization_role) | **DELETE** /api/v2/authorization/roles/{roleId} | Delete an organization role.
[**delete_authorization_subject_division_role**](AuthorizationApi.md#delete_authorization_subject_division_role) | **DELETE** /api/v2/authorization/subjects/{subjectId}/divisions/{divisionId}/roles/{roleId} | Delete a grant of a role in a division
[**get_authorization_division**](AuthorizationApi.md#get_authorization_division) | **GET** /api/v2/authorization/divisions/{divisionId} | Returns an authorization division.
[**get_authorization_division_grants**](AuthorizationApi.md#get_authorization_division_grants) | **GET** /api/v2/authorization/divisions/{divisionId}/grants | Gets all grants for a given division.
[**get_authorization_divisions**](AuthorizationApi.md#get_authorization_divisions) | **GET** /api/v2/authorization/divisions | Retrieve a list of all divisions defined for the organization
[**get_authorization_divisions_home**](AuthorizationApi.md#get_authorization_divisions_home) | **GET** /api/v2/authorization/divisions/home | Retrieve the home division for the organization.
[**get_authorization_divisions_limit**](AuthorizationApi.md#get_authorization_divisions_limit) | **GET** /api/v2/authorization/divisions/limit | Returns the maximum allowed number of divisions.
[**get_authorization_divisionspermitted_me**](AuthorizationApi.md#get_authorization_divisionspermitted_me) | **GET** /api/v2/authorization/divisionspermitted/me | Returns which divisions the current user has the given permission in.
[**get_authorization_divisionspermitted_paged_me**](AuthorizationApi.md#get_authorization_divisionspermitted_paged_me) | **GET** /api/v2/authorization/divisionspermitted/paged/me | Returns which divisions the current user has the given permission in.
[**get_authorization_divisionspermitted_paged_subject_id**](AuthorizationApi.md#get_authorization_divisionspermitted_paged_subject_id) | **GET** /api/v2/authorization/divisionspermitted/paged/{subjectId} | Returns which divisions the specified user has the given permission in.
[**get_authorization_permissions**](AuthorizationApi.md#get_authorization_permissions) | **GET** /api/v2/authorization/permissions | Get all permissions.
[**get_authorization_products**](AuthorizationApi.md#get_authorization_products) | **GET** /api/v2/authorization/products | Get the list of enabled products
[**get_authorization_role**](AuthorizationApi.md#get_authorization_role) | **GET** /api/v2/authorization/roles/{roleId} | Get a single organization role.
[**get_authorization_role_comparedefault_right_role_id**](AuthorizationApi.md#get_authorization_role_comparedefault_right_role_id) | **GET** /api/v2/authorization/roles/{leftRoleId}/comparedefault/{rightRoleId} | Get an org role to default role comparison
[**get_authorization_role_subjectgrants**](AuthorizationApi.md#get_authorization_role_subjectgrants) | **GET** /api/v2/authorization/roles/{roleId}/subjectgrants | Get the subjects' granted divisions in the specified role.
[**get_authorization_role_users**](AuthorizationApi.md#get_authorization_role_users) | **GET** /api/v2/authorization/roles/{roleId}/users | Get a list of the users in a specified role.
[**get_authorization_roles**](AuthorizationApi.md#get_authorization_roles) | **GET** /api/v2/authorization/roles | Retrieve a list of all roles defined for the organization
[**get_authorization_subject**](AuthorizationApi.md#get_authorization_subject) | **GET** /api/v2/authorization/subjects/{subjectId} | Returns a listing of roles and permissions for a user.
[**get_authorization_subjects_me**](AuthorizationApi.md#get_authorization_subjects_me) | **GET** /api/v2/authorization/subjects/me | Returns a listing of roles and permissions for the currently authenticated user.
[**get_authorization_subjects_rolecounts**](AuthorizationApi.md#get_authorization_subjects_rolecounts) | **GET** /api/v2/authorization/subjects/rolecounts | Get the count of roles granted to a list of subjects
[**get_user_roles**](AuthorizationApi.md#get_user_roles) | **GET** /api/v2/users/{userId}/roles | Returns a listing of roles and permissions for a user.
[**patch_authorization_role**](AuthorizationApi.md#patch_authorization_role) | **PATCH** /api/v2/authorization/roles/{roleId} | Patch Organization Role for needsUpdate Field
[**post_authorization_division_object**](AuthorizationApi.md#post_authorization_division_object) | **POST** /api/v2/authorization/divisions/{divisionId}/objects/{objectType} | Assign a list of objects to a division
[**post_authorization_division_restore**](AuthorizationApi.md#post_authorization_division_restore) | **POST** /api/v2/authorization/divisions/{divisionId}/restore | Recreate a previously deleted division.
[**post_authorization_divisions**](AuthorizationApi.md#post_authorization_divisions) | **POST** /api/v2/authorization/divisions | Create a division.
[**post_authorization_role**](AuthorizationApi.md#post_authorization_role) | **POST** /api/v2/authorization/roles/{roleId} | Bulk-grant subjects and divisions with an organization role.
[**post_authorization_role_comparedefault_right_role_id**](AuthorizationApi.md#post_authorization_role_comparedefault_right_role_id) | **POST** /api/v2/authorization/roles/{leftRoleId}/comparedefault/{rightRoleId} | Get an unsaved org role to default role comparison
[**post_authorization_roles**](AuthorizationApi.md#post_authorization_roles) | **POST** /api/v2/authorization/roles | Create an organization role.
[**post_authorization_roles_default**](AuthorizationApi.md#post_authorization_roles_default) | **POST** /api/v2/authorization/roles/default | Restores all default roles
[**post_authorization_subject_bulkadd**](AuthorizationApi.md#post_authorization_subject_bulkadd) | **POST** /api/v2/authorization/subjects/{subjectId}/bulkadd | Bulk-grant roles and divisions to a subject.
[**post_authorization_subject_bulkremove**](AuthorizationApi.md#post_authorization_subject_bulkremove) | **POST** /api/v2/authorization/subjects/{subjectId}/bulkremove | Bulk-remove grants from a subject.
[**post_authorization_subject_bulkreplace**](AuthorizationApi.md#post_authorization_subject_bulkreplace) | **POST** /api/v2/authorization/subjects/{subjectId}/bulkreplace | Replace subject's roles and divisions with the exact list supplied in the request.
[**post_authorization_subject_division_role**](AuthorizationApi.md#post_authorization_subject_division_role) | **POST** /api/v2/authorization/subjects/{subjectId}/divisions/{divisionId}/roles/{roleId} | Make a grant of a role in a division
[**put_authorization_division**](AuthorizationApi.md#put_authorization_division) | **PUT** /api/v2/authorization/divisions/{divisionId} | Update a division.
[**put_authorization_role**](AuthorizationApi.md#put_authorization_role) | **PUT** /api/v2/authorization/roles/{roleId} | Update an organization role.
[**put_authorization_role_users_add**](AuthorizationApi.md#put_authorization_role_users_add) | **PUT** /api/v2/authorization/roles/{roleId}/users/add | Sets the users for the role
[**put_authorization_role_users_remove**](AuthorizationApi.md#put_authorization_role_users_remove) | **PUT** /api/v2/authorization/roles/{roleId}/users/remove | Removes the users from the role
[**put_authorization_roles_default**](AuthorizationApi.md#put_authorization_roles_default) | **PUT** /api/v2/authorization/roles/default | Restore specified default roles
[**put_user_roles**](AuthorizationApi.md#put_user_roles) | **PUT** /api/v2/users/{userId}/roles | Sets the user's roles



## delete_authorization_division

> delete_authorization_division(division_id, force)
Delete a division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**force** | Option<**bool**> | Force delete this division as well as the grants and objects associated with it |  |[default to false]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_authorization_role

> delete_authorization_role(role_id)
Delete an organization role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_authorization_subject_division_role

> delete_authorization_subject_division_role(subject_id, division_id, role_id)
Delete a grant of a role in a division

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** | Subject ID (user or group) | [required] |
**division_id** | **String** | the id of the division of the grant | [required] |
**role_id** | **String** | the id of the role of the grant | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_division

> crate::models::AuthzDivision get_authorization_division(division_id, object_count)
Returns an authorization division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**object_count** | Option<**bool**> | Get count of objects in this division, grouped by type |  |[default to false]

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_division_grants

> crate::models::AuthzDivisionGrantEntityListing get_authorization_division_grants(division_id, page_number, page_size)
Gets all grants for a given division.

Returns all grants assigned to a given division. Maximum page size is 500.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::AuthzDivisionGrantEntityListing**](AuthzDivisionGrantEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisions

> crate::models::AuthzDivisionEntityListing get_authorization_divisions(page_size, page_number, sort_by, expand, next_page, previous_page, object_count, id, name)
Retrieve a list of all divisions defined for the organization

Request specific divisions by id using a query param \"id\", e.g.  ?id=5f777167-63be-4c24-ad41-374155d9e28b&id=72e9fb25-c484-488d-9312-7acba82435b3

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**object_count** | Option<**bool**> | Include the count of objects contained in the division |  |[default to false]
**id** | Option<[**Vec<String>**](String.md)> | Optionally request specific divisions by their IDs |  |
**name** | Option<**String**> | Search term to filter by division name |  |

### Return type

[**crate::models::AuthzDivisionEntityListing**](AuthzDivisionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisions_home

> crate::models::AuthzDivision get_authorization_divisions_home()
Retrieve the home division for the organization.

Will not include object counts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisions_limit

> i32 get_authorization_divisions_limit()
Returns the maximum allowed number of divisions.

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisionspermitted_me

> Vec<crate::models::AuthzDivision> get_authorization_divisionspermitted_me(permission, name)
Returns which divisions the current user has the given permission in.

This route is deprecated, use authorization/divisionspermitted/paged/me instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission** | **String** | The permission string, including the object to access, e.g. routing:queue:view | [required] |
**name** | Option<**String**> | Search term to filter by division name |  |

### Return type

[**Vec<crate::models::AuthzDivision>**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisionspermitted_paged_me

> crate::models::DivsPermittedEntityListing get_authorization_divisionspermitted_paged_me(permission, page_number, page_size)
Returns which divisions the current user has the given permission in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permission** | **String** | The permission string, including the object to access, e.g. routing:queue:view | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::DivsPermittedEntityListing**](DivsPermittedEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_divisionspermitted_paged_subject_id

> crate::models::DivsPermittedEntityListing get_authorization_divisionspermitted_paged_subject_id(subject_id, permission, page_number, page_size)
Returns which divisions the specified user has the given permission in.

This route is deprecated, use authorization/divisionspermitted/paged/me instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** | Subject ID (user or group) | [required] |
**permission** | **String** | The permission string, including the object to access, e.g. routing:queue:view | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::DivsPermittedEntityListing**](DivsPermittedEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_permissions

> crate::models::PermissionCollectionEntityListing get_authorization_permissions(page_size, page_number, query_type, query)
Get all permissions.

Retrieve a list of all permission defined in the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**query_type** | Option<**String**> | Query filter type |  |
**query** | Option<**String**> | Comma-separated list of permissions or domains to query |  |

### Return type

[**crate::models::PermissionCollectionEntityListing**](PermissionCollectionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_products

> crate::models::OrganizationProductEntityListing get_authorization_products()
Get the list of enabled products

Gets the list of enabled products. Some example product names are: collaborateFree, collaboratePro, communicate, and engage.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OrganizationProductEntityListing**](OrganizationProductEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_role

> crate::models::DomainOrganizationRole get_authorization_role(role_id, expand)
Get a single organization role.

Get the organization role specified by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. \"unusedPermissions\" returns the permissions not used for the role |  |

### Return type

[**crate::models::DomainOrganizationRole**](DomainOrganizationRole.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_role_comparedefault_right_role_id

> crate::models::DomainOrgRoleDifference get_authorization_role_comparedefault_right_role_id(left_role_id, right_role_id)
Get an org role to default role comparison

Compares any organization role to a default role id and show differences

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**left_role_id** | **String** | Left Role ID | [required] |
**right_role_id** | **String** | Right Role id | [required] |

### Return type

[**crate::models::DomainOrgRoleDifference**](DomainOrgRoleDifference.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_role_subjectgrants

> crate::models::SubjectDivisionGrantsEntityListing get_authorization_role_subjectgrants(role_id, page_size, page_number, sort_by, expand, next_page, previous_page)
Get the subjects' granted divisions in the specified role.

Includes the divisions for which the subject has a grant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |

### Return type

[**crate::models::SubjectDivisionGrantsEntityListing**](SubjectDivisionGrantsEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_role_users

> crate::models::UserEntityListing get_authorization_role_users(role_id, page_size, page_number)
Get a list of the users in a specified role.

Get an array of the UUIDs of the users in the specified role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::UserEntityListing**](UserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_roles

> crate::models::OrganizationRoleEntityListing get_authorization_roles(page_size, page_number, sort_by, expand, next_page, previous_page, name, permission, default_role_id, user_count, id)
Retrieve a list of all roles defined for the organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**name** | Option<**String**> |  |  |
**permission** | Option<[**Vec<String>**](String.md)> |  |  |
**default_role_id** | Option<[**Vec<String>**](String.md)> |  |  |
**user_count** | Option<**bool**> |  |  |[default to true]
**id** | Option<[**Vec<String>**](String.md)> | id |  |

### Return type

[**crate::models::OrganizationRoleEntityListing**](OrganizationRoleEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_subject

> crate::models::AuthzSubject get_authorization_subject(subject_id)
Returns a listing of roles and permissions for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** | Subject ID (user or group) | [required] |

### Return type

[**crate::models::AuthzSubject**](AuthzSubject.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_subjects_me

> crate::models::AuthzSubject get_authorization_subjects_me()
Returns a listing of roles and permissions for the currently authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AuthzSubject**](AuthzSubject.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_authorization_subjects_rolecounts

> ::std::collections::HashMap<String, serde_json::Value> get_authorization_subjects_rolecounts(id)
Get the count of roles granted to a list of subjects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | Option<[**Vec<String>**](String.md)> | id |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_roles

> crate::models::UserAuthorization get_user_roles(user_id)
Returns a listing of roles and permissions for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::UserAuthorization**](UserAuthorization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_authorization_role

> crate::models::DomainOrganizationRole patch_authorization_role(role_id, body)
Patch Organization Role for needsUpdate Field

Patch Organization Role for needsUpdate Field

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**body** | [**DomainOrganizationRole**](DomainOrganizationRole.md) | Organization role | [required] |

### Return type

[**crate::models::DomainOrganizationRole**](DomainOrganizationRole.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_division_object

> post_authorization_division_object(division_id, object_type, body)
Assign a list of objects to a division

Set the division of a specified list of objects. The objects must all be of the same type, one of:  CAMPAIGN, MANAGEMENTUNIT, FLOW, QUEUE, DATATABLES or USER.  The body of the request is a list of object IDs, which are expected to be  GUIDs, e.g. [\"206ce31f-61ec-40ed-a8b1-be6f06303998\",\"250a754e-f5e4-4f51-800f-a92f09d3bf8c\"]

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**object_type** | **String** | The type of the objects. Must be one of the valid object types | [required] |
**body** | [**Vec<String>**](String.md) | Object Id List | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_division_restore

> crate::models::AuthzDivision post_authorization_division_restore(division_id, body)
Recreate a previously deleted division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**body** | [**AuthzDivision**](AuthzDivision.md) | Recreated division data | [required] |

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_divisions

> crate::models::AuthzDivision post_authorization_divisions(body)
Create a division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AuthzDivision**](AuthzDivision.md) | Division | [required] |

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_role

> post_authorization_role(role_id, body, subject_type)
Bulk-grant subjects and divisions with an organization role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**body** | [**SubjectDivisions**](SubjectDivisions.md) | Subjects and Divisions | [required] |
**subject_type** | Option<**String**> | what the type of the subjects are (PC_GROUP, PC_USER or PC_OAUTH_CLIENT) |  |[default to PC_USER]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_role_comparedefault_right_role_id

> crate::models::DomainOrgRoleDifference post_authorization_role_comparedefault_right_role_id(left_role_id, right_role_id, body)
Get an unsaved org role to default role comparison

Allows users to compare their existing roles in an unsaved state to its default role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**left_role_id** | **String** | Left Role ID | [required] |
**right_role_id** | **String** | Right Role id | [required] |
**body** | [**DomainOrganizationRole**](DomainOrganizationRole.md) | Organization role | [required] |

### Return type

[**crate::models::DomainOrgRoleDifference**](DomainOrgRoleDifference.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_roles

> crate::models::DomainOrganizationRole post_authorization_roles(body)
Create an organization role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DomainOrganizationRoleCreate**](DomainOrganizationRoleCreate.md) | Organization role | [required] |

### Return type

[**crate::models::DomainOrganizationRole**](DomainOrganizationRole.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_roles_default

> crate::models::OrganizationRoleEntityListing post_authorization_roles_default(force)
Restores all default roles

This endpoint serves several purposes. 1. It provides the org with default roles. This is important for default roles that will be added after go-live (they can retroactively add the new default-role). Note: When not using a query param of force=true, it only adds the default roles not configured for the org; it does not overwrite roles. 2. Using the query param force=true, you can restore all default roles. Note: This does not have an effect on custom roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | Option<**bool**> | Restore default roles |  |[default to false]

### Return type

[**crate::models::OrganizationRoleEntityListing**](OrganizationRoleEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_subject_bulkadd

> post_authorization_subject_bulkadd(subject_id, body, subject_type)
Bulk-grant roles and divisions to a subject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** | Subject ID (user or group) | [required] |
**body** | [**RoleDivisionGrants**](RoleDivisionGrants.md) | Pairs of role and division IDs | [required] |
**subject_type** | Option<**String**> | what the type of the subject is (PC_GROUP, PC_USER or PC_OAUTH_CLIENT) |  |[default to PC_USER]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_subject_bulkremove

> post_authorization_subject_bulkremove(subject_id, body)
Bulk-remove grants from a subject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** | Subject ID (user or group) | [required] |
**body** | [**RoleDivisionGrants**](RoleDivisionGrants.md) | Pairs of role and division IDs | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_subject_bulkreplace

> post_authorization_subject_bulkreplace(subject_id, body, subject_type)
Replace subject's roles and divisions with the exact list supplied in the request.

This operation will not remove grants that are inherited from group membership. It will only set the grants directly applied to the subject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** | Subject ID (user or group) | [required] |
**body** | [**RoleDivisionGrants**](RoleDivisionGrants.md) | Pairs of role and division IDs | [required] |
**subject_type** | Option<**String**> | what the type of the subject is (PC_GROUP, PC_USER or PC_OAUTH_CLIENT) |  |[default to PC_USER]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_authorization_subject_division_role

> post_authorization_subject_division_role(subject_id, division_id, role_id, subject_type)
Make a grant of a role in a division

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject_id** | **String** | Subject ID (user or group) | [required] |
**division_id** | **String** | the id of the division to which to make the grant | [required] |
**role_id** | **String** | the id of the role to grant | [required] |
**subject_type** | Option<**String**> | what the type of the subject is: PC_GROUP, PC_USER or PC_OAUTH_CLIENT (note: for cross-org authorization, please use the Organization Authorization endpoints) |  |[default to PC_USER]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_authorization_division

> crate::models::AuthzDivision put_authorization_division(division_id, body)
Update a division.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | **String** | Division ID | [required] |
**body** | [**AuthzDivision**](AuthzDivision.md) | Updated division data | [required] |

### Return type

[**crate::models::AuthzDivision**](AuthzDivision.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_authorization_role

> crate::models::DomainOrganizationRole put_authorization_role(role_id, body)
Update an organization role.

Update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**body** | [**DomainOrganizationRoleUpdate**](DomainOrganizationRoleUpdate.md) | Organization role | [required] |

### Return type

[**crate::models::DomainOrganizationRole**](DomainOrganizationRole.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_authorization_role_users_add

> Vec<String> put_authorization_role_users_add(role_id, body)
Sets the users for the role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**body** | [**Vec<String>**](String.md) | List of user IDs | [required] |

### Return type

**Vec<String>**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_authorization_role_users_remove

> Vec<String> put_authorization_role_users_remove(role_id, body)
Removes the users from the role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_id** | **String** | Role ID | [required] |
**body** | [**Vec<String>**](String.md) | List of user IDs | [required] |

### Return type

**Vec<String>**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_authorization_roles_default

> crate::models::OrganizationRoleEntityListing put_authorization_roles_default(body)
Restore specified default roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::DomainOrganizationRole>**](DomainOrganizationRole.md) | Organization roles list | [required] |

### Return type

[**crate::models::OrganizationRoleEntityListing**](OrganizationRoleEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_roles

> crate::models::UserAuthorization put_user_roles(user_id, body)
Sets the user's roles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Vec<String>**](String.md) | List of roles | [required] |

### Return type

[**crate::models::UserAuthorization**](UserAuthorization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

