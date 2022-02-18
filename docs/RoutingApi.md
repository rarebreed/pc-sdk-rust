# \RoutingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_routing_assessment**](RoutingApi.md#delete_routing_assessment) | **DELETE** /api/v2/routing/assessments/{assessmentId} | Delete single benefit assessment.
[**delete_routing_email_domain**](RoutingApi.md#delete_routing_email_domain) | **DELETE** /api/v2/routing/email/domains/{domainId} | Delete a domain
[**delete_routing_email_domain_route**](RoutingApi.md#delete_routing_email_domain_route) | **DELETE** /api/v2/routing/email/domains/{domainName}/routes/{routeId} | Delete a route
[**delete_routing_predictor**](RoutingApi.md#delete_routing_predictor) | **DELETE** /api/v2/routing/predictors/{predictorId} | Delete single predictor.
[**delete_routing_queue**](RoutingApi.md#delete_routing_queue) | **DELETE** /api/v2/routing/queues/{queueId} | Delete a queue
[**delete_routing_queue_member**](RoutingApi.md#delete_routing_queue_member) | **DELETE** /api/v2/routing/queues/{queueId}/members/{memberId} | Delete a queue member.
[**delete_routing_queue_user**](RoutingApi.md#delete_routing_queue_user) | **DELETE** /api/v2/routing/queues/{queueId}/users/{memberId} | DEPRECATED: use DELETE /routing/queues/{queueId}/members/{memberId}.  Delete queue member.
[**delete_routing_queue_wrapupcode**](RoutingApi.md#delete_routing_queue_wrapupcode) | **DELETE** /api/v2/routing/queues/{queueId}/wrapupcodes/{codeId} | Delete a wrap-up code from a queue
[**delete_routing_settings**](RoutingApi.md#delete_routing_settings) | **DELETE** /api/v2/routing/settings | Delete an organization's routing settings
[**delete_routing_skill**](RoutingApi.md#delete_routing_skill) | **DELETE** /api/v2/routing/skills/{skillId} | Delete Routing Skill
[**delete_routing_sms_address**](RoutingApi.md#delete_routing_sms_address) | **DELETE** /api/v2/routing/sms/addresses/{addressId} | Delete an Address by Id for SMS
[**delete_routing_sms_phonenumber**](RoutingApi.md#delete_routing_sms_phonenumber) | **DELETE** /api/v2/routing/sms/phonenumbers/{addressId} | Delete a phone number provisioned for SMS.
[**delete_routing_user_utilization**](RoutingApi.md#delete_routing_user_utilization) | **DELETE** /api/v2/routing/users/{userId}/utilization | Delete the user's max utilization settings and revert to the organization-wide default.
[**delete_routing_utilization**](RoutingApi.md#delete_routing_utilization) | **DELETE** /api/v2/routing/utilization | Delete the organization-wide max utilization settings and revert to the system default.
[**delete_routing_wrapupcode**](RoutingApi.md#delete_routing_wrapupcode) | **DELETE** /api/v2/routing/wrapupcodes/{codeId} | Delete wrap-up code
[**delete_user_routinglanguage**](RoutingApi.md#delete_user_routinglanguage) | **DELETE** /api/v2/users/{userId}/routinglanguages/{languageId} | Remove routing language from user
[**delete_user_routingskill**](RoutingApi.md#delete_user_routingskill) | **DELETE** /api/v2/users/{userId}/routingskills/{skillId} | Remove routing skill from user
[**get_routing_assessment**](RoutingApi.md#get_routing_assessment) | **GET** /api/v2/routing/assessments/{assessmentId} | Retrieve a single benefit assessment.
[**get_routing_assessments**](RoutingApi.md#get_routing_assessments) | **GET** /api/v2/routing/assessments | Retrieve all benefit assessments.
[**get_routing_assessments_job**](RoutingApi.md#get_routing_assessments_job) | **GET** /api/v2/routing/assessments/jobs/{jobId} | Retrieve a single benefit assessments job.
[**get_routing_assessments_jobs**](RoutingApi.md#get_routing_assessments_jobs) | **GET** /api/v2/routing/assessments/jobs | Retrieve all benefit assessment jobs.
[**get_routing_email_domain**](RoutingApi.md#get_routing_email_domain) | **GET** /api/v2/routing/email/domains/{domainId} | Get domain
[**get_routing_email_domain_route**](RoutingApi.md#get_routing_email_domain_route) | **GET** /api/v2/routing/email/domains/{domainName}/routes/{routeId} | Get a route
[**get_routing_email_domain_routes**](RoutingApi.md#get_routing_email_domain_routes) | **GET** /api/v2/routing/email/domains/{domainName}/routes | Get routes
[**get_routing_email_domains**](RoutingApi.md#get_routing_email_domains) | **GET** /api/v2/routing/email/domains | Get domains
[**get_routing_email_setup**](RoutingApi.md#get_routing_email_setup) | **GET** /api/v2/routing/email/setup | Get email setup
[**get_routing_languages**](RoutingApi.md#get_routing_languages) | **GET** /api/v2/routing/languages | Get the list of supported languages.
[**get_routing_message_recipient**](RoutingApi.md#get_routing_message_recipient) | **GET** /api/v2/routing/message/recipients/{recipientId} | Get a recipient
[**get_routing_message_recipients**](RoutingApi.md#get_routing_message_recipients) | **GET** /api/v2/routing/message/recipients | Get recipients
[**get_routing_predictor**](RoutingApi.md#get_routing_predictor) | **GET** /api/v2/routing/predictors/{predictorId} | Retrieve a single predictor.
[**get_routing_predictors**](RoutingApi.md#get_routing_predictors) | **GET** /api/v2/routing/predictors | Retrieve all predictors.
[**get_routing_predictors_keyperformanceindicators**](RoutingApi.md#get_routing_predictors_keyperformanceindicators) | **GET** /api/v2/routing/predictors/keyperformanceindicators | Get a list of Key Performance Indicators available for the predictors.
[**get_routing_queue**](RoutingApi.md#get_routing_queue) | **GET** /api/v2/routing/queues/{queueId} | Get details about this queue.
[**get_routing_queue_comparisonperiod**](RoutingApi.md#get_routing_queue_comparisonperiod) | **GET** /api/v2/routing/queues/{queueId}/comparisonperiods/{comparisonPeriodId} | Get a Comparison Period.
[**get_routing_queue_comparisonperiods**](RoutingApi.md#get_routing_queue_comparisonperiods) | **GET** /api/v2/routing/queues/{queueId}/comparisonperiods | Get list of comparison periods
[**get_routing_queue_estimatedwaittime**](RoutingApi.md#get_routing_queue_estimatedwaittime) | **GET** /api/v2/routing/queues/{queueId}/estimatedwaittime | Get Estimated Wait Time
[**get_routing_queue_mediatype_estimatedwaittime**](RoutingApi.md#get_routing_queue_mediatype_estimatedwaittime) | **GET** /api/v2/routing/queues/{queueId}/mediatypes/{mediaType}/estimatedwaittime | Get Estimated Wait Time
[**get_routing_queue_members**](RoutingApi.md#get_routing_queue_members) | **GET** /api/v2/routing/queues/{queueId}/members | Get the members of this queue.
[**get_routing_queue_users**](RoutingApi.md#get_routing_queue_users) | **GET** /api/v2/routing/queues/{queueId}/users | DEPRECATED: use GET /routing/queues/{queueId}/members.  Get the members of this queue.
[**get_routing_queue_wrapupcodes**](RoutingApi.md#get_routing_queue_wrapupcodes) | **GET** /api/v2/routing/queues/{queueId}/wrapupcodes | Get the wrap-up codes for a queue
[**get_routing_queues**](RoutingApi.md#get_routing_queues) | **GET** /api/v2/routing/queues | Get list of queues.
[**get_routing_queues_divisionviews**](RoutingApi.md#get_routing_queues_divisionviews) | **GET** /api/v2/routing/queues/divisionviews | Get a paged listing of simplified queue objects, filterable by name, queue ID(s), or division ID(s).
[**get_routing_queues_divisionviews_all**](RoutingApi.md#get_routing_queues_divisionviews_all) | **GET** /api/v2/routing/queues/divisionviews/all | Get a paged listing of simplified queue objects, sorted by name.  Can be used to get a digest of all queues in an organization.
[**get_routing_queues_me**](RoutingApi.md#get_routing_queues_me) | **GET** /api/v2/routing/queues/me | Get a paged listing of queues the user is a member of.
[**get_routing_settings**](RoutingApi.md#get_routing_settings) | **GET** /api/v2/routing/settings | Get an organization's routing settings
[**get_routing_settings_contactcenter**](RoutingApi.md#get_routing_settings_contactcenter) | **GET** /api/v2/routing/settings/contactcenter | Get Contact Center Settings
[**get_routing_settings_transcription**](RoutingApi.md#get_routing_settings_transcription) | **GET** /api/v2/routing/settings/transcription | Get Transcription Settings
[**get_routing_skill**](RoutingApi.md#get_routing_skill) | **GET** /api/v2/routing/skills/{skillId} | Get Routing Skill
[**get_routing_skills**](RoutingApi.md#get_routing_skills) | **GET** /api/v2/routing/skills | Get the list of routing skills.
[**get_routing_sms_address**](RoutingApi.md#get_routing_sms_address) | **GET** /api/v2/routing/sms/addresses/{addressId} | Get an Address by Id for SMS
[**get_routing_sms_addresses**](RoutingApi.md#get_routing_sms_addresses) | **GET** /api/v2/routing/sms/addresses | Get a list of Addresses for SMS
[**get_routing_sms_availablephonenumbers**](RoutingApi.md#get_routing_sms_availablephonenumbers) | **GET** /api/v2/routing/sms/availablephonenumbers | Get a list of available phone numbers for SMS provisioning.
[**get_routing_sms_phonenumber**](RoutingApi.md#get_routing_sms_phonenumber) | **GET** /api/v2/routing/sms/phonenumbers/{addressId} | Get a phone number provisioned for SMS.
[**get_routing_sms_phonenumbers**](RoutingApi.md#get_routing_sms_phonenumbers) | **GET** /api/v2/routing/sms/phonenumbers | Get a list of provisioned phone numbers.
[**get_routing_user_utilization**](RoutingApi.md#get_routing_user_utilization) | **GET** /api/v2/routing/users/{userId}/utilization | Get the user's max utilization settings.  If not configured, the organization-wide default is returned.
[**get_routing_utilization**](RoutingApi.md#get_routing_utilization) | **GET** /api/v2/routing/utilization | Get the organization-wide max utilization settings.
[**get_routing_wrapupcode**](RoutingApi.md#get_routing_wrapupcode) | **GET** /api/v2/routing/wrapupcodes/{codeId} | Get details about this wrap-up code.
[**get_routing_wrapupcodes**](RoutingApi.md#get_routing_wrapupcodes) | **GET** /api/v2/routing/wrapupcodes | Get list of wrapup codes.
[**get_user_queues**](RoutingApi.md#get_user_queues) | **GET** /api/v2/users/{userId}/queues | Get queues for user
[**get_user_routinglanguages**](RoutingApi.md#get_user_routinglanguages) | **GET** /api/v2/users/{userId}/routinglanguages | List routing language for user
[**get_user_routingskills**](RoutingApi.md#get_user_routingskills) | **GET** /api/v2/users/{userId}/routingskills | List routing skills for user
[**patch_routing_conversation**](RoutingApi.md#patch_routing_conversation) | **PATCH** /api/v2/routing/conversations/{conversationId} | Update attributes of an in-queue conversation
[**patch_routing_email_domain**](RoutingApi.md#patch_routing_email_domain) | **PATCH** /api/v2/routing/email/domains/{domainId} | Update domain settings
[**patch_routing_email_domain_validate**](RoutingApi.md#patch_routing_email_domain_validate) | **PATCH** /api/v2/routing/email/domains/{domainId}/validate | Validate domain settings
[**patch_routing_predictor**](RoutingApi.md#patch_routing_predictor) | **PATCH** /api/v2/routing/predictors/{predictorId} | Update single predictor.
[**patch_routing_queue_member**](RoutingApi.md#patch_routing_queue_member) | **PATCH** /api/v2/routing/queues/{queueId}/members/{memberId} | Update the ring number OR joined status for a queue member.
[**patch_routing_queue_members**](RoutingApi.md#patch_routing_queue_members) | **PATCH** /api/v2/routing/queues/{queueId}/members | Join or unjoin a set of users for a queue
[**patch_routing_queue_user**](RoutingApi.md#patch_routing_queue_user) | **PATCH** /api/v2/routing/queues/{queueId}/users/{memberId} | DEPRECATED: use PATCH /routing/queues/{queueId}/members/{memberId}.  Update the ring number OR joined status for a User in a Queue.
[**patch_routing_queue_users**](RoutingApi.md#patch_routing_queue_users) | **PATCH** /api/v2/routing/queues/{queueId}/users | DEPRECATED: use PATCH /routing/queues/{queueId}/members.  Join or unjoin a set of users for a queue.
[**patch_routing_settings_contactcenter**](RoutingApi.md#patch_routing_settings_contactcenter) | **PATCH** /api/v2/routing/settings/contactcenter | Update Contact Center Settings
[**patch_user_queue**](RoutingApi.md#patch_user_queue) | **PATCH** /api/v2/users/{userId}/queues/{queueId} | Join or unjoin a queue for a user
[**patch_user_queues**](RoutingApi.md#patch_user_queues) | **PATCH** /api/v2/users/{userId}/queues | Join or unjoin a set of queues for a user
[**patch_user_routinglanguage**](RoutingApi.md#patch_user_routinglanguage) | **PATCH** /api/v2/users/{userId}/routinglanguages/{languageId} | Update routing language proficiency or state.
[**patch_user_routinglanguages_bulk**](RoutingApi.md#patch_user_routinglanguages_bulk) | **PATCH** /api/v2/users/{userId}/routinglanguages/bulk | Add bulk routing language to user. Max limit 50 languages
[**patch_user_routingskills_bulk**](RoutingApi.md#patch_user_routingskills_bulk) | **PATCH** /api/v2/users/{userId}/routingskills/bulk | Bulk add routing skills to user
[**post_analytics_queues_observations_query**](RoutingApi.md#post_analytics_queues_observations_query) | **POST** /api/v2/analytics/queues/observations/query | Query for queue observations
[**post_routing_assessments**](RoutingApi.md#post_routing_assessments) | **POST** /api/v2/routing/assessments | Create a benefit assessment.
[**post_routing_assessments_jobs**](RoutingApi.md#post_routing_assessments_jobs) | **POST** /api/v2/routing/assessments/jobs | Create a benefit assessment job.
[**post_routing_email_domain_routes**](RoutingApi.md#post_routing_email_domain_routes) | **POST** /api/v2/routing/email/domains/{domainName}/routes | Create a route
[**post_routing_email_domain_testconnection**](RoutingApi.md#post_routing_email_domain_testconnection) | **POST** /api/v2/routing/email/domains/{domainId}/testconnection | Tests the custom SMTP server integration connection set on this domain
[**post_routing_email_domains**](RoutingApi.md#post_routing_email_domains) | **POST** /api/v2/routing/email/domains | Create a domain
[**post_routing_languages**](RoutingApi.md#post_routing_languages) | **POST** /api/v2/routing/languages | Create Language
[**post_routing_predictors**](RoutingApi.md#post_routing_predictors) | **POST** /api/v2/routing/predictors | Create a predictor.
[**post_routing_queue_members**](RoutingApi.md#post_routing_queue_members) | **POST** /api/v2/routing/queues/{queueId}/members | Bulk add or delete up to 100 queue members
[**post_routing_queue_users**](RoutingApi.md#post_routing_queue_users) | **POST** /api/v2/routing/queues/{queueId}/users | DEPRECATED: use POST /routing/queues/{queueId}/members.  Bulk add or delete up to 100 queue members.
[**post_routing_queue_wrapupcodes**](RoutingApi.md#post_routing_queue_wrapupcodes) | **POST** /api/v2/routing/queues/{queueId}/wrapupcodes | Add up to 100 wrap-up codes to a queue
[**post_routing_queues**](RoutingApi.md#post_routing_queues) | **POST** /api/v2/routing/queues | Create a queue
[**post_routing_skills**](RoutingApi.md#post_routing_skills) | **POST** /api/v2/routing/skills | Create Skill
[**post_routing_sms_addresses**](RoutingApi.md#post_routing_sms_addresses) | **POST** /api/v2/routing/sms/addresses | Provision an Address for SMS
[**post_routing_sms_phonenumbers**](RoutingApi.md#post_routing_sms_phonenumbers) | **POST** /api/v2/routing/sms/phonenumbers | Provision a phone number for SMS
[**post_routing_wrapupcodes**](RoutingApi.md#post_routing_wrapupcodes) | **POST** /api/v2/routing/wrapupcodes | Create a wrap-up code
[**post_user_routinglanguages**](RoutingApi.md#post_user_routinglanguages) | **POST** /api/v2/users/{userId}/routinglanguages | Add routing language to user
[**post_user_routingskills**](RoutingApi.md#post_user_routingskills) | **POST** /api/v2/users/{userId}/routingskills | Add routing skill to user
[**put_routing_email_domain_route**](RoutingApi.md#put_routing_email_domain_route) | **PUT** /api/v2/routing/email/domains/{domainName}/routes/{routeId} | Update a route
[**put_routing_message_recipient**](RoutingApi.md#put_routing_message_recipient) | **PUT** /api/v2/routing/message/recipients/{recipientId} | Update a recipient
[**put_routing_queue**](RoutingApi.md#put_routing_queue) | **PUT** /api/v2/routing/queues/{queueId} | Update a queue
[**put_routing_settings**](RoutingApi.md#put_routing_settings) | **PUT** /api/v2/routing/settings | Update an organization's routing settings
[**put_routing_settings_transcription**](RoutingApi.md#put_routing_settings_transcription) | **PUT** /api/v2/routing/settings/transcription | Update Transcription Settings
[**put_routing_sms_phonenumber**](RoutingApi.md#put_routing_sms_phonenumber) | **PUT** /api/v2/routing/sms/phonenumbers/{addressId} | Update a phone number provisioned for SMS.
[**put_routing_user_utilization**](RoutingApi.md#put_routing_user_utilization) | **PUT** /api/v2/routing/users/{userId}/utilization | Update the user's max utilization settings.  Include only those media types requiring custom configuration.
[**put_routing_utilization**](RoutingApi.md#put_routing_utilization) | **PUT** /api/v2/routing/utilization | Update the organization-wide max utilization settings.  Include only those media types requiring custom configuration.
[**put_routing_wrapupcode**](RoutingApi.md#put_routing_wrapupcode) | **PUT** /api/v2/routing/wrapupcodes/{codeId} | Update wrap-up code
[**put_user_routingskill**](RoutingApi.md#put_user_routingskill) | **PUT** /api/v2/users/{userId}/routingskills/{skillId} | Update routing skill proficiency or state.
[**put_user_routingskills_bulk**](RoutingApi.md#put_user_routingskills_bulk) | **PUT** /api/v2/users/{userId}/routingskills/bulk | Replace all routing skills assigned to a user



## delete_routing_assessment

> delete_routing_assessment(assessment_id)
Delete single benefit assessment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assessment_id** | **String** | Benefit Assessment ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_email_domain

> delete_routing_email_domain(domain_id)
Delete a domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | domain ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_email_domain_route

> delete_routing_email_domain_route(domain_name, route_id)
Delete a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | email domain | [required] |
**route_id** | **String** | route ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_predictor

> delete_routing_predictor(predictor_id)
Delete single predictor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**predictor_id** | **String** | Predictor ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_queue

> delete_routing_queue(queue_id, force_delete)
Delete a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**force_delete** | Option<**bool**> | forceDelete |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_queue_member

> delete_routing_queue_member(queue_id, member_id)
Delete a queue member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**member_id** | **String** | Member ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_queue_user

> delete_routing_queue_user(queue_id, member_id)
DEPRECATED: use DELETE /routing/queues/{queueId}/members/{memberId}.  Delete queue member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**member_id** | **String** | Member ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_queue_wrapupcode

> delete_routing_queue_wrapupcode(queue_id, code_id)
Delete a wrap-up code from a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**code_id** | **String** | Code ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_settings

> delete_routing_settings()
Delete an organization's routing settings

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_skill

> delete_routing_skill(skill_id)
Delete Routing Skill

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | Skill ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_sms_address

> delete_routing_sms_address(address_id)
Delete an Address by Id for SMS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_id** | **String** | Address ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_sms_phonenumber

> delete_routing_sms_phonenumber(address_id, _async)
Delete a phone number provisioned for SMS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_id** | **String** | Address ID | [required] |
**_async** | Option<**bool**> | Delete a phone number for SMS in an asynchronous manner. If the async parameter value is true, this initiates the deletion of a provisioned phone number.  |  |[default to false]

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


## delete_routing_utilization

> delete_routing_utilization()
Delete the organization-wide max utilization settings and revert to the system default.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_routing_wrapupcode

> delete_routing_wrapupcode(code_id)
Delete wrap-up code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_id** | **String** | Wrapup Code ID | [required] |

### Return type

 (empty response body)

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


## get_routing_assessment

> crate::models::BenefitAssessment get_routing_assessment(assessment_id)
Retrieve a single benefit assessment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assessment_id** | **String** | Benefit Assessment ID | [required] |

### Return type

[**crate::models::BenefitAssessment**](BenefitAssessment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_assessments

> crate::models::AssessmentListing get_routing_assessments(before, after, limit, page_size, queue_id)
Retrieve all benefit assessments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | The cursor that points to the start of the set of entities that has been returned. |  |
**after** | Option<**String**> | The cursor that points to the end of the set of entities that has been returned. |  |
**limit** | Option<**String**> | Number of entities to return. Maximum of 200. Deprecated in favour of pageSize, use CursorQueryParameters instead. |  |
**page_size** | Option<**String**> | Number of entities to return. Maximum of 200. |  |
**queue_id** | Option<[**Vec<String>**](String.md)> | Queue ID(s) to filter assessments by. |  |

### Return type

[**crate::models::AssessmentListing**](AssessmentListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_assessments_job

> crate::models::BenefitAssessmentJob get_routing_assessments_job(job_id)
Retrieve a single benefit assessments job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Benefit Assessment Job ID | [required] |

### Return type

[**crate::models::BenefitAssessmentJob**](BenefitAssessmentJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_assessments_jobs

> crate::models::AssessmentJobListing get_routing_assessments_jobs(division_id)
Retrieve all benefit assessment jobs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) to filter assessment jobs by. |  |

### Return type

[**crate::models::AssessmentJobListing**](AssessmentJobListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_email_domain

> crate::models::InboundDomain get_routing_email_domain(domain_id)
Get domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | domain ID | [required] |

### Return type

[**crate::models::InboundDomain**](InboundDomain.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_email_domain_route

> crate::models::InboundRoute get_routing_email_domain_route(domain_name, route_id)
Get a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | email domain | [required] |
**route_id** | **String** | route ID | [required] |

### Return type

[**crate::models::InboundRoute**](InboundRoute.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_email_domain_routes

> crate::models::InboundRouteEntityListing get_routing_email_domain_routes(domain_name, page_size, page_number, pattern)
Get routes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | email domain | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**pattern** | Option<**String**> | Filter routes by the route's pattern property |  |

### Return type

[**crate::models::InboundRouteEntityListing**](InboundRouteEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_email_domains

> crate::models::InboundDomainEntityListing get_routing_email_domains()
Get domains

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InboundDomainEntityListing**](InboundDomainEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_email_setup

> crate::models::EmailSetup get_routing_email_setup()
Get email setup

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::EmailSetup**](EmailSetup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_languages

> crate::models::LanguageEntityListing get_routing_languages(page_size, page_number, sort_order, name, id)
Get the list of supported languages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Ascending or descending sort order |  |[default to ASC]
**name** | Option<**String**> | Name |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |

### Return type

[**crate::models::LanguageEntityListing**](LanguageEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_message_recipient

> crate::models::Recipient get_routing_message_recipient(recipient_id)
Get a recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | **String** | Recipient ID | [required] |

### Return type

[**crate::models::Recipient**](Recipient.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_message_recipients

> crate::models::RecipientListing get_routing_message_recipients(messenger_type, page_size, page_number)
Get recipients

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**messenger_type** | Option<**String**> | Messenger Type |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::RecipientListing**](RecipientListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_predictor

> crate::models::Predictor get_routing_predictor(predictor_id)
Retrieve a single predictor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**predictor_id** | **String** | Predictor ID | [required] |

### Return type

[**crate::models::Predictor**](Predictor.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_predictors

> crate::models::PredictorListing get_routing_predictors(before, after, limit, page_size, queue_id)
Retrieve all predictors.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> | The cursor that points to the start of the set of entities that has been returned. |  |
**after** | Option<**String**> | The cursor that points to the end of the set of entities that has been returned. |  |
**limit** | Option<**String**> | Number of entities to return. Maximum of 200. Deprecated in favour of pageSize, use CursorQueryParameters instead. |  |
**page_size** | Option<**String**> | Number of entities to return. Maximum of 200. |  |
**queue_id** | Option<[**Vec<String>**](String.md)> | Comma-separated list of queue Ids to filter by. |  |

### Return type

[**crate::models::PredictorListing**](PredictorListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_predictors_keyperformanceindicators

> Vec<crate::models::KeyPerformanceIndicator> get_routing_predictors_keyperformanceindicators()
Get a list of Key Performance Indicators available for the predictors.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::KeyPerformanceIndicator>**](KeyPerformanceIndicator.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue

> crate::models::Queue get_routing_queue(queue_id)
Get details about this queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |

### Return type

[**crate::models::Queue**](Queue.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue_comparisonperiod

> crate::models::ComparisonPeriod get_routing_queue_comparisonperiod(queue_id, comparison_period_id)
Get a Comparison Period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue id | [required] |
**comparison_period_id** | **String** | ComparisonPeriod id | [required] |

### Return type

[**crate::models::ComparisonPeriod**](ComparisonPeriod.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue_comparisonperiods

> crate::models::ComparisonPeriodListing get_routing_queue_comparisonperiods(queue_id)
Get list of comparison periods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue id | [required] |

### Return type

[**crate::models::ComparisonPeriodListing**](ComparisonPeriodListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue_estimatedwaittime

> crate::models::EstimatedWaitTimePredictions get_routing_queue_estimatedwaittime(queue_id, conversation_id)
Get Estimated Wait Time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | queueId | [required] |
**conversation_id** | Option<**String**> | conversationId |  |

### Return type

[**crate::models::EstimatedWaitTimePredictions**](EstimatedWaitTimePredictions.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue_mediatype_estimatedwaittime

> crate::models::EstimatedWaitTimePredictions get_routing_queue_mediatype_estimatedwaittime(queue_id, media_type)
Get Estimated Wait Time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | queueId | [required] |
**media_type** | **String** | mediaType | [required] |

### Return type

[**crate::models::EstimatedWaitTimePredictions**](EstimatedWaitTimePredictions.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue_members

> crate::models::QueueMemberEntityListing get_routing_queue_members(queue_id, page_number, page_size, sort_order, expand, name, profile_skills, skills, languages, routing_status, presence, member_by, joined)
Get the members of this queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**page_number** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> | Max value is 100 |  |[default to 25]
**sort_order** | Option<**String**> | Note: results are sorted by name. |  |[default to asc]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |
**name** | Option<**String**> | Filter by queue member name |  |
**profile_skills** | Option<[**Vec<String>**](String.md)> | Filter by profile skill |  |
**skills** | Option<[**Vec<String>**](String.md)> | Filter by skill |  |
**languages** | Option<[**Vec<String>**](String.md)> | Filter by language |  |
**routing_status** | Option<[**Vec<String>**](String.md)> | Filter by routing status |  |
**presence** | Option<[**Vec<String>**](String.md)> | Filter by presence |  |
**member_by** | Option<**String**> | Filter by member type |  |
**joined** | Option<**bool**> | Filter by joined status |  |

### Return type

[**crate::models::QueueMemberEntityListing**](QueueMemberEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue_users

> crate::models::QueueMemberEntityListing get_routing_queue_users(queue_id, page_number, page_size, sort_order, expand, joined, name, profile_skills, skills, languages, routing_status, presence)
DEPRECATED: use GET /routing/queues/{queueId}/members.  Get the members of this queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**page_number** | Option<**i32**> |  |  |[default to 1]
**page_size** | Option<**i32**> | Max value is 100 |  |[default to 25]
**sort_order** | Option<**String**> | Note: results are sorted by name. |  |[default to asc]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |
**joined** | Option<**bool**> | Filter by joined status |  |
**name** | Option<**String**> | Filter by queue member name |  |
**profile_skills** | Option<[**Vec<String>**](String.md)> | Filter by profile skill |  |
**skills** | Option<[**Vec<String>**](String.md)> | Filter by skill |  |
**languages** | Option<[**Vec<String>**](String.md)> | Filter by language |  |
**routing_status** | Option<[**Vec<String>**](String.md)> | Filter by routing status |  |
**presence** | Option<[**Vec<String>**](String.md)> | Filter by presence |  |

### Return type

[**crate::models::QueueMemberEntityListing**](QueueMemberEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queue_wrapupcodes

> crate::models::WrapupCodeEntityListing get_routing_queue_wrapupcodes(queue_id, page_size, page_number)
Get the wrap-up codes for a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::WrapupCodeEntityListing**](WrapupCodeEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queues

> crate::models::QueueEntityListing get_routing_queues(page_number, page_size, sort_order, name, id, division_id)
Get list of queues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_order** | Option<**String**> | Note: results are sorted by name. |  |[default to asc]
**name** | Option<**String**> | Filter by queue name |  |
**id** | Option<[**Vec<String>**](String.md)> | Filter by queue ID(s) |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Filter by queue division ID(s) |  |

### Return type

[**crate::models::QueueEntityListing**](QueueEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queues_divisionviews

> crate::models::QueueEntityListing get_routing_queues_divisionviews(page_size, page_number, sort_by, sort_order, name, id, division_id)
Get a paged listing of simplified queue objects, filterable by name, queue ID(s), or division ID(s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size [max value is 100] |  |[default to 25]
**page_number** | Option<**i32**> | Page number [max value is 5] |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**name** | Option<**String**> | Name |  |
**id** | Option<[**Vec<String>**](String.md)> | Queue ID(s) |  |
**division_id** | Option<[**Vec<String>**](String.md)> | Division ID(s) |  |

### Return type

[**crate::models::QueueEntityListing**](QueueEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queues_divisionviews_all

> crate::models::QueueEntityListing get_routing_queues_divisionviews_all(page_size, page_number, sort_order)
Get a paged listing of simplified queue objects, sorted by name.  Can be used to get a digest of all queues in an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size [max value is 500] |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]

### Return type

[**crate::models::QueueEntityListing**](QueueEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_queues_me

> crate::models::UserQueueEntityListing get_routing_queues_me(page_number, page_size, joined, sort_order)
Get a paged listing of queues the user is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**joined** | Option<**bool**> | Filter by joined status. |  |
**sort_order** | Option<**String**> | Note: results are sorted by name. |  |[default to asc]

### Return type

[**crate::models::UserQueueEntityListing**](UserQueueEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_settings

> crate::models::RoutingSettings get_routing_settings()
Get an organization's routing settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RoutingSettings**](RoutingSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_settings_contactcenter

> crate::models::ContactCenterSettings get_routing_settings_contactcenter()
Get Contact Center Settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ContactCenterSettings**](ContactCenterSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_settings_transcription

> crate::models::TranscriptionSettings get_routing_settings_transcription()
Get Transcription Settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TranscriptionSettings**](TranscriptionSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_skill

> crate::models::RoutingSkill get_routing_skill(skill_id)
Get Routing Skill

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | Skill ID | [required] |

### Return type

[**crate::models::RoutingSkill**](RoutingSkill.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_skills

> crate::models::SkillEntityListing get_routing_skills(page_size, page_number, name, id)
Get the list of routing skills.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**name** | Option<**String**> | Filter for results that start with this value |  |
**id** | Option<[**Vec<String>**](String.md)> | id |  |

### Return type

[**crate::models::SkillEntityListing**](SkillEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_sms_address

> crate::models::SmsAddress get_routing_sms_address(address_id)
Get an Address by Id for SMS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_id** | **String** | Address ID | [required] |

### Return type

[**crate::models::SmsAddress**](SmsAddress.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_sms_addresses

> crate::models::SmsAddressEntityListing get_routing_sms_addresses(page_size, page_number)
Get a list of Addresses for SMS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::SmsAddressEntityListing**](SmsAddressEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_sms_availablephonenumbers

> crate::models::SmsAvailablePhoneNumberEntityListing get_routing_sms_availablephonenumbers(country_code, phone_number_type, region, city, area_code, pattern, address_requirement)
Get a list of available phone numbers for SMS provisioning.

This request will return up to 30 random phone numbers matching the criteria specified.  To get additional phone numbers repeat the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | **String** | The ISO 3166-1 alpha-2 country code of the county for which available phone numbers should be returned | [required] |
**phone_number_type** | **String** | Type of available phone numbers searched | [required] |
**region** | Option<**String**> | Region/province/state that can be used to restrict the numbers returned |  |
**city** | Option<**String**> | City that can be used to restrict the numbers returned |  |
**area_code** | Option<**String**> | Area code that can be used to restrict the numbers returned |  |
**pattern** | Option<**String**> | A pattern to match phone numbers. Valid characters are '*' and [0-9a-zA-Z]. The '*' character will match any single digit. |  |
**address_requirement** | Option<**String**> | This indicates whether the phone number requires to have an Address registered. |  |

### Return type

[**crate::models::SmsAvailablePhoneNumberEntityListing**](SMSAvailablePhoneNumberEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_sms_phonenumber

> crate::models::SmsPhoneNumber get_routing_sms_phonenumber(address_id)
Get a phone number provisioned for SMS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_id** | **String** | Address ID | [required] |

### Return type

[**crate::models::SmsPhoneNumber**](SmsPhoneNumber.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_sms_phonenumbers

> crate::models::SmsPhoneNumberEntityListing get_routing_sms_phonenumbers(phone_number, phone_number_type, phone_number_status, country_code, page_size, page_number, sort_by, sort_order, language)
Get a list of provisioned phone numbers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phone_number** | Option<**String**> | Filter on phone number address. Allowable characters are the digits '0-9' and the wild card character '\\*'. If just digits are present, a contains search is done on the address pattern. For example, '317' could be matched anywhere in the address. An '\\*' will match multiple digits. For example, to match a specific area code within the US a pattern like '1317*' could be used. |  |
**phone_number_type** | Option<[**Vec<String>**](String.md)> | Filter on phone number type |  |
**phone_number_status** | Option<[**Vec<String>**](String.md)> | Filter on phone number status |  |
**country_code** | Option<[**Vec<String>**](String.md)> | Filter on country code |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Optional field to sort results |  |
**sort_order** | Option<**String**> | Sort order |  |
**language** | Option<**String**> | A language tag (which is sometimes referred to as a \"locale identifier\") to use to localize country field and sort operations |  |[default to en-US]

### Return type

[**crate::models::SmsPhoneNumberEntityListing**](SmsPhoneNumberEntityListing.md)

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


## get_routing_utilization

> crate::models::Utilization get_routing_utilization()
Get the organization-wide max utilization settings.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Utilization**](Utilization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_wrapupcode

> crate::models::WrapupCode get_routing_wrapupcode(code_id)
Get details about this wrap-up code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_id** | **String** | Wrapup Code ID | [required] |

### Return type

[**crate::models::WrapupCode**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_routing_wrapupcodes

> crate::models::WrapupCodeEntityListing get_routing_wrapupcodes(page_size, page_number, sort_by, sort_order, name)
Get list of wrapup codes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]
**name** | Option<**String**> | Wrapup code's name ('Sort by' param is ignored unless this field is provided) |  |

### Return type

[**crate::models::WrapupCodeEntityListing**](WrapupCodeEntityListing.md)

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


## patch_routing_conversation

> crate::models::RoutingConversationAttributesResponse patch_routing_conversation(conversation_id, body)
Update attributes of an in-queue conversation

Returns an object indicating the updated values of all settable attributes. Supported attributes: skillIds, languageId, and priority.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**body** | [**RoutingConversationAttributesRequest**](RoutingConversationAttributesRequest.md) | Conversation Attributes | [required] |

### Return type

[**crate::models::RoutingConversationAttributesResponse**](RoutingConversationAttributesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_email_domain

> crate::models::InboundDomain patch_routing_email_domain(domain_id, body)
Update domain settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | domain ID | [required] |
**body** | [**InboundDomainPatchRequest**](InboundDomainPatchRequest.md) | Domain settings | [required] |

### Return type

[**crate::models::InboundDomain**](InboundDomain.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_email_domain_validate

> crate::models::InboundDomain patch_routing_email_domain_validate(domain_id, body)
Validate domain settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | domain ID | [required] |
**body** | [**InboundDomainPatchRequest**](InboundDomainPatchRequest.md) | Domain settings | [required] |

### Return type

[**crate::models::InboundDomain**](InboundDomain.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_predictor

> crate::models::Predictor patch_routing_predictor(predictor_id, body)
Update single predictor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**predictor_id** | **String** | Predictor ID | [required] |
**body** | Option<[**PatchPredictorRequest**](PatchPredictorRequest.md)> |  |  |

### Return type

[**crate::models::Predictor**](Predictor.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_queue_member

> patch_routing_queue_member(queue_id, member_id, body)
Update the ring number OR joined status for a queue member.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**member_id** | **String** | Member ID | [required] |
**body** | [**QueueMember**](QueueMember.md) | Queue Member | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_queue_members

> crate::models::QueueMemberEntityListing patch_routing_queue_members(queue_id, body)
Join or unjoin a set of users for a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**body** | [**Vec<crate::models::QueueMember>**](QueueMember.md) | Queue Members | [required] |

### Return type

[**crate::models::QueueMemberEntityListing**](QueueMemberEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_queue_user

> patch_routing_queue_user(queue_id, member_id, body)
DEPRECATED: use PATCH /routing/queues/{queueId}/members/{memberId}.  Update the ring number OR joined status for a User in a Queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**member_id** | **String** | Member ID | [required] |
**body** | [**QueueMember**](QueueMember.md) | Queue Member | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_queue_users

> crate::models::QueueMemberEntityListing patch_routing_queue_users(queue_id, body)
DEPRECATED: use PATCH /routing/queues/{queueId}/members.  Join or unjoin a set of users for a queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**body** | [**Vec<crate::models::QueueMember>**](QueueMember.md) | Queue Members | [required] |

### Return type

[**crate::models::QueueMemberEntityListing**](QueueMemberEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_routing_settings_contactcenter

> patch_routing_settings_contactcenter(body)
Update Contact Center Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContactCenterSettings**](ContactCenterSettings.md) | Contact Center Settings | [required] |

### Return type

 (empty response body)

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


## post_analytics_queues_observations_query

> crate::models::QueueObservationQueryResponse post_analytics_queues_observations_query(body)
Query for queue observations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**QueueObservationQuery**](QueueObservationQuery.md) | query | [required] |

### Return type

[**crate::models::QueueObservationQueryResponse**](QueueObservationQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_assessments

> crate::models::BenefitAssessment post_routing_assessments(body)
Create a benefit assessment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateBenefitAssessmentRequest**](CreateBenefitAssessmentRequest.md)> |  |  |

### Return type

[**crate::models::BenefitAssessment**](BenefitAssessment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_assessments_jobs

> crate::models::BenefitAssessmentJob post_routing_assessments_jobs(body)
Create a benefit assessment job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateBenefitAssessmentJobRequest**](CreateBenefitAssessmentJobRequest.md)> |  |  |

### Return type

[**crate::models::BenefitAssessmentJob**](BenefitAssessmentJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_email_domain_routes

> crate::models::InboundRoute post_routing_email_domain_routes(domain_name, body)
Create a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | email domain | [required] |
**body** | [**InboundRoute**](InboundRoute.md) | Route | [required] |

### Return type

[**crate::models::InboundRoute**](InboundRoute.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_email_domain_testconnection

> crate::models::TestMessage post_routing_email_domain_testconnection(domain_id, body)
Tests the custom SMTP server integration connection set on this domain

The request body is optional. If omitted, this endpoint will just test the connection of the Custom SMTP Server. If the body is specified, there will be an attempt to send an email message to the server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **String** | domain ID | [required] |
**body** | Option<[**TestMessage**](TestMessage.md)> | TestMessage |  |

### Return type

[**crate::models::TestMessage**](TestMessage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_email_domains

> crate::models::InboundDomain post_routing_email_domains(body)
Create a domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InboundDomain**](InboundDomain.md) | Domain | [required] |

### Return type

[**crate::models::InboundDomain**](InboundDomain.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_languages

> crate::models::Language post_routing_languages(body)
Create Language

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Language**](Language.md) | Language | [required] |

### Return type

[**crate::models::Language**](Language.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_predictors

> crate::models::Predictor post_routing_predictors(body)
Create a predictor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreatePredictorRequest**](CreatePredictorRequest.md)> |  |  |

### Return type

[**crate::models::Predictor**](Predictor.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_queue_members

> post_routing_queue_members(queue_id, body, delete)
Bulk add or delete up to 100 queue members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**body** | [**Vec<crate::models::WritableEntity>**](WritableEntity.md) | Queue Members | [required] |
**delete** | Option<**bool**> | True to delete queue members |  |[default to false]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_queue_users

> post_routing_queue_users(queue_id, body, delete)
DEPRECATED: use POST /routing/queues/{queueId}/members.  Bulk add or delete up to 100 queue members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**body** | [**Vec<crate::models::WritableEntity>**](WritableEntity.md) | Queue Members | [required] |
**delete** | Option<**bool**> | True to delete queue members |  |[default to false]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_queue_wrapupcodes

> Vec<crate::models::WrapupCode> post_routing_queue_wrapupcodes(queue_id, body)
Add up to 100 wrap-up codes to a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**body** | [**Vec<crate::models::WrapUpCodeReference>**](WrapUpCodeReference.md) | List of wrapup codes | [required] |

### Return type

[**Vec<crate::models::WrapupCode>**](WrapupCode.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_queues

> crate::models::Queue post_routing_queues(body)
Create a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateQueueRequest**](CreateQueueRequest.md) | Queue | [required] |

### Return type

[**crate::models::Queue**](Queue.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_skills

> crate::models::RoutingSkill post_routing_skills(body)
Create Skill

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RoutingSkill**](RoutingSkill.md) | Skill | [required] |

### Return type

[**crate::models::RoutingSkill**](RoutingSkill.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_sms_addresses

> crate::models::SmsAddress post_routing_sms_addresses(body)
Provision an Address for SMS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SmsAddressProvision**](SmsAddressProvision.md) | SmsAddress | [required] |

### Return type

[**crate::models::SmsAddress**](SmsAddress.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_sms_phonenumbers

> crate::models::SmsPhoneNumber post_routing_sms_phonenumbers(body, _async)
Provision a phone number for SMS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**SmsPhoneNumberProvision**](SmsPhoneNumberProvision.md) | SmsPhoneNumber | [required] |
**_async** | Option<**bool**> | Provision a new phone number for SMS in an asynchronous manner. If the async parameter value is true, this initiates the provisioning of a new phone number. Check the phoneNumber's provisioningStatus for completion of this request. |  |[default to false]

### Return type

[**crate::models::SmsPhoneNumber**](SmsPhoneNumber.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_routing_wrapupcodes

> crate::models::WrapupCode post_routing_wrapupcodes(body)
Create a wrap-up code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WrapupCode**](WrapupCode.md) | WrapupCode | [required] |

### Return type

[**crate::models::WrapupCode**](WrapupCode.md)

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


## put_routing_email_domain_route

> crate::models::InboundRoute put_routing_email_domain_route(domain_name, route_id, body)
Update a route

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_name** | **String** | email domain | [required] |
**route_id** | **String** | route ID | [required] |
**body** | [**InboundRoute**](InboundRoute.md) | Route | [required] |

### Return type

[**crate::models::InboundRoute**](InboundRoute.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_routing_message_recipient

> crate::models::Recipient put_routing_message_recipient(recipient_id, body)
Update a recipient

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipient_id** | **String** | Recipient ID | [required] |
**body** | [**Recipient**](Recipient.md) | Recipient | [required] |

### Return type

[**crate::models::Recipient**](Recipient.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_routing_queue

> crate::models::Queue put_routing_queue(queue_id, body)
Update a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **String** | Queue ID | [required] |
**body** | [**QueueRequest**](QueueRequest.md) | Queue | [required] |

### Return type

[**crate::models::Queue**](Queue.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_routing_settings

> crate::models::RoutingSettings put_routing_settings(body)
Update an organization's routing settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**RoutingSettings**](RoutingSettings.md) | Organization Settings | [required] |

### Return type

[**crate::models::RoutingSettings**](RoutingSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_routing_settings_transcription

> crate::models::TranscriptionSettings put_routing_settings_transcription(body)
Update Transcription Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TranscriptionSettings**](TranscriptionSettings.md) | Organization Settings | [required] |

### Return type

[**crate::models::TranscriptionSettings**](TranscriptionSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_routing_sms_phonenumber

> crate::models::SmsPhoneNumber put_routing_sms_phonenumber(address_id, body, _async)
Update a phone number provisioned for SMS.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_id** | **String** | Address ID | [required] |
**body** | [**SmsPhoneNumber**](SmsPhoneNumber.md) | SmsPhoneNumber | [required] |
**_async** | Option<**bool**> | Update an existing phone number for SMS in an asynchronous manner. If the async parameter value is true, this initiates the update of a provisioned phone number. Check the phoneNumber's provisioningStatus for the progress of this request. |  |[default to false]

### Return type

[**crate::models::SmsPhoneNumber**](SmsPhoneNumber.md)

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


## put_routing_utilization

> crate::models::Utilization put_routing_utilization(body)
Update the organization-wide max utilization settings.  Include only those media types requiring custom configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Utilization**](Utilization.md) | utilization | [required] |

### Return type

[**crate::models::Utilization**](Utilization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_routing_wrapupcode

> crate::models::WrapupCode put_routing_wrapupcode(code_id, body)
Update wrap-up code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_id** | **String** | Wrapup Code ID | [required] |
**body** | [**WrapupCode**](WrapupCode.md) | WrapupCode | [required] |

### Return type

[**crate::models::WrapupCode**](WrapupCode.md)

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

