# \UsersApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_analytics_users_details_job**](UsersApi.md#delete_analytics_users_details_job) | **DELETE** /api/v2/analytics/users/details/jobs/{jobId} | Delete/cancel an async request
[**delete_authorization_subject_division_role**](UsersApi.md#delete_authorization_subject_division_role) | **DELETE** /api/v2/authorization/subjects/{subjectId}/divisions/{divisionId}/roles/{roleId} | Delete a grant of a role in a division
[**delete_routing_user_utilization**](UsersApi.md#delete_routing_user_utilization) | **DELETE** /api/v2/routing/users/{userId}/utilization | Delete the user's max utilization settings and revert to the organization-wide default.
[**delete_user**](UsersApi.md#delete_user) | **DELETE** /api/v2/users/{userId} | Delete user
[**delete_user_routinglanguage**](UsersApi.md#delete_user_routinglanguage) | **DELETE** /api/v2/users/{userId}/routinglanguages/{languageId} | Remove routing language from user
[**delete_user_routingskill**](UsersApi.md#delete_user_routingskill) | **DELETE** /api/v2/users/{userId}/routingskills/{skillId} | Remove routing skill from user
[**delete_user_station_associatedstation**](UsersApi.md#delete_user_station_associatedstation) | **DELETE** /api/v2/users/{userId}/station/associatedstation | Clear associated station
[**delete_user_station_defaultstation**](UsersApi.md#delete_user_station_defaultstation) | **DELETE** /api/v2/users/{userId}/station/defaultstation | Clear default station
[**get_analytics_users_details_job**](UsersApi.md#get_analytics_users_details_job) | **GET** /api/v2/analytics/users/details/jobs/{jobId} | Get status for async query for user details
[**get_analytics_users_details_job_results**](UsersApi.md#get_analytics_users_details_job_results) | **GET** /api/v2/analytics/users/details/jobs/{jobId}/results | Fetch a page of results for an async query
[**get_analytics_users_details_jobs_availability**](UsersApi.md#get_analytics_users_details_jobs_availability) | **GET** /api/v2/analytics/users/details/jobs/availability | Lookup the datalake availability date and time
[**get_authorization_divisionspermitted_me**](UsersApi.md#get_authorization_divisionspermitted_me) | **GET** /api/v2/authorization/divisionspermitted/me | Returns which divisions the current user has the given permission in.
[**get_authorization_divisionspermitted_paged_me**](UsersApi.md#get_authorization_divisionspermitted_paged_me) | **GET** /api/v2/authorization/divisionspermitted/paged/me | Returns which divisions the current user has the given permission in.
[**get_authorization_divisionspermitted_paged_subject_id**](UsersApi.md#get_authorization_divisionspermitted_paged_subject_id) | **GET** /api/v2/authorization/divisionspermitted/paged/{subjectId} | Returns which divisions the specified user has the given permission in.
[**get_authorization_subject**](UsersApi.md#get_authorization_subject) | **GET** /api/v2/authorization/subjects/{subjectId} | Returns a listing of roles and permissions for a user.
[**get_authorization_subjects_me**](UsersApi.md#get_authorization_subjects_me) | **GET** /api/v2/authorization/subjects/me | Returns a listing of roles and permissions for the currently authenticated user.
[**get_fieldconfig**](UsersApi.md#get_fieldconfig) | **GET** /api/v2/fieldconfig | Fetch field config for an entity type
[**get_profiles_users**](UsersApi.md#get_profiles_users) | **GET** /api/v2/profiles/users | Get a user profile listing
[**get_routing_user_utilization**](UsersApi.md#get_routing_user_utilization) | **GET** /api/v2/routing/users/{userId}/utilization | Get the user's max utilization settings.  If not configured, the organization-wide default is returned.
[**get_user**](UsersApi.md#get_user) | **GET** /api/v2/users/{userId} | Get user.
[**get_user_adjacents**](UsersApi.md#get_user_adjacents) | **GET** /api/v2/users/{userId}/adjacents | Get adjacents
[**get_user_callforwarding**](UsersApi.md#get_user_callforwarding) | **GET** /api/v2/users/{userId}/callforwarding | Get a user's CallForwarding
[**get_user_directreports**](UsersApi.md#get_user_directreports) | **GET** /api/v2/users/{userId}/directreports | Get direct reports
[**get_user_favorites**](UsersApi.md#get_user_favorites) | **GET** /api/v2/users/{userId}/favorites | Deprecated; will be revived with new contract
[**get_user_geolocation**](UsersApi.md#get_user_geolocation) | **GET** /api/v2/users/{userId}/geolocations/{clientId} | Get a user's Geolocation
[**get_user_outofoffice**](UsersApi.md#get_user_outofoffice) | **GET** /api/v2/users/{userId}/outofoffice | Get a OutOfOffice
[**get_user_profile**](UsersApi.md#get_user_profile) | **GET** /api/v2/users/{userId}/profile | Get user profile
[**get_user_profileskills**](UsersApi.md#get_user_profileskills) | **GET** /api/v2/users/{userId}/profileskills | List profile skills for a user
[**get_user_queues**](UsersApi.md#get_user_queues) | **GET** /api/v2/users/{userId}/queues | Get queues for user
[**get_user_roles**](UsersApi.md#get_user_roles) | **GET** /api/v2/users/{userId}/roles | Returns a listing of roles and permissions for a user.
[**get_user_routinglanguages**](UsersApi.md#get_user_routinglanguages) | **GET** /api/v2/users/{userId}/routinglanguages | List routing language for user
[**get_user_routingskills**](UsersApi.md#get_user_routingskills) | **GET** /api/v2/users/{userId}/routingskills | List routing skills for user
[**get_user_routingstatus**](UsersApi.md#get_user_routingstatus) | **GET** /api/v2/users/{userId}/routingstatus | Fetch the routing status of a user
[**get_user_state**](UsersApi.md#get_user_state) | **GET** /api/v2/users/{userId}/state | Get user state information.
[**get_user_station**](UsersApi.md#get_user_station) | **GET** /api/v2/users/{userId}/station | Get station information for user
[**get_user_superiors**](UsersApi.md#get_user_superiors) | **GET** /api/v2/users/{userId}/superiors | Get superiors
[**get_user_trustors**](UsersApi.md#get_user_trustors) | **GET** /api/v2/users/{userId}/trustors | List the organizations that have authorized/trusted the user.
[**get_users**](UsersApi.md#get_users) | **GET** /api/v2/users | Get the list of available users.
[**get_users_development_activities**](UsersApi.md#get_users_development_activities) | **GET** /api/v2/users/development/activities | Get list of Development Activities
[**get_users_development_activities_me**](UsersApi.md#get_users_development_activities_me) | **GET** /api/v2/users/development/activities/me | Get list of Development Activities for current user
[**get_users_development_activity**](UsersApi.md#get_users_development_activity) | **GET** /api/v2/users/development/activities/{activityId} | Get a Development Activity
[**get_users_me**](UsersApi.md#get_users_me) | **GET** /api/v2/users/me | Get current user details.
[**get_users_search**](UsersApi.md#get_users_search) | **GET** /api/v2/users/search | Search users using the q64 value returned from a previous search
[**patch_user**](UsersApi.md#patch_user) | **PATCH** /api/v2/users/{userId} | Update user
[**patch_user_callforwarding**](UsersApi.md#patch_user_callforwarding) | **PATCH** /api/v2/users/{userId}/callforwarding | Patch a user's CallForwarding
[**patch_user_geolocation**](UsersApi.md#patch_user_geolocation) | **PATCH** /api/v2/users/{userId}/geolocations/{clientId} | Patch a user's Geolocation
[**patch_user_queue**](UsersApi.md#patch_user_queue) | **PATCH** /api/v2/users/{userId}/queues/{queueId} | Join or unjoin a queue for a user
[**patch_user_queues**](UsersApi.md#patch_user_queues) | **PATCH** /api/v2/users/{userId}/queues | Join or unjoin a set of queues for a user
[**patch_user_routinglanguage**](UsersApi.md#patch_user_routinglanguage) | **PATCH** /api/v2/users/{userId}/routinglanguages/{languageId} | Update routing language proficiency or state.
[**patch_user_routinglanguages_bulk**](UsersApi.md#patch_user_routinglanguages_bulk) | **PATCH** /api/v2/users/{userId}/routinglanguages/bulk | Add bulk routing language to user. Max limit 50 languages
[**patch_user_routingskills_bulk**](UsersApi.md#patch_user_routingskills_bulk) | **PATCH** /api/v2/users/{userId}/routingskills/bulk | Bulk add routing skills to user
[**patch_users_bulk**](UsersApi.md#patch_users_bulk) | **PATCH** /api/v2/users/bulk | Update bulk acd autoanswer on users
[**post_analytics_users_aggregates_query**](UsersApi.md#post_analytics_users_aggregates_query) | **POST** /api/v2/analytics/users/aggregates/query | Query for user aggregates
[**post_analytics_users_details_jobs**](UsersApi.md#post_analytics_users_details_jobs) | **POST** /api/v2/analytics/users/details/jobs | Query for user details asynchronously
[**post_analytics_users_details_query**](UsersApi.md#post_analytics_users_details_query) | **POST** /api/v2/analytics/users/details/query | Query for user details
[**post_analytics_users_observations_query**](UsersApi.md#post_analytics_users_observations_query) | **POST** /api/v2/analytics/users/observations/query | Query for user observations
[**post_authorization_subject_bulkadd**](UsersApi.md#post_authorization_subject_bulkadd) | **POST** /api/v2/authorization/subjects/{subjectId}/bulkadd | Bulk-grant roles and divisions to a subject.
[**post_authorization_subject_bulkremove**](UsersApi.md#post_authorization_subject_bulkremove) | **POST** /api/v2/authorization/subjects/{subjectId}/bulkremove | Bulk-remove grants from a subject.
[**post_authorization_subject_bulkreplace**](UsersApi.md#post_authorization_subject_bulkreplace) | **POST** /api/v2/authorization/subjects/{subjectId}/bulkreplace | Replace subject's roles and divisions with the exact list supplied in the request.
[**post_authorization_subject_division_role**](UsersApi.md#post_authorization_subject_division_role) | **POST** /api/v2/authorization/subjects/{subjectId}/divisions/{divisionId}/roles/{roleId} | Make a grant of a role in a division
[**post_user_invite**](UsersApi.md#post_user_invite) | **POST** /api/v2/users/{userId}/invite | Send an activation email to the user
[**post_user_password**](UsersApi.md#post_user_password) | **POST** /api/v2/users/{userId}/password | Change a users password
[**post_user_routinglanguages**](UsersApi.md#post_user_routinglanguages) | **POST** /api/v2/users/{userId}/routinglanguages | Add routing language to user
[**post_user_routingskills**](UsersApi.md#post_user_routingskills) | **POST** /api/v2/users/{userId}/routingskills | Add routing skill to user
[**post_users**](UsersApi.md#post_users) | **POST** /api/v2/users | Create user
[**post_users_development_activities_aggregates_query**](UsersApi.md#post_users_development_activities_aggregates_query) | **POST** /api/v2/users/development/activities/aggregates/query | Retrieve aggregated development activity data
[**post_users_me_password**](UsersApi.md#post_users_me_password) | **POST** /api/v2/users/me/password | Change your password
[**post_users_search**](UsersApi.md#post_users_search) | **POST** /api/v2/users/search | Search users
[**put_routing_user_utilization**](UsersApi.md#put_routing_user_utilization) | **PUT** /api/v2/routing/users/{userId}/utilization | Update the user's max utilization settings.  Include only those media types requiring custom configuration.
[**put_user_callforwarding**](UsersApi.md#put_user_callforwarding) | **PUT** /api/v2/users/{userId}/callforwarding | Update a user's CallForwarding
[**put_user_outofoffice**](UsersApi.md#put_user_outofoffice) | **PUT** /api/v2/users/{userId}/outofoffice | Update an OutOfOffice
[**put_user_profileskills**](UsersApi.md#put_user_profileskills) | **PUT** /api/v2/users/{userId}/profileskills | Update profile skills for a user
[**put_user_roles**](UsersApi.md#put_user_roles) | **PUT** /api/v2/users/{userId}/roles | Sets the user's roles
[**put_user_routingskill**](UsersApi.md#put_user_routingskill) | **PUT** /api/v2/users/{userId}/routingskills/{skillId} | Update routing skill proficiency or state.
[**put_user_routingskills_bulk**](UsersApi.md#put_user_routingskills_bulk) | **PUT** /api/v2/users/{userId}/routingskills/bulk | Replace all routing skills assigned to a user
[**put_user_routingstatus**](UsersApi.md#put_user_routingstatus) | **PUT** /api/v2/users/{userId}/routingstatus | Update the routing status of a user
[**put_user_state**](UsersApi.md#put_user_state) | **PUT** /api/v2/users/{userId}/state | Update user state information.
[**put_user_station_associatedstation_station_id**](UsersApi.md#put_user_station_associatedstation_station_id) | **PUT** /api/v2/users/{userId}/station/associatedstation/{stationId} | Set associated station
[**put_user_station_defaultstation_station_id**](UsersApi.md#put_user_station_defaultstation_station_id) | **PUT** /api/v2/users/{userId}/station/defaultstation/{stationId} | Set default station



## delete_analytics_users_details_job

> delete_analytics_users_details_job(job_id)
Delete/cancel an async request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

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


## delete_routing_user_utilization

> delete_routing_user_utilization(user_id)
Delete the user's max utilization settings and revert to the organization-wide default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user

> serde_json::Value delete_user(user_id)
Delete user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_routinglanguage

> delete_user_routinglanguage(user_id, language_id)
Remove routing language from user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**language_id** | **String** | languageId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_routingskill

> delete_user_routingskill(user_id, skill_id)
Remove routing skill from user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**skill_id** | **String** | skillId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_station_associatedstation

> delete_user_station_associatedstation(user_id)
Clear associated station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_station_defaultstation

> delete_user_station_defaultstation(user_id)
Clear default station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_users_details_job

> crate::models::AsyncQueryStatus get_analytics_users_details_job(job_id)
Get status for async query for user details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |

### Return type

[**crate::models::AsyncQueryStatus**](AsyncQueryStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_users_details_job_results

> crate::models::AnalyticsUserDetailsAsyncQueryResponse get_analytics_users_details_job_results(job_id, cursor, page_size)
Fetch a page of results for an async query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | jobId | [required] |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page) |  |
**page_size** | Option<**i32**> | The desired maximum number of results |  |

### Return type

[**crate::models::AnalyticsUserDetailsAsyncQueryResponse**](AnalyticsUserDetailsAsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_analytics_users_details_jobs_availability

> crate::models::DataAvailabilityResponse get_analytics_users_details_jobs_availability()
Lookup the datalake availability date and time

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataAvailabilityResponse**](DataAvailabilityResponse.md)

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


## get_profiles_users

> crate::models::UserProfileEntityListing get_profiles_users(page_size, page_number, id, jid, sort_order, expand, integration_presence_source)
Get a user profile listing

This api is deprecated. User /api/v2/users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**id** | Option<[**Vec<String>**](String.md)> | id |  |
**jid** | Option<[**Vec<String>**](String.md)> | jid |  |
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |
**integration_presence_source** | Option<**String**> | Gets an integration presence for users instead of their defaults. This parameter will only be used when presence is provided as an \"expand\". |  |

### Return type

[**crate::models::UserProfileEntityListing**](UserProfileEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_user_utilization

> crate::models::AgentMaxUtilization get_routing_user_utilization(user_id)
Get the user's max utilization settings.  If not configured, the organization-wide default is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::AgentMaxUtilization**](AgentMaxUtilization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> crate::models::User get_user(user_id, expand, integration_presence_source, state)
Get user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |
**integration_presence_source** | Option<**String**> | Gets an integration presence for a user instead of their default. |  |
**state** | Option<**String**> | Search for a user with this state |  |[default to active]

### Return type

[**crate::models::User**](User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_adjacents

> crate::models::Adjacents get_user_adjacents(user_id, expand)
Get adjacents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::Adjacents**](Adjacents.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_callforwarding

> crate::models::CallForwarding get_user_callforwarding(user_id)
Get a user's CallForwarding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::CallForwarding**](CallForwarding.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_directreports

> Vec<crate::models::User> get_user_directreports(user_id, expand)
Get direct reports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_favorites

> crate::models::UserEntityListing get_user_favorites(user_id, page_size, page_number, sort_order, expand)
Deprecated; will be revived with new contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::UserEntityListing**](UserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_geolocation

> crate::models::Geolocation get_user_geolocation(user_id, client_id)
Get a user's Geolocation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |
**client_id** | **String** | client Id | [required] |

### Return type

[**crate::models::Geolocation**](Geolocation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_outofoffice

> crate::models::OutOfOffice get_user_outofoffice(user_id)
Get a OutOfOffice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::OutOfOffice**](OutOfOffice.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_profile

> crate::models::UserProfile get_user_profile(user_id, expand, integration_presence_source)
Get user profile

This api has been deprecated. Use api/v2/users instead

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | userId | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |
**integration_presence_source** | Option<**String**> | Gets an integration presence for a user instead of their default. |  |

### Return type

[**crate::models::UserProfile**](UserProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_profileskills

> Vec<String> get_user_profileskills(user_id)
List profile skills for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

**Vec<String>**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_queues

> crate::models::UserQueueEntityListing get_user_queues(user_id, page_size, page_number, joined, division_id)
Get queues for user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**joined** | Option<**bool**> | Is joined to the queue |  |[default to true]
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |

### Return type

[**crate::models::UserQueueEntityListing**](UserQueueEntityListing.md)

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


## get_user_routinglanguages

> crate::models::UserLanguageEntityListing get_user_routinglanguages(user_id, page_size, page_number, sort_order)
List routing language for user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]

### Return type

[**crate::models::UserLanguageEntityListing**](UserLanguageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_routingskills

> crate::models::UserSkillEntityListing get_user_routingskills(user_id, page_size, page_number, sort_order)
List routing skills for user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]

### Return type

[**crate::models::UserSkillEntityListing**](UserSkillEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_routingstatus

> crate::models::RoutingStatus get_user_routingstatus(user_id)
Fetch the routing status of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::RoutingStatus**](RoutingStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_state

> crate::models::UserState get_user_state(user_id)
Get user state information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::UserState**](UserState.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_station

> crate::models::UserStations get_user_station(user_id)
Get station information for user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |

### Return type

[**crate::models::UserStations**](UserStations.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_superiors

> Vec<crate::models::User> get_user_superiors(user_id, expand)
Get superiors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**Vec<crate::models::User>**](User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_trustors

> crate::models::TrustorEntityListing get_user_trustors(user_id, page_size, page_number)
List the organizations that have authorized/trusted the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
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


## get_users

> crate::models::UserEntityListing get_users(page_size, page_number, id, jabber_id, sort_order, expand, integration_presence_source, state)
Get the list of available users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**id** | Option<[**Vec<String>**](String.md)> | A list of user IDs to fetch by bulk |  |
**jabber_id** | Option<[**Vec<String>**](String.md)> | A list of jabberIds to fetch by bulk (cannot be used with the \"id\" parameter) |  |
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |
**integration_presence_source** | Option<**String**> | Gets an integration presence for users instead of their defaults. This parameter will only be used when presence is provided as an \"expand\". When using this parameter the maximum number of users that can be returned is 100. |  |
**state** | Option<**String**> | Only list users of this state |  |[default to active]

### Return type

[**crate::models::UserEntityListing**](UserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_development_activities

> crate::models::DevelopmentActivityListing get_users_development_activities(user_id, module_id, interval, completion_interval, overdue, page_size, page_number, sort_order, types, statuses, relationship)
Get list of Development Activities

Either moduleId or userId is required. Results are filtered based on the applicable permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<[**Vec<String>**](String.md)> | Specifies the list of user IDs to be queried, up to 100 user IDs. It searches for any relationship for the userId. |  |
**module_id** | Option<**String**> | Specifies the ID of the learning module. |  |
**interval** | Option<**String**> | Specifies the dateDue range to be queried. Milliseconds will be truncated. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**completion_interval** | Option<**String**> | Specifies the range of completion dates to be used for filtering. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**overdue** | Option<**String**> | Specifies if non-overdue, overdue, or all activities are returned. If not specified, all activities are returned |  |[default to Any]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Specifies result set sort order sorted by the date due; if not specified, default sort order is descending (Desc) |  |[default to Desc]
**types** | Option<[**Vec<String>**](String.md)> | Specifies the activity types. |  |
**statuses** | Option<[**Vec<String>**](String.md)> | Specifies the activity statuses to filter by |  |
**relationship** | Option<[**Vec<String>**](String.md)> | Specifies how the current user relation should be interpreted, and filters the activities returned to only the activities that have the specified relationship. If a value besides Attendee is specified, it will only return Coaching Appointments. If not specified, no filtering is applied. |  |

### Return type

[**crate::models::DevelopmentActivityListing**](DevelopmentActivityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_development_activities_me

> crate::models::DevelopmentActivityListing get_users_development_activities_me(module_id, interval, completion_interval, overdue, page_size, page_number, sort_order, types, statuses, relationship)
Get list of Development Activities for current user

Results are filtered based on the applicable permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_id** | Option<**String**> | Specifies the ID of the learning module. |  |
**interval** | Option<**String**> | Specifies the dateDue range to be queried. Milliseconds will be truncated. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**completion_interval** | Option<**String**> | Specifies the range of completion dates to be used for filtering. A maximum of 1 year can be specified in the range. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**overdue** | Option<**String**> | Specifies if non-overdue, overdue, or all activities are returned. If not specified, all activities are returned |  |[default to Any]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Specifies result set sort order sorted by the date due; if not specified, default sort order is descending (Desc) |  |[default to Desc]
**types** | Option<[**Vec<String>**](String.md)> | Specifies the activity types. |  |
**statuses** | Option<[**Vec<String>**](String.md)> | Specifies the activity statuses to filter by |  |
**relationship** | Option<[**Vec<String>**](String.md)> | Specifies how the current user relation should be interpreted, and filters the activities returned to only the activities that have the specified relationship. If a value besides Attendee is specified, it will only return Coaching Appointments. If not specified, no filtering is applied. |  |

### Return type

[**crate::models::DevelopmentActivityListing**](DevelopmentActivityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_development_activity

> crate::models::DevelopmentActivity get_users_development_activity(activity_id, _type)
Get a Development Activity

Permission not required if you are the attendee, creator or facilitator of the coaching appointment or you are the assigned user of the learning assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_id** | **String** | Specifies the activity ID, maps to either assignment or appointment ID | [required] |
**_type** | **String** | Specifies the activity type. | [required] |

### Return type

[**crate::models::DevelopmentActivity**](DevelopmentActivity.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_me

> crate::models::UserMe get_users_me(expand, integration_presence_source)
Get current user details.

This request is not valid when using the Client Credentials OAuth grant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |
**integration_presence_source** | Option<**String**> | Get your presence for a given integration. This parameter will only be used when presence is provided as an \"expand\". |  |

### Return type

[**crate::models::UserMe**](UserMe.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_search

> crate::models::UsersSearchResponse get_users_search(q64, expand, integration_presence_source)
Search users using the q64 value returned from a previous search

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q64** | **String** | q64 | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | expand |  |
**integration_presence_source** | Option<**String**> | integrationPresenceSource |  |

### Return type

[**crate::models::UsersSearchResponse**](UsersSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user

> crate::models::User patch_user(user_id, body)
Update user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**UpdateUser**](UpdateUser.md) | User | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_callforwarding

> crate::models::CallForwarding patch_user_callforwarding(user_id, body)
Patch a user's CallForwarding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**CallForwarding**](CallForwarding.md) | Call forwarding | [required] |

### Return type

[**crate::models::CallForwarding**](CallForwarding.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_geolocation

> crate::models::Geolocation patch_user_geolocation(user_id, client_id, body)
Patch a user's Geolocation

The geolocation object can be patched one of three ways. Option 1: Set the 'primary' property to true. This will set the client as the user's primary geolocation source.  Option 2: Provide the 'latitude' and 'longitude' values.  This will enqueue an asynchronous update of the 'city', 'region', and 'country', generating a notification. A subsequent GET operation will include the new values for 'city', 'region' and 'country'.  Option 3:  Provide the 'city', 'region', 'country' values.  Option 1 can be combined with Option 2 or Option 3.  For example, update the client as primary and provide latitude and longitude values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | user Id | [required] |
**client_id** | **String** | client Id | [required] |
**body** | [**Geolocation**](Geolocation.md) | Geolocation | [required] |

### Return type

[**crate::models::Geolocation**](Geolocation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_queue

> crate::models::UserQueue patch_user_queue(queue_id, user_id, body)
Join or unjoin a queue for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**user_id** | **String** | User ID | [required] |
**body** | [**UserQueue**](UserQueue.md) | Queue Member | [required] |

### Return type

[**crate::models::UserQueue**](UserQueue.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_queues

> crate::models::UserQueueEntityListing patch_user_queues(user_id, body, division_id)
Join or unjoin a set of queues for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Vec<crate::models::UserQueue>**](UserQueue.md) | User Queues | [required] |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |

### Return type

[**crate::models::UserQueueEntityListing**](UserQueueEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_routinglanguage

> crate::models::UserRoutingLanguage patch_user_routinglanguage(user_id, language_id, body)
Update routing language proficiency or state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**language_id** | **String** | languageId | [required] |
**body** | [**UserRoutingLanguage**](UserRoutingLanguage.md) | Language | [required] |

### Return type

[**crate::models::UserRoutingLanguage**](UserRoutingLanguage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_routinglanguages_bulk

> crate::models::UserLanguageEntityListing patch_user_routinglanguages_bulk(user_id, body)
Add bulk routing language to user. Max limit 50 languages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Vec<crate::models::UserRoutingLanguagePost>**](UserRoutingLanguagePost.md) | Language | [required] |

### Return type

[**crate::models::UserLanguageEntityListing**](UserLanguageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_user_routingskills_bulk

> crate::models::UserSkillEntityListing patch_user_routingskills_bulk(user_id, body)
Bulk add routing skills to user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Vec<crate::models::UserRoutingSkillPost>**](UserRoutingSkillPost.md) | Skill | [required] |

### Return type

[**crate::models::UserSkillEntityListing**](UserSkillEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_users_bulk

> crate::models::UserEntityListing patch_users_bulk(body)
Update bulk acd autoanswer on users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::PatchUser>**](PatchUser.md) | Users | [required] |

### Return type

[**crate::models::UserEntityListing**](UserEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_aggregates_query

> crate::models::UserAggregateQueryResponse post_analytics_users_aggregates_query(body)
Query for user aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserAggregationQuery**](UserAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::UserAggregateQueryResponse**](UserAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_details_jobs

> crate::models::AsyncQueryResponse post_analytics_users_details_jobs(body)
Query for user details asynchronously

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**AsyncUserDetailsQuery**](AsyncUserDetailsQuery.md) | query | [required] |

### Return type

[**crate::models::AsyncQueryResponse**](AsyncQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_details_query

> crate::models::AnalyticsUserDetailsQueryResponse post_analytics_users_details_query(body)
Query for user details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserDetailsQuery**](UserDetailsQuery.md) | query | [required] |

### Return type

[**crate::models::AnalyticsUserDetailsQueryResponse**](AnalyticsUserDetailsQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_users_observations_query

> crate::models::UserObservationQueryResponse post_analytics_users_observations_query(body)
Query for user observations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserObservationQuery**](UserObservationQuery.md) | query | [required] |

### Return type

[**crate::models::UserObservationQueryResponse**](UserObservationQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
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


## post_user_invite

> post_user_invite(user_id, force)
Send an activation email to the user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**force** | Option<**bool**> | Resend the invitation even if one is already outstanding |  |[default to false]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_password

> post_user_password(user_id, body)
Change a users password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**ChangePasswordRequest**](ChangePasswordRequest.md) | Password | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_routinglanguages

> crate::models::UserRoutingLanguage post_user_routinglanguages(user_id, body)
Add routing language to user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**UserRoutingLanguagePost**](UserRoutingLanguagePost.md) | Language | [required] |

### Return type

[**crate::models::UserRoutingLanguage**](UserRoutingLanguage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_user_routingskills

> crate::models::UserRoutingSkill post_user_routingskills(user_id, body)
Add routing skill to user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**UserRoutingSkillPost**](UserRoutingSkillPost.md) | Skill | [required] |

### Return type

[**crate::models::UserRoutingSkill**](UserRoutingSkill.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users

> crate::models::User post_users(body)
Create user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateUser**](CreateUser.md) | User | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_development_activities_aggregates_query

> crate::models::DevelopmentActivityAggregateResponse post_users_development_activities_aggregates_query(body)
Retrieve aggregated development activity data

Results are filtered based on the applicable permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DevelopmentActivityAggregateParam**](DevelopmentActivityAggregateParam.md) | Aggregate Request | [required] |

### Return type

[**crate::models::DevelopmentActivityAggregateResponse**](DevelopmentActivityAggregateResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_me_password

> post_users_me_password(body)
Change your password

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ChangeMyPasswordRequest**](ChangeMyPasswordRequest.md) | Password | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_users_search

> crate::models::UsersSearchResponse post_users_search(body)
Search users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**UserSearchRequest**](UserSearchRequest.md) | Search request options | [required] |

### Return type

[**crate::models::UsersSearchResponse**](UsersSearchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_routing_user_utilization

> crate::models::AgentMaxUtilization put_routing_user_utilization(user_id, body)
Update the user's max utilization settings.  Include only those media types requiring custom configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Utilization**](Utilization.md) | utilization | [required] |

### Return type

[**crate::models::AgentMaxUtilization**](AgentMaxUtilization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_callforwarding

> crate::models::CallForwarding put_user_callforwarding(user_id, body)
Update a user's CallForwarding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**CallForwarding**](CallForwarding.md) | Call forwarding | [required] |

### Return type

[**crate::models::CallForwarding**](CallForwarding.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_outofoffice

> crate::models::OutOfOffice put_user_outofoffice(user_id, body)
Update an OutOfOffice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**OutOfOffice**](OutOfOffice.md) | The updated OutOffOffice | [required] |

### Return type

[**crate::models::OutOfOffice**](OutOfOffice.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_profileskills

> Vec<String> put_user_profileskills(user_id, body)
Update profile skills for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Vec<String>**](String.md) | Skills | [required] |

### Return type

**Vec<String>**

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


## put_user_routingskill

> crate::models::UserRoutingSkill put_user_routingskill(user_id, skill_id, body)
Update routing skill proficiency or state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**skill_id** | **String** | skillId | [required] |
**body** | [**UserRoutingSkill**](UserRoutingSkill.md) | Skill | [required] |

### Return type

[**crate::models::UserRoutingSkill**](UserRoutingSkill.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_routingskills_bulk

> crate::models::UserSkillEntityListing put_user_routingskills_bulk(user_id, body)
Replace all routing skills assigned to a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**Vec<crate::models::UserRoutingSkillPost>**](UserRoutingSkillPost.md) | Skill | [required] |

### Return type

[**crate::models::UserSkillEntityListing**](UserSkillEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_routingstatus

> crate::models::RoutingStatus put_user_routingstatus(user_id, body)
Update the routing status of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**RoutingStatus**](RoutingStatus.md) | Routing Status | [required] |

### Return type

[**crate::models::RoutingStatus**](RoutingStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_state

> crate::models::UserState put_user_state(user_id, body)
Update user state information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**body** | [**UserState**](UserState.md) | User | [required] |

### Return type

[**crate::models::UserState**](UserState.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_station_associatedstation_station_id

> put_user_station_associatedstation_station_id(user_id, station_id)
Set associated station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**station_id** | **String** | stationId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_user_station_defaultstation_station_id

> put_user_station_defaultstation_station_id(user_id, station_id)
Set default station

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | User ID | [required] |
**station_id** | **String** | stationId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

