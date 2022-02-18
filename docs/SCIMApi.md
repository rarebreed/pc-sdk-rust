# \SCIMApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_scim_user**](SCIMApi.md#delete_scim_user) | **DELETE** /api/v2/scim/users/{userId} | Delete a user
[**delete_scim_v2_user**](SCIMApi.md#delete_scim_v2_user) | **DELETE** /api/v2/scim/v2/users/{userId} | Delete a user
[**get_scim_group**](SCIMApi.md#get_scim_group) | **GET** /api/v2/scim/groups/{groupId} | Get a group
[**get_scim_groups**](SCIMApi.md#get_scim_groups) | **GET** /api/v2/scim/groups | Get a list of groups
[**get_scim_resourcetype**](SCIMApi.md#get_scim_resourcetype) | **GET** /api/v2/scim/resourcetypes/{resourceType} | Get a resource type
[**get_scim_resourcetypes**](SCIMApi.md#get_scim_resourcetypes) | **GET** /api/v2/scim/resourcetypes | Get a list of resource types
[**get_scim_schema**](SCIMApi.md#get_scim_schema) | **GET** /api/v2/scim/schemas/{schemaId} | Get a SCIM schema
[**get_scim_schemas**](SCIMApi.md#get_scim_schemas) | **GET** /api/v2/scim/schemas | Get a list of SCIM schemas
[**get_scim_serviceproviderconfig**](SCIMApi.md#get_scim_serviceproviderconfig) | **GET** /api/v2/scim/serviceproviderconfig | Get a service provider's configuration
[**get_scim_user**](SCIMApi.md#get_scim_user) | **GET** /api/v2/scim/users/{userId} | Get a user
[**get_scim_users**](SCIMApi.md#get_scim_users) | **GET** /api/v2/scim/users | Get a list of users
[**get_scim_v2_group**](SCIMApi.md#get_scim_v2_group) | **GET** /api/v2/scim/v2/groups/{groupId} | Get a group
[**get_scim_v2_groups**](SCIMApi.md#get_scim_v2_groups) | **GET** /api/v2/scim/v2/groups | Get a list of groups
[**get_scim_v2_resourcetype**](SCIMApi.md#get_scim_v2_resourcetype) | **GET** /api/v2/scim/v2/resourcetypes/{resourceType} | Get a resource type
[**get_scim_v2_resourcetypes**](SCIMApi.md#get_scim_v2_resourcetypes) | **GET** /api/v2/scim/v2/resourcetypes | Get a list of resource types
[**get_scim_v2_schema**](SCIMApi.md#get_scim_v2_schema) | **GET** /api/v2/scim/v2/schemas/{schemaId} | Get a SCIM schema
[**get_scim_v2_schemas**](SCIMApi.md#get_scim_v2_schemas) | **GET** /api/v2/scim/v2/schemas | Get a list of SCIM schemas
[**get_scim_v2_serviceproviderconfig**](SCIMApi.md#get_scim_v2_serviceproviderconfig) | **GET** /api/v2/scim/v2/serviceproviderconfig | Get a service provider's configuration
[**get_scim_v2_user**](SCIMApi.md#get_scim_v2_user) | **GET** /api/v2/scim/v2/users/{userId} | Get a user
[**get_scim_v2_users**](SCIMApi.md#get_scim_v2_users) | **GET** /api/v2/scim/v2/users | Get a list of users
[**patch_scim_group**](SCIMApi.md#patch_scim_group) | **PATCH** /api/v2/scim/groups/{groupId} | Modify a group
[**patch_scim_user**](SCIMApi.md#patch_scim_user) | **PATCH** /api/v2/scim/users/{userId} | Modify a user
[**patch_scim_v2_group**](SCIMApi.md#patch_scim_v2_group) | **PATCH** /api/v2/scim/v2/groups/{groupId} | Modify a group
[**patch_scim_v2_user**](SCIMApi.md#patch_scim_v2_user) | **PATCH** /api/v2/scim/v2/users/{userId} | Modify a user
[**post_scim_users**](SCIMApi.md#post_scim_users) | **POST** /api/v2/scim/users | Create a user
[**post_scim_v2_users**](SCIMApi.md#post_scim_v2_users) | **POST** /api/v2/scim/v2/users | Create a user
[**put_scim_group**](SCIMApi.md#put_scim_group) | **PUT** /api/v2/scim/groups/{groupId} | Replace a group
[**put_scim_user**](SCIMApi.md#put_scim_user) | **PUT** /api/v2/scim/users/{userId} | Replace a user
[**put_scim_v2_group**](SCIMApi.md#put_scim_v2_group) | **PUT** /api/v2/scim/v2/groups/{groupId} | Replace a group
[**put_scim_v2_user**](SCIMApi.md#put_scim_v2_user) | **PUT** /api/v2/scim/v2/users/{userId} | Replace a user



## delete_scim_user

> serde_json::Value delete_scim_user(user_id, if_match)
Delete a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/users. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scim_v2_user

> serde_json::Value delete_scim_v2_user(user_id, if_match)
Delete a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/v2/users. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_group

> crate::models::ScimV2Group get_scim_group(group_id, attributes, excluded_attributes, if_none_match)
Get a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of a group. Returned with GET /api/v2/scim/groups. | [required] |
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns \"id\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |
**if_none_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/groups/{groupId}. Example: \"42\". If the ETag is different from the version on the server, returns the current configuration of the resource. If the ETag is current, returns 304 Not Modified. |  |

### Return type

[**crate::models::ScimV2Group**](ScimV2Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_groups

> crate::models::ScimGroupListResponse get_scim_groups(start_index, count, attributes, excluded_attributes, filter)
Get a list of groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | The 1-based index of the first query result. |  |[default to 1]
**count** | Option<**i32**> | The requested number of items per page. A value of 0 returns \"totalResults\". A page size over 25 may exceed internal resource limits and return a 429 error. For a page size over 25, use the \"excludedAttributes\" or \"attributes\" query parameters to exclude or only include secondary lookup values such as \"externalId\",  \"roles\", \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingLanguages\", or \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingSkills\". |  |[default to 25]
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns \"id\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |
**filter** | Option<**String**> | Filters results. If nothing is specified, returns all groups. Examples of valid values: \"id eq 5f4bc742-a019-4e38-8e2a-d39d5bc0b0f3\", \"displayname eq Sales\". |  |

### Return type

[**crate::models::ScimGroupListResponse**](ScimGroupListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_resourcetype

> crate::models::ScimConfigResourceType get_scim_resourcetype(resource_type)
Get a resource type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_type** | **String** | The type of resource. Returned with GET /api/v2/scim/resourcetypes. | [required] |

### Return type

[**crate::models::ScimConfigResourceType**](ScimConfigResourceType.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_resourcetypes

> crate::models::ScimConfigResourceTypesListResponse get_scim_resourcetypes()
Get a list of resource types

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ScimConfigResourceTypesListResponse**](ScimConfigResourceTypesListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_schema

> crate::models::ScimV2SchemaDefinition get_scim_schema(schema_id)
Get a SCIM schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | The ID of a schema. Returned with GET /api/v2/scim/schemas. | [required] |

### Return type

[**crate::models::ScimV2SchemaDefinition**](ScimV2SchemaDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_schemas

> crate::models::ScimV2SchemaListResponse get_scim_schemas(filter)
Get a list of SCIM schemas

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filtered results are invalid and return 403 Unauthorized. |  |

### Return type

[**crate::models::ScimV2SchemaListResponse**](ScimV2SchemaListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_serviceproviderconfig

> crate::models::ScimServiceProviderConfig get_scim_serviceproviderconfig(if_none_match)
Get a service provider's configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**if_none_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/serviceproviderconfig. Example: \"42\". If the ETag is different from the version on the server, returns the current configuration of the resource. If the ETag is current, returns 304 Not Modified.  |  |

### Return type

[**crate::models::ScimServiceProviderConfig**](ScimServiceProviderConfig.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_user

> crate::models::ScimV2User get_scim_user(user_id, attributes, excluded_attributes, if_none_match)
Get a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/users. | [required] |
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |
**if_none_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns the current configuration of the resource. If the ETag is current, returns 304 Not Modified. |  |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_users

> crate::models::ScimUserListResponse get_scim_users(start_index, count, attributes, excluded_attributes, filter)
Get a list of users

To return all active users, do not use the filter parameter. To return inactive users, set the filter parameter to \"active eq false\". By default, returns SCIM attributes \"externalId\", \"enterprise-user:manager\", and \"roles\". To exclude these attributes, set the attributes parameter to \"id,active\" or the excludeAttributes parameter to \"externalId,roles,urn:ietf:params:scim:schemas:extension:enterprise:2.0:User:division\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | The 1-based index of the first query result. |  |[default to 1]
**count** | Option<**i32**> | The requested number of items per page. A value of 0 returns \"totalResults\". A page size over 25 may exceed internal resource limits and return a 429 error. For a page size over 25, use the \"excludedAttributes\" or \"attributes\" query parameters to exclude or only include secondary lookup values such as \"externalId\",  \"roles\", \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingLanguages\", or \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingSkills\". |  |[default to 25]
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |
**filter** | Option<**String**> | Filters results. If nothing is specified, returns all active users. Examples of valid values: \"id eq 857449b0-d9e7-4cd0-acbf-a6adfb9ef1e9\", \"userName eq search@sample.org\", \"manager eq 16e10e2f-1136-43fe-bb84-eac073168a49\", \"email eq search@sample.org\", \"division eq divisionName\", \"externalId eq 167844\", \"active eq false\", \"employeeNumber eq 9876543210\". |  |

### Return type

[**crate::models::ScimUserListResponse**](ScimUserListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_group

> crate::models::ScimV2Group get_scim_v2_group(group_id, attributes, excluded_attributes, if_none_match)
Get a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of a group. Returned with GET /api/v2/scim/v2/groups. | [required] |
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns \"id\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |
**if_none_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/groups/{groupId}. Example: \"42\". If the ETag is different from the version on the server, returns the current configuration of the resource. If the ETag is current, returns 304 Not Modified. |  |

### Return type

[**crate::models::ScimV2Group**](ScimV2Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_groups

> crate::models::ScimGroupListResponse get_scim_v2_groups(filter, start_index, count, attributes, excluded_attributes)
Get a list of groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | **String** | Filters results. If nothing is specified, returns all groups. Examples of valid values: \"id eq 5f4bc742-a019-4e38-8e2a-d39d5bc0b0f3\", \"displayname eq Sales\". | [required] |
**start_index** | Option<**i32**> | The 1-based index of the first query result. |  |[default to 1]
**count** | Option<**i32**> | The requested number of items per page. A value of 0 returns \"totalResults\". A page size over 25 may exceed internal resource limits and return a 429 error. For a page size over 25, use the \"excludedAttributes\" or \"attributes\" query parameters to exclude or only include secondary lookup values such as \"externalId\",  \"roles\", \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingLanguages\", or \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingSkills\". |  |[default to 25]
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns \"id\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |

### Return type

[**crate::models::ScimGroupListResponse**](ScimGroupListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_resourcetype

> crate::models::ScimConfigResourceType get_scim_v2_resourcetype(resource_type)
Get a resource type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_type** | **String** | The type of resource. Returned with GET /api/v2/scim/v2/resourcetypes. | [required] |

### Return type

[**crate::models::ScimConfigResourceType**](ScimConfigResourceType.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_resourcetypes

> crate::models::ScimConfigResourceTypesListResponse get_scim_v2_resourcetypes()
Get a list of resource types

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ScimConfigResourceTypesListResponse**](ScimConfigResourceTypesListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_schema

> crate::models::ScimV2SchemaDefinition get_scim_v2_schema(schema_id)
Get a SCIM schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | The ID of a schema. Returned with GET /api/v2/scim/v2/schemas. | [required] |

### Return type

[**crate::models::ScimV2SchemaDefinition**](ScimV2SchemaDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_schemas

> crate::models::ScimV2SchemaListResponse get_scim_v2_schemas(filter)
Get a list of SCIM schemas

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter** | Option<**String**> | Filtered results are invalid and return 403 Unauthorized. |  |

### Return type

[**crate::models::ScimV2SchemaListResponse**](ScimV2SchemaListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_serviceproviderconfig

> crate::models::ScimServiceProviderConfig get_scim_v2_serviceproviderconfig(if_none_match)
Get a service provider's configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**if_none_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/serviceproviderconfig. Example: \"42\". If the ETag is different from the version on the server, returns the current configuration of the resource. If the ETag is current, returns 304 Not Modified.  |  |

### Return type

[**crate::models::ScimServiceProviderConfig**](ScimServiceProviderConfig.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_user

> crate::models::ScimV2User get_scim_v2_user(user_id, attributes, excluded_attributes, if_none_match)
Get a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/v2/users. | [required] |
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |
**if_none_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns the current configuration of the resource. If the ETag is current, returns 304 Not Modified. |  |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scim_v2_users

> crate::models::ScimUserListResponse get_scim_v2_users(start_index, count, attributes, excluded_attributes, filter)
Get a list of users

To return all active users, do not use the filter parameter. To return inactive users, set the filter parameter to \"active eq false\". By default, returns SCIM attributes \"externalId\", \"enterprise-user:manager\", and \"roles\". To exclude these attributes, set the attributes parameter to \"id,active\" or the excludeAttributes parameter to \"externalId,roles,urn:ietf:params:scim:schemas:extension:enterprise:2.0:User:division\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_index** | Option<**i32**> | The 1-based index of the first query result. |  |[default to 1]
**count** | Option<**i32**> | The requested number of items per page. A value of 0 returns \"totalResults\". A page size over 25 may exceed internal resource limits and return a 429 error. For a page size over 25, use the \"excludedAttributes\" or \"attributes\" query parameters to exclude or only include secondary lookup values such as \"externalId\",  \"roles\", \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingLanguages\", or \"urn:ietf:params:scim:schemas:extension:genesys:purecloud:2.0:User:routingSkills\". |  |[default to 25]
**attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to include. Returns these attributes and the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"attributes\" to avoid expensive secondary calls for the default attributes. |  |
**excluded_attributes** | Option<[**Vec<String>**](String.md)> | Indicates which attributes to exclude. Returns the default attributes minus \"excludedAttributes\". Always returns the \"id\", \"userName\", \"active\", and \"meta\" attributes. Use \"excludedAttributes\" to avoid expensive secondary calls for the default attributes. |  |
**filter** | Option<**String**> | Filters results. If nothing is specified, returns all active users. Examples of valid values: \"id eq 857449b0-d9e7-4cd0-acbf-a6adfb9ef1e9\", \"userName eq search@sample.org\", \"manager eq 16e10e2f-1136-43fe-bb84-eac073168a49\", \"email eq search@sample.org\", \"division eq divisionName\", \"externalId eq 167844\", \"active eq false\", \"employeeNumber eq 9876543210\". |  |

### Return type

[**crate::models::ScimUserListResponse**](ScimUserListResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_scim_group

> crate::models::ScimV2Group patch_scim_group(group_id, body, if_match)
Modify a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of a group. Returned with GET /api/v2/scim/groups. | [required] |
**body** | [**ScimV2PatchRequest**](ScimV2PatchRequest.md) | The information used to modify a group. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/groups/{groupId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2Group**](ScimV2Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_scim_user

> crate::models::ScimV2User patch_scim_user(user_id, body, if_match)
Modify a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/users. | [required] |
**body** | [**ScimV2PatchRequest**](ScimV2PatchRequest.md) | The information used to modify a user. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_scim_v2_group

> crate::models::ScimV2Group patch_scim_v2_group(group_id, body, if_match)
Modify a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of a group. Returned with GET /api/v2/scim/v2/groups. | [required] |
**body** | [**ScimV2PatchRequest**](ScimV2PatchRequest.md) | The information used to modify a group. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/groups/{groupId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2Group**](ScimV2Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_scim_v2_user

> crate::models::ScimV2User patch_scim_v2_user(user_id, body, if_match)
Modify a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/v2/users. | [required] |
**body** | [**ScimV2PatchRequest**](ScimV2PatchRequest.md) | The information used to modify a user. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_scim_users

> crate::models::ScimV2User post_scim_users(body)
Create a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScimV2CreateUser**](ScimV2CreateUser.md) | The information used to create a user. | [required] |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_scim_v2_users

> crate::models::ScimV2User post_scim_v2_users(body)
Create a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScimV2CreateUser**](ScimV2CreateUser.md) | The information used to create a user. | [required] |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_scim_group

> crate::models::ScimV2Group put_scim_group(group_id, body, if_match)
Replace a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of a group. Returned with GET /api/v2/scim/groups. | [required] |
**body** | [**ScimV2Group**](ScimV2Group.md) | The information used to replace a group. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/groups/{groupId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2Group**](ScimV2Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_scim_user

> crate::models::ScimV2User put_scim_user(user_id, body, if_match)
Replace a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/users. | [required] |
**body** | [**ScimV2User**](ScimV2User.md) | The information used to replace a user. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_scim_v2_group

> crate::models::ScimV2Group put_scim_v2_group(group_id, body, if_match)
Replace a group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | The ID of a group. Returned with GET /api/v2/scim/v2/groups. | [required] |
**body** | [**ScimV2Group**](ScimV2Group.md) | The information used to replace a group. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/groups/{groupId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2Group**](ScimV2Group.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_scim_v2_user

> crate::models::ScimV2User put_scim_v2_user(user_id, body, if_match)
Replace a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The ID of a user. Returned with GET /api/v2/scim/v2/users. | [required] |
**body** | [**ScimV2User**](ScimV2User.md) | The information used to replace a user. | [required] |
**if_match** | Option<**String**> | The ETag of a resource in double quotes. Returned as header and meta.version with initial call to GET /api/v2/scim/v2/users/{userId}. Example: \"42\". If the ETag is different from the version on the server, returns 400 with a \"scimType\" of \"invalidVers\". |  |

### Return type

[**crate::models::ScimV2User**](ScimV2User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/scim+json, application/json
- **Accept**: application/scim+json, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

