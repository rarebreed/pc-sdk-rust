# \ArchitectApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_architect_emergencygroup**](ArchitectApi.md#delete_architect_emergencygroup) | **DELETE** /api/v2/architect/emergencygroups/{emergencyGroupId} | Deletes a emergency group by ID
[**delete_architect_ivr**](ArchitectApi.md#delete_architect_ivr) | **DELETE** /api/v2/architect/ivrs/{ivrId} | Delete an IVR Config.
[**delete_architect_prompt**](ArchitectApi.md#delete_architect_prompt) | **DELETE** /api/v2/architect/prompts/{promptId} | Delete specified user prompt
[**delete_architect_prompt_resource**](ArchitectApi.md#delete_architect_prompt_resource) | **DELETE** /api/v2/architect/prompts/{promptId}/resources/{languageCode} | Delete specified user prompt resource
[**delete_architect_prompt_resource_audio**](ArchitectApi.md#delete_architect_prompt_resource_audio) | **DELETE** /api/v2/architect/prompts/{promptId}/resources/{languageCode}/audio | Delete specified user prompt resource audio
[**delete_architect_prompts**](ArchitectApi.md#delete_architect_prompts) | **DELETE** /api/v2/architect/prompts | Batch-delete a list of prompts
[**delete_architect_schedule**](ArchitectApi.md#delete_architect_schedule) | **DELETE** /api/v2/architect/schedules/{scheduleId} | Delete a schedule by id
[**delete_architect_schedulegroup**](ArchitectApi.md#delete_architect_schedulegroup) | **DELETE** /api/v2/architect/schedulegroups/{scheduleGroupId} | Deletes a schedule group by ID
[**delete_architect_systemprompt_resource**](ArchitectApi.md#delete_architect_systemprompt_resource) | **DELETE** /api/v2/architect/systemprompts/{promptId}/resources/{languageCode} | Delete a system prompt resource override.
[**delete_flow**](ArchitectApi.md#delete_flow) | **DELETE** /api/v2/flows/{flowId} | Delete flow
[**delete_flows**](ArchitectApi.md#delete_flows) | **DELETE** /api/v2/flows | Batch-delete a list of flows
[**delete_flows_datatable**](ArchitectApi.md#delete_flows_datatable) | **DELETE** /api/v2/flows/datatables/{datatableId} | deletes a specific datatable by id
[**delete_flows_datatable_row**](ArchitectApi.md#delete_flows_datatable_row) | **DELETE** /api/v2/flows/datatables/{datatableId}/rows/{rowId} | Delete a row entry
[**delete_flows_milestone**](ArchitectApi.md#delete_flows_milestone) | **DELETE** /api/v2/flows/milestones/{milestoneId} | Delete a flow milestone.
[**get_architect_dependencytracking**](ArchitectApi.md#get_architect_dependencytracking) | **GET** /api/v2/architect/dependencytracking | Get Dependency Tracking objects that have a given display name
[**get_architect_dependencytracking_build**](ArchitectApi.md#get_architect_dependencytracking_build) | **GET** /api/v2/architect/dependencytracking/build | Get Dependency Tracking build status for an organization
[**get_architect_dependencytracking_consumedresources**](ArchitectApi.md#get_architect_dependencytracking_consumedresources) | **GET** /api/v2/architect/dependencytracking/consumedresources | Get resources that are consumed by a given Dependency Tracking object
[**get_architect_dependencytracking_consumingresources**](ArchitectApi.md#get_architect_dependencytracking_consumingresources) | **GET** /api/v2/architect/dependencytracking/consumingresources | Get resources that consume a given Dependency Tracking object
[**get_architect_dependencytracking_deletedresourceconsumers**](ArchitectApi.md#get_architect_dependencytracking_deletedresourceconsumers) | **GET** /api/v2/architect/dependencytracking/deletedresourceconsumers | Get Dependency Tracking objects that consume deleted resources
[**get_architect_dependencytracking_object**](ArchitectApi.md#get_architect_dependencytracking_object) | **GET** /api/v2/architect/dependencytracking/object | Get a Dependency Tracking object
[**get_architect_dependencytracking_type**](ArchitectApi.md#get_architect_dependencytracking_type) | **GET** /api/v2/architect/dependencytracking/types/{typeId} | Get a Dependency Tracking type.
[**get_architect_dependencytracking_types**](ArchitectApi.md#get_architect_dependencytracking_types) | **GET** /api/v2/architect/dependencytracking/types | Get Dependency Tracking types.
[**get_architect_dependencytracking_updatedresourceconsumers**](ArchitectApi.md#get_architect_dependencytracking_updatedresourceconsumers) | **GET** /api/v2/architect/dependencytracking/updatedresourceconsumers | Get Dependency Tracking objects that depend on updated resources
[**get_architect_emergencygroup**](ArchitectApi.md#get_architect_emergencygroup) | **GET** /api/v2/architect/emergencygroups/{emergencyGroupId} | Gets a emergency group by ID
[**get_architect_emergencygroups**](ArchitectApi.md#get_architect_emergencygroups) | **GET** /api/v2/architect/emergencygroups | Get a list of emergency groups.
[**get_architect_ivr**](ArchitectApi.md#get_architect_ivr) | **GET** /api/v2/architect/ivrs/{ivrId} | Get an IVR config.
[**get_architect_ivrs**](ArchitectApi.md#get_architect_ivrs) | **GET** /api/v2/architect/ivrs | Get IVR configs.
[**get_architect_prompt**](ArchitectApi.md#get_architect_prompt) | **GET** /api/v2/architect/prompts/{promptId} | Get specified user prompt
[**get_architect_prompt_history_history_id**](ArchitectApi.md#get_architect_prompt_history_history_id) | **GET** /api/v2/architect/prompts/{promptId}/history/{historyId} | Get generated prompt history
[**get_architect_prompt_resource**](ArchitectApi.md#get_architect_prompt_resource) | **GET** /api/v2/architect/prompts/{promptId}/resources/{languageCode} | Get specified user prompt resource
[**get_architect_prompt_resources**](ArchitectApi.md#get_architect_prompt_resources) | **GET** /api/v2/architect/prompts/{promptId}/resources | Get a pageable list of user prompt resources
[**get_architect_prompts**](ArchitectApi.md#get_architect_prompts) | **GET** /api/v2/architect/prompts | Get a pageable list of user prompts
[**get_architect_schedule**](ArchitectApi.md#get_architect_schedule) | **GET** /api/v2/architect/schedules/{scheduleId} | Get a schedule by ID
[**get_architect_schedulegroup**](ArchitectApi.md#get_architect_schedulegroup) | **GET** /api/v2/architect/schedulegroups/{scheduleGroupId} | Gets a schedule group by ID
[**get_architect_schedulegroups**](ArchitectApi.md#get_architect_schedulegroups) | **GET** /api/v2/architect/schedulegroups | Get a list of schedule groups.
[**get_architect_schedules**](ArchitectApi.md#get_architect_schedules) | **GET** /api/v2/architect/schedules | Get a list of schedules.
[**get_architect_systemprompt**](ArchitectApi.md#get_architect_systemprompt) | **GET** /api/v2/architect/systemprompts/{promptId} | Get a system prompt
[**get_architect_systemprompt_history_history_id**](ArchitectApi.md#get_architect_systemprompt_history_history_id) | **GET** /api/v2/architect/systemprompts/{promptId}/history/{historyId} | Get generated prompt history
[**get_architect_systemprompt_resource**](ArchitectApi.md#get_architect_systemprompt_resource) | **GET** /api/v2/architect/systemprompts/{promptId}/resources/{languageCode} | Get a system prompt resource.
[**get_architect_systemprompt_resources**](ArchitectApi.md#get_architect_systemprompt_resources) | **GET** /api/v2/architect/systemprompts/{promptId}/resources | Get system prompt resources.
[**get_architect_systemprompts**](ArchitectApi.md#get_architect_systemprompts) | **GET** /api/v2/architect/systemprompts | Get System Prompts
[**get_flow**](ArchitectApi.md#get_flow) | **GET** /api/v2/flows/{flowId} | Get flow
[**get_flow_history_history_id**](ArchitectApi.md#get_flow_history_history_id) | **GET** /api/v2/flows/{flowId}/history/{historyId} | Get generated flow history
[**get_flow_latestconfiguration**](ArchitectApi.md#get_flow_latestconfiguration) | **GET** /api/v2/flows/{flowId}/latestconfiguration | Get the latest configuration for flow
[**get_flow_version**](ArchitectApi.md#get_flow_version) | **GET** /api/v2/flows/{flowId}/versions/{versionId} | Get flow version
[**get_flow_version_configuration**](ArchitectApi.md#get_flow_version_configuration) | **GET** /api/v2/flows/{flowId}/versions/{versionId}/configuration | Create flow version configuration
[**get_flow_versions**](ArchitectApi.md#get_flow_versions) | **GET** /api/v2/flows/{flowId}/versions | Get flow version list
[**get_flows**](ArchitectApi.md#get_flows) | **GET** /api/v2/flows | Get a pageable list of flows, filtered by query parameters
[**get_flows_datatable**](ArchitectApi.md#get_flows_datatable) | **GET** /api/v2/flows/datatables/{datatableId} | Returns a specific datatable by id
[**get_flows_datatable_export_job**](ArchitectApi.md#get_flows_datatable_export_job) | **GET** /api/v2/flows/datatables/{datatableId}/export/jobs/{exportJobId} | Returns the state information about an export job
[**get_flows_datatable_import_job**](ArchitectApi.md#get_flows_datatable_import_job) | **GET** /api/v2/flows/datatables/{datatableId}/import/jobs/{importJobId} | Returns the state information about an import job
[**get_flows_datatable_import_jobs**](ArchitectApi.md#get_flows_datatable_import_jobs) | **GET** /api/v2/flows/datatables/{datatableId}/import/jobs | Get all recent import jobs
[**get_flows_datatable_row**](ArchitectApi.md#get_flows_datatable_row) | **GET** /api/v2/flows/datatables/{datatableId}/rows/{rowId} | Returns a specific row for the datatable
[**get_flows_datatable_rows**](ArchitectApi.md#get_flows_datatable_rows) | **GET** /api/v2/flows/datatables/{datatableId}/rows | Returns the rows for the datatable with the given id
[**get_flows_datatables**](ArchitectApi.md#get_flows_datatables) | **GET** /api/v2/flows/datatables | Retrieve a list of datatables for the org
[**get_flows_datatables_divisionview**](ArchitectApi.md#get_flows_datatables_divisionview) | **GET** /api/v2/flows/datatables/divisionviews/{datatableId} | Returns a specific datatable by id
[**get_flows_datatables_divisionviews**](ArchitectApi.md#get_flows_datatables_divisionviews) | **GET** /api/v2/flows/datatables/divisionviews | Retrieve a list of datatables for the org
[**get_flows_divisionviews**](ArchitectApi.md#get_flows_divisionviews) | **GET** /api/v2/flows/divisionviews | Get a pageable list of basic flow information objects filterable by query parameters.
[**get_flows_execution**](ArchitectApi.md#get_flows_execution) | **GET** /api/v2/flows/executions/{flowExecutionId} | Get a flow execution's details. Flow execution details are available for several days after the flow is started.
[**get_flows_milestone**](ArchitectApi.md#get_flows_milestone) | **GET** /api/v2/flows/milestones/{milestoneId} | Get a flow milestone
[**get_flows_milestones**](ArchitectApi.md#get_flows_milestones) | **GET** /api/v2/flows/milestones | Get a pageable list of flow milestones, filtered by query parameters
[**get_flows_milestones_divisionviews**](ArchitectApi.md#get_flows_milestones_divisionviews) | **GET** /api/v2/flows/milestones/divisionviews | Get a pageable list of basic flow milestone information objects filterable by query parameters.
[**get_flows_outcome**](ArchitectApi.md#get_flows_outcome) | **GET** /api/v2/flows/outcomes/{flowOutcomeId} | Get a flow outcome
[**get_flows_outcomes**](ArchitectApi.md#get_flows_outcomes) | **GET** /api/v2/flows/outcomes | Get a pageable list of flow outcomes, filtered by query parameters
[**get_flows_outcomes_divisionviews**](ArchitectApi.md#get_flows_outcomes_divisionviews) | **GET** /api/v2/flows/outcomes/divisionviews | Get a pageable list of basic flow outcome information objects filterable by query parameters.
[**post_architect_dependencytracking_build**](ArchitectApi.md#post_architect_dependencytracking_build) | **POST** /api/v2/architect/dependencytracking/build | Rebuild Dependency Tracking data for an organization
[**post_architect_emergencygroups**](ArchitectApi.md#post_architect_emergencygroups) | **POST** /api/v2/architect/emergencygroups | Creates a new emergency group
[**post_architect_ivrs**](ArchitectApi.md#post_architect_ivrs) | **POST** /api/v2/architect/ivrs | Create IVR config.
[**post_architect_prompt_history**](ArchitectApi.md#post_architect_prompt_history) | **POST** /api/v2/architect/prompts/{promptId}/history | Generate prompt history
[**post_architect_prompt_resources**](ArchitectApi.md#post_architect_prompt_resources) | **POST** /api/v2/architect/prompts/{promptId}/resources | Create a new user prompt resource
[**post_architect_prompts**](ArchitectApi.md#post_architect_prompts) | **POST** /api/v2/architect/prompts | Create a new user prompt
[**post_architect_schedulegroups**](ArchitectApi.md#post_architect_schedulegroups) | **POST** /api/v2/architect/schedulegroups | Creates a new schedule group
[**post_architect_schedules**](ArchitectApi.md#post_architect_schedules) | **POST** /api/v2/architect/schedules | Create a new schedule.
[**post_architect_systemprompt_history**](ArchitectApi.md#post_architect_systemprompt_history) | **POST** /api/v2/architect/systemprompts/{promptId}/history | Generate system prompt history
[**post_architect_systemprompt_resources**](ArchitectApi.md#post_architect_systemprompt_resources) | **POST** /api/v2/architect/systemprompts/{promptId}/resources | Create system prompt resource override.
[**post_flow_history**](ArchitectApi.md#post_flow_history) | **POST** /api/v2/flows/{flowId}/history | Generate flow history
[**post_flow_versions**](ArchitectApi.md#post_flow_versions) | **POST** /api/v2/flows/{flowId}/versions | Create flow version
[**post_flows**](ArchitectApi.md#post_flows) | **POST** /api/v2/flows | Create flow
[**post_flows_actions_checkin**](ArchitectApi.md#post_flows_actions_checkin) | **POST** /api/v2/flows/actions/checkin | Check-in flow
[**post_flows_actions_checkout**](ArchitectApi.md#post_flows_actions_checkout) | **POST** /api/v2/flows/actions/checkout | Check-out flow
[**post_flows_actions_deactivate**](ArchitectApi.md#post_flows_actions_deactivate) | **POST** /api/v2/flows/actions/deactivate | Deactivate flow
[**post_flows_actions_publish**](ArchitectApi.md#post_flows_actions_publish) | **POST** /api/v2/flows/actions/publish | Publish flow
[**post_flows_actions_revert**](ArchitectApi.md#post_flows_actions_revert) | **POST** /api/v2/flows/actions/revert | Revert flow
[**post_flows_actions_unlock**](ArchitectApi.md#post_flows_actions_unlock) | **POST** /api/v2/flows/actions/unlock | Unlock flow
[**post_flows_datatable_export_jobs**](ArchitectApi.md#post_flows_datatable_export_jobs) | **POST** /api/v2/flows/datatables/{datatableId}/export/jobs | Begin an export process for exporting all rows from a datatable
[**post_flows_datatable_import_jobs**](ArchitectApi.md#post_flows_datatable_import_jobs) | **POST** /api/v2/flows/datatables/{datatableId}/import/jobs | Begin an import process for importing rows into a datatable
[**post_flows_datatable_rows**](ArchitectApi.md#post_flows_datatable_rows) | **POST** /api/v2/flows/datatables/{datatableId}/rows | Create a new row entry for the datatable.
[**post_flows_datatables**](ArchitectApi.md#post_flows_datatables) | **POST** /api/v2/flows/datatables | Create a new datatable with the specified json-schema definition
[**post_flows_executions**](ArchitectApi.md#post_flows_executions) | **POST** /api/v2/flows/executions | Launch an instance of a flow definition, for flow types that support it such as the 'workflow' type.
[**post_flows_milestones**](ArchitectApi.md#post_flows_milestones) | **POST** /api/v2/flows/milestones | Create a flow milestone
[**post_flows_outcomes**](ArchitectApi.md#post_flows_outcomes) | **POST** /api/v2/flows/outcomes | Create a flow outcome
[**put_architect_emergencygroup**](ArchitectApi.md#put_architect_emergencygroup) | **PUT** /api/v2/architect/emergencygroups/{emergencyGroupId} | Updates a emergency group by ID
[**put_architect_ivr**](ArchitectApi.md#put_architect_ivr) | **PUT** /api/v2/architect/ivrs/{ivrId} | Update an IVR Config.
[**put_architect_prompt**](ArchitectApi.md#put_architect_prompt) | **PUT** /api/v2/architect/prompts/{promptId} | Update specified user prompt
[**put_architect_prompt_resource**](ArchitectApi.md#put_architect_prompt_resource) | **PUT** /api/v2/architect/prompts/{promptId}/resources/{languageCode} | Update specified user prompt resource
[**put_architect_schedule**](ArchitectApi.md#put_architect_schedule) | **PUT** /api/v2/architect/schedules/{scheduleId} | Update schedule by ID
[**put_architect_schedulegroup**](ArchitectApi.md#put_architect_schedulegroup) | **PUT** /api/v2/architect/schedulegroups/{scheduleGroupId} | Updates a schedule group by ID
[**put_architect_systemprompt_resource**](ArchitectApi.md#put_architect_systemprompt_resource) | **PUT** /api/v2/architect/systemprompts/{promptId}/resources/{languageCode} | Updates a system prompt resource override.
[**put_flow**](ArchitectApi.md#put_flow) | **PUT** /api/v2/flows/{flowId} | Update flow
[**put_flows_datatable**](ArchitectApi.md#put_flows_datatable) | **PUT** /api/v2/flows/datatables/{datatableId} | Updates a specific datatable by id
[**put_flows_datatable_row**](ArchitectApi.md#put_flows_datatable_row) | **PUT** /api/v2/flows/datatables/{datatableId}/rows/{rowId} | Update a row entry
[**put_flows_milestone**](ArchitectApi.md#put_flows_milestone) | **PUT** /api/v2/flows/milestones/{milestoneId} | Updates a flow milestone
[**put_flows_outcome**](ArchitectApi.md#put_flows_outcome) | **PUT** /api/v2/flows/outcomes/{flowOutcomeId} | Updates a flow outcome



## delete_architect_emergencygroup

> delete_architect_emergencygroup(emergency_group_id)
Deletes a emergency group by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emergency_group_id** | **String** | Emergency group ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_ivr

> delete_architect_ivr(ivr_id)
Delete an IVR Config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ivr_id** | **String** | IVR id | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_prompt

> delete_architect_prompt(prompt_id, all_resources)
Delete specified user prompt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**all_resources** | Option<**bool**> | Whether or not to delete all the prompt resources |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_prompt_resource

> delete_architect_prompt_resource(prompt_id, language_code)
Delete specified user prompt resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**language_code** | **String** | Language | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_prompt_resource_audio

> delete_architect_prompt_resource_audio(prompt_id, language_code)
Delete specified user prompt resource audio

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**language_code** | **String** | Language | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_prompts

> crate::models::Operation delete_architect_prompts(id)
Batch-delete a list of prompts

Multiple IDs can be specified, in which case all specified prompts will be deleted.  Asynchronous.  Notification topic: v2.architect.prompts.{promptId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | List of Prompt IDs | [required] |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_schedule

> delete_architect_schedule(schedule_id)
Delete a schedule by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_schedulegroup

> delete_architect_schedulegroup(schedule_group_id)
Deletes a schedule group by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_group_id** | **String** | Schedule group ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_architect_systemprompt_resource

> delete_architect_systemprompt_resource(prompt_id, language_code)
Delete a system prompt resource override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**language_code** | **String** | Language | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flow

> delete_flow(flow_id)
Delete flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flows

> crate::models::Operation delete_flows(id)
Batch-delete a list of flows

Multiple IDs can be specified, in which case all specified flows will be deleted.  Asynchronous.  Notification topic: v2.flows.{flowId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | [**Vec<String>**](String.md) | List of Flow IDs | [required] |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flows_datatable

> delete_flows_datatable(datatable_id, force)
deletes a specific datatable by id

Deletes an entire datatable (including the schema and data) with a given datatableId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**force** | Option<**bool**> | force delete, even if in use |  |[default to false]

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flows_datatable_row

> delete_flows_datatable_row(datatable_id, row_id)
Delete a row entry

Deletes a row with a given rowId (the value of the key field).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**row_id** | **String** | the key for the row | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_flows_milestone

> serde_json::Value delete_flows_milestone(milestone_id)
Delete a flow milestone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_id** | **String** | flow milestone ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking

> crate::models::DependencyObjectEntityListing get_architect_dependencytracking(name, page_number, page_size, object_type, consumed_resources, consuming_resources, consumed_resource_type, consuming_resource_type)
Get Dependency Tracking objects that have a given display name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Object name to search for | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**object_type** | Option<[**Vec<String>**](String.md)> | Object type(s) to search for |  |
**consumed_resources** | Option<**bool**> | Include resources each result item consumes |  |
**consuming_resources** | Option<**bool**> | Include resources that consume each result item |  |
**consumed_resource_type** | Option<[**Vec<String>**](String.md)> | Types of consumed resources to return, if consumed resources are requested |  |
**consuming_resource_type** | Option<[**Vec<String>**](String.md)> | Types of consuming resources to return, if consuming resources are requested |  |

### Return type

[**crate::models::DependencyObjectEntityListing**](DependencyObjectEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_build

> crate::models::DependencyStatus get_architect_dependencytracking_build()
Get Dependency Tracking build status for an organization

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DependencyStatus**](DependencyStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_consumedresources

> crate::models::ConsumedResourcesEntityListing get_architect_dependencytracking_consumedresources(id, version, object_type, resource_type, page_number, page_size)
Get resources that are consumed by a given Dependency Tracking object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Consuming object ID | [required] |
**version** | **String** | Consuming object version | [required] |
**object_type** | **String** | Consuming object type.  Only versioned types are allowed here. | [required] |
**resource_type** | Option<[**Vec<String>**](String.md)> | Types of consumed resources to show |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::ConsumedResourcesEntityListing**](ConsumedResourcesEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_consumingresources

> crate::models::ConsumingResourcesEntityListing get_architect_dependencytracking_consumingresources(id, object_type, resource_type, version, page_number, page_size, flow_filter)
Get resources that consume a given Dependency Tracking object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Consumed object ID | [required] |
**object_type** | **String** | Consumed object type | [required] |
**resource_type** | Option<[**Vec<String>**](String.md)> | Types of consuming resources to show.  Only versioned types are allowed here. |  |
**version** | Option<**String**> | Object version |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**flow_filter** | Option<**String**> | Show only checkedIn or published flows |  |

### Return type

[**crate::models::ConsumingResourcesEntityListing**](ConsumingResourcesEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_deletedresourceconsumers

> crate::models::DependencyObjectEntityListing get_architect_dependencytracking_deletedresourceconsumers(name, object_type, flow_filter, consumed_resources, consumed_resource_type, page_number, page_size)
Get Dependency Tracking objects that consume deleted resources

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name to search for |  |
**object_type** | Option<[**Vec<String>**](String.md)> | Object type(s) to search for |  |
**flow_filter** | Option<**String**> | Show only checkedIn or published flows |  |
**consumed_resources** | Option<**bool**> | Return consumed resources? |  |[default to false]
**consumed_resource_type** | Option<[**Vec<String>**](String.md)> | Resource type(s) to return |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::DependencyObjectEntityListing**](DependencyObjectEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_object

> crate::models::DependencyObject get_architect_dependencytracking_object(id, version, object_type, consumed_resources, consuming_resources, consumed_resource_type, consuming_resource_type, consumed_resource_request)
Get a Dependency Tracking object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Object ID | [required] |
**version** | Option<**String**> | Object version |  |
**object_type** | Option<**String**> | Object type |  |
**consumed_resources** | Option<**bool**> | Include resources this item consumes |  |
**consuming_resources** | Option<**bool**> | Include resources that consume this item |  |
**consumed_resource_type** | Option<[**Vec<String>**](String.md)> | Types of consumed resources to return, if consumed resources are requested |  |
**consuming_resource_type** | Option<[**Vec<String>**](String.md)> | Types of consuming resources to return, if consuming resources are requested |  |
**consumed_resource_request** | Option<**bool**> | Indicate that this is going to look up a consumed resource object |  |

### Return type

[**crate::models::DependencyObject**](DependencyObject.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_type

> crate::models::DependencyType get_architect_dependencytracking_type(type_id)
Get a Dependency Tracking type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **String** | Type ID | [required] |

### Return type

[**crate::models::DependencyType**](DependencyType.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_types

> crate::models::DependencyTypeEntityListing get_architect_dependencytracking_types(page_number, page_size)
Get Dependency Tracking types.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::DependencyTypeEntityListing**](DependencyTypeEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_dependencytracking_updatedresourceconsumers

> crate::models::DependencyObjectEntityListing get_architect_dependencytracking_updatedresourceconsumers(name, object_type, consumed_resources, consumed_resource_type, page_number, page_size)
Get Dependency Tracking objects that depend on updated resources

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> | Name to search for |  |
**object_type** | Option<[**Vec<String>**](String.md)> | Object type(s) to search for |  |
**consumed_resources** | Option<**bool**> | Return consumed resources? |  |[default to false]
**consumed_resource_type** | Option<[**Vec<String>**](String.md)> | Resource type(s) to return |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::DependencyObjectEntityListing**](DependencyObjectEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_emergencygroup

> crate::models::EmergencyGroup get_architect_emergencygroup(emergency_group_id)
Gets a emergency group by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emergency_group_id** | **String** | Emergency group ID | [required] |

### Return type

[**crate::models::EmergencyGroup**](EmergencyGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_emergencygroups

> crate::models::EmergencyGroupListing get_architect_emergencygroups(page_number, page_size, sort_by, sort_order, name)
Get a list of emergency groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**name** | Option<**String**> | Name of the Emergency Group to filter by. |  |

### Return type

[**crate::models::EmergencyGroupListing**](EmergencyGroupListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_ivr

> crate::models::Ivr get_architect_ivr(ivr_id)
Get an IVR config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ivr_id** | **String** | IVR id | [required] |

### Return type

[**crate::models::Ivr**](IVR.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_ivrs

> crate::models::IvrEntityListing get_architect_ivrs(page_number, page_size, sort_by, sort_order, name)
Get IVR configs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**name** | Option<**String**> | Name of the IVR to filter by. |  |

### Return type

[**crate::models::IvrEntityListing**](IVREntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_prompt

> crate::models::Prompt get_architect_prompt(prompt_id)
Get specified user prompt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |

### Return type

[**crate::models::Prompt**](Prompt.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_prompt_history_history_id

> crate::models::HistoryListing get_architect_prompt_history_history_id(prompt_id, history_id, page_number, page_size, sort_order, sort_by, action)
Get generated prompt history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**history_id** | **String** | History request ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_order** | Option<**String**> | Sort order |  |[default to desc]
**sort_by** | Option<**String**> | Sort by |  |[default to timestamp]
**action** | Option<[**Vec<String>**](String.md)> | Flow actions to include (omit to include all) |  |

### Return type

[**crate::models::HistoryListing**](HistoryListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_prompt_resource

> crate::models::PromptAsset get_architect_prompt_resource(prompt_id, language_code)
Get specified user prompt resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**language_code** | **String** | Language | [required] |

### Return type

[**crate::models::PromptAsset**](PromptAsset.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_prompt_resources

> crate::models::PromptAssetEntityListing get_architect_prompt_resources(prompt_id, page_number, page_size)
Get a pageable list of user prompt resources

The returned list is pageable, and query parameters can be used for filtering.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::PromptAssetEntityListing**](PromptAssetEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_prompts

> crate::models::PromptEntityListing get_architect_prompts(page_number, page_size, name, description, name_or_description, sort_by, sort_order)
Get a pageable list of user prompts

The returned list is pageable, and query parameters can be used for filtering.  Multiple names can be specified, in which case all matching prompts will be returned, and no other filters will be evaluated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**name** | Option<[**Vec<String>**](String.md)> | Name |  |
**description** | Option<**String**> | Description |  |
**name_or_description** | Option<**String**> | Name or description |  |
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]

### Return type

[**crate::models::PromptEntityListing**](PromptEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_schedule

> crate::models::Schedule get_architect_schedule(schedule_id)
Get a schedule by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_schedulegroup

> crate::models::ScheduleGroup get_architect_schedulegroup(schedule_group_id)
Gets a schedule group by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_group_id** | **String** | Schedule group ID | [required] |

### Return type

[**crate::models::ScheduleGroup**](ScheduleGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_schedulegroups

> crate::models::ScheduleGroupEntityListing get_architect_schedulegroups(page_number, page_size, sort_by, sort_order, name, schedule_ids, division_id)
Get a list of schedule groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**name** | Option<**String**> | Name of the Schedule Group to filter by. |  |
**schedule_ids** | Option<**String**> | A comma-delimited list of Schedule IDs to filter by. |  |
**division_id** | Option<[**Vec<String>**](String.md)> | List of divisionIds on which to filter. |  |

### Return type

[**crate::models::ScheduleGroupEntityListing**](ScheduleGroupEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_schedules

> crate::models::ScheduleEntityListing get_architect_schedules(page_number, page_size, sort_by, sort_order, name, division_id)
Get a list of schedules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to name]
**sort_order** | Option<**String**> | Sort order |  |[default to ASC]
**name** | Option<**String**> | Name of the Schedule to filter by. |  |
**division_id** | Option<[**Vec<String>**](String.md)> | List of divisionIds on which to filter. |  |

### Return type

[**crate::models::ScheduleEntityListing**](ScheduleEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_systemprompt

> crate::models::SystemPrompt get_architect_systemprompt(prompt_id)
Get a system prompt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | promptId | [required] |

### Return type

[**crate::models::SystemPrompt**](SystemPrompt.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_systemprompt_history_history_id

> crate::models::HistoryListing get_architect_systemprompt_history_history_id(prompt_id, history_id, page_number, page_size, sort_order, sort_by, action)
Get generated prompt history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | promptId | [required] |
**history_id** | **String** | History request ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_order** | Option<**String**> | Sort order |  |[default to desc]
**sort_by** | Option<**String**> | Sort by |  |[default to timestamp]
**action** | Option<[**Vec<String>**](String.md)> | Flow actions to include (omit to include all) |  |

### Return type

[**crate::models::HistoryListing**](HistoryListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_systemprompt_resource

> crate::models::SystemPromptAsset get_architect_systemprompt_resource(prompt_id, language_code)
Get a system prompt resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**language_code** | **String** | Language | [required] |

### Return type

[**crate::models::SystemPromptAsset**](SystemPromptAsset.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_systemprompt_resources

> crate::models::SystemPromptAssetEntityListing get_architect_systemprompt_resources(prompt_id, page_number, page_size, sort_by, sort_order)
Get system prompt resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]

### Return type

[**crate::models::SystemPromptAssetEntityListing**](SystemPromptAssetEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_architect_systemprompts

> crate::models::SystemPromptEntityListing get_architect_systemprompts(page_number, page_size, sort_by, sort_order, name, description, name_or_description)
Get System Prompts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**name** | Option<**String**> | Name |  |
**description** | Option<**String**> | Description |  |
**name_or_description** | Option<**String**> | Name or description |  |

### Return type

[**crate::models::SystemPromptEntityListing**](SystemPromptEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow

> crate::models::Flow get_flow(flow_id, deleted)
Get flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**deleted** | Option<**bool**> | Deleted flows |  |[default to false]

### Return type

[**crate::models::Flow**](Flow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_history_history_id

> crate::models::HistoryListing get_flow_history_history_id(flow_id, history_id, page_number, page_size, sort_order, sort_by, action)
Get generated flow history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**history_id** | **String** | History request ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_order** | Option<**String**> | Sort order |  |[default to desc]
**sort_by** | Option<**String**> | Sort by |  |[default to timestamp]
**action** | Option<[**Vec<String>**](String.md)> | Flow actions to include (omit to include all) |  |

### Return type

[**crate::models::HistoryListing**](HistoryListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_latestconfiguration

> serde_json::Value get_flow_latestconfiguration(flow_id, deleted)
Get the latest configuration for flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**deleted** | Option<**bool**> | Deleted flows |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_version

> crate::models::FlowVersion get_flow_version(flow_id, version_id, deleted)
Get flow version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**version_id** | **String** | Version ID | [required] |
**deleted** | Option<**String**> | Deleted flows |  |

### Return type

[**crate::models::FlowVersion**](FlowVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_version_configuration

> serde_json::Value get_flow_version_configuration(flow_id, version_id, deleted)
Create flow version configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**version_id** | **String** | Version ID | [required] |
**deleted** | Option<**String**> | Deleted flows |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flow_versions

> crate::models::FlowVersionEntityListing get_flow_versions(flow_id, page_number, page_size, deleted)
Get flow version list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**deleted** | Option<**bool**> | Include Deleted flows |  |

### Return type

[**crate::models::FlowVersionEntityListing**](FlowVersionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows

> crate::models::FlowEntityListing get_flows(_type, page_number, page_size, sort_by, sort_order, id, name, description, name_or_description, publish_version_id, editable_by, locked_by, locked_by_client_id, secure, deleted, include_schemas, published_after, published_before, division_id)
Get a pageable list of flows, filtered by query parameters

If one or more IDs are specified, the search will fetch flows that match the given ID(s) and not use any additional supplied query parameters in the search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**Vec<String>**](String.md)> | Type |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**id** | Option<[**Vec<String>**](String.md)> | ID |  |
**name** | Option<**String**> | Name |  |
**description** | Option<**String**> | Description |  |
**name_or_description** | Option<**String**> | Name or description |  |
**publish_version_id** | Option<**String**> | Publish version ID |  |
**editable_by** | Option<**String**> | Editable by |  |
**locked_by** | Option<**String**> | Locked by |  |
**locked_by_client_id** | Option<**String**> | Locked by client ID |  |
**secure** | Option<**String**> | Secure |  |
**deleted** | Option<**bool**> | Include deleted |  |[default to false]
**include_schemas** | Option<**bool**> | Include variable schemas |  |[default to false]
**published_after** | Option<**String**> | Published after |  |
**published_before** | Option<**String**> | Published before |  |
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |

### Return type

[**crate::models::FlowEntityListing**](FlowEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatable

> crate::models::DataTable get_flows_datatable(datatable_id, expand)
Returns a specific datatable by id

Given a datatableId returns the datatable object and schema associated with it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**expand** | Option<**String**> | Expand instructions for the result |  |

### Return type

[**crate::models::DataTable**](DataTable.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatable_export_job

> crate::models::DataTableExportJob get_flows_datatable_export_job(datatable_id, export_job_id)
Returns the state information about an export job

Returns the state information about an export job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**export_job_id** | **String** | id of export job | [required] |

### Return type

[**crate::models::DataTableExportJob**](DataTableExportJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatable_import_job

> crate::models::DataTableImportJob get_flows_datatable_import_job(datatable_id, import_job_id)
Returns the state information about an import job

Returns the state information about an import job.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**import_job_id** | **String** | id of import job | [required] |

### Return type

[**crate::models::DataTableImportJob**](DataTableImportJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatable_import_jobs

> crate::models::DataTableImportEntityListing get_flows_datatable_import_jobs(datatable_id, page_number, page_size)
Get all recent import jobs

Get all recent import jobs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::DataTableImportEntityListing**](DataTableImportEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatable_row

> ::std::collections::HashMap<String, serde_json::Value> get_flows_datatable_row(datatable_id, row_id, showbrief)
Returns a specific row for the datatable

Given a datatableId and a rowId (the value of the key field) this will return the full row contents for that rowId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**row_id** | **String** | The key for the row | [required] |
**showbrief** | Option<**bool**> | if true returns just the key field for the row |  |[default to true]

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatable_rows

> crate::models::DataTableRowEntityListing get_flows_datatable_rows(datatable_id, page_number, page_size, showbrief)
Returns the rows for the datatable with the given id

Returns all of the rows for the datatable with the given datatableId.  By default this will just be a truncated list returning the key for each row. Set showBrief to false to return all of the row contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**showbrief** | Option<**bool**> | If true returns just the key value of the row |  |[default to true]

### Return type

[**crate::models::DataTableRowEntityListing**](DataTableRowEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatables

> crate::models::DataTablesDomainEntityListing get_flows_datatables(expand, page_number, page_size, sort_by, sort_order, division_id, name)
Retrieve a list of datatables for the org

Returns a metadata list of the datatables associated with this org, including datatableId, name and description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<**String**> | Expand instructions for the result |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |
**name** | Option<**String**> | Name to filter by |  |

### Return type

[**crate::models::DataTablesDomainEntityListing**](DataTablesDomainEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatables_divisionview

> crate::models::DataTable get_flows_datatables_divisionview(datatable_id, expand)
Returns a specific datatable by id

Given a datatableId returns the datatable object and schema associated with it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**expand** | Option<**String**> | Expand instructions for the result |  |

### Return type

[**crate::models::DataTable**](DataTable.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_datatables_divisionviews

> crate::models::DataTablesDomainEntityListing get_flows_datatables_divisionviews(expand, page_number, page_size, sort_by, sort_order, division_id, name)
Retrieve a list of datatables for the org

Returns a metadata list of the datatables associated with this org, including datatableId, name and description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<**String**> | Expand instructions for the result |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |
**name** | Option<**String**> | Name to filter by |  |

### Return type

[**crate::models::DataTablesDomainEntityListing**](DataTablesDomainEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_divisionviews

> crate::models::FlowDivisionViewEntityListing get_flows_divisionviews(_type, page_number, page_size, sort_by, sort_order, id, name, publish_version_id, published_after, published_before, division_id, include_schemas)
Get a pageable list of basic flow information objects filterable by query parameters.

This returns a simplified version of /flow consisting of name and type. If one or more IDs are specified, the search will fetch flows that match the given ID(s) and not use any additional supplied query parameters in the search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | Option<[**Vec<String>**](String.md)> | Type |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**id** | Option<[**Vec<String>**](String.md)> | ID |  |
**name** | Option<**String**> | Name |  |
**publish_version_id** | Option<**String**> | Publish version ID |  |
**published_after** | Option<**String**> | Published after |  |
**published_before** | Option<**String**> | Published before |  |
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |
**include_schemas** | Option<**bool**> | Include variable schemas |  |[default to false]

### Return type

[**crate::models::FlowDivisionViewEntityListing**](FlowDivisionViewEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_execution

> crate::models::FlowRuntimeExecution get_flows_execution(flow_execution_id)
Get a flow execution's details. Flow execution details are available for several days after the flow is started.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_execution_id** | **String** | flow execution ID | [required] |

### Return type

[**crate::models::FlowRuntimeExecution**](FlowRuntimeExecution.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_milestone

> crate::models::FlowMilestone get_flows_milestone(milestone_id)
Get a flow milestone

Returns a specified flow milestone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_id** | **String** | flow milestone ID | [required] |

### Return type

[**crate::models::FlowMilestone**](FlowMilestone.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_milestones

> crate::models::FlowMilestoneListing get_flows_milestones(page_number, page_size, sort_by, sort_order, id, name, description, name_or_description, division_id)
Get a pageable list of flow milestones, filtered by query parameters

Multiple IDs can be specified, in which case all matching flow milestones will be returned, and no other parameters will be evaluated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**id** | Option<[**Vec<String>**](String.md)> | ID |  |
**name** | Option<**String**> | Name |  |
**description** | Option<**String**> | Description |  |
**name_or_description** | Option<**String**> | Name or description |  |
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |

### Return type

[**crate::models::FlowMilestoneListing**](FlowMilestoneListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_milestones_divisionviews

> crate::models::FlowMilestoneDivisionViewEntityListing get_flows_milestones_divisionviews(page_number, page_size, sort_by, sort_order, id, name, division_id)
Get a pageable list of basic flow milestone information objects filterable by query parameters.

This returns flow milestones consisting of name and division. If one or more IDs are specified, the search will fetch flow milestones that match the given ID(s) and not use any additional supplied query parameters in the search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**id** | Option<[**Vec<String>**](String.md)> | ID |  |
**name** | Option<**String**> | Name |  |
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |

### Return type

[**crate::models::FlowMilestoneDivisionViewEntityListing**](FlowMilestoneDivisionViewEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_outcome

> crate::models::FlowOutcome get_flows_outcome(flow_outcome_id)
Get a flow outcome

Returns a specified flow outcome

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_outcome_id** | **String** | flow outcome ID | [required] |

### Return type

[**crate::models::FlowOutcome**](FlowOutcome.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_outcomes

> crate::models::FlowOutcomeListing get_flows_outcomes(page_number, page_size, sort_by, sort_order, id, name, description, name_or_description, division_id)
Get a pageable list of flow outcomes, filtered by query parameters

Multiple IDs can be specified, in which case all matching flow outcomes will be returned, and no other parameters will be evaluated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**id** | Option<[**Vec<String>**](String.md)> | ID |  |
**name** | Option<**String**> | Name |  |
**description** | Option<**String**> | Description |  |
**name_or_description** | Option<**String**> | Name or description |  |
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |

### Return type

[**crate::models::FlowOutcomeListing**](FlowOutcomeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_outcomes_divisionviews

> crate::models::FlowOutcomeDivisionViewEntityListing get_flows_outcomes_divisionviews(page_number, page_size, sort_by, sort_order, id, name, division_id)
Get a pageable list of basic flow outcome information objects filterable by query parameters.

This returns flow outcomes consisting of name and division. If one or more IDs are specified, the search will fetch flow outcomes that match the given ID(s) and not use any additional supplied query parameters in the search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Sort by |  |[default to id]
**sort_order** | Option<**String**> | Sort order |  |[default to asc]
**id** | Option<[**Vec<String>**](String.md)> | ID |  |
**name** | Option<**String**> | Name |  |
**division_id** | Option<[**Vec<String>**](String.md)> | division ID(s) |  |

### Return type

[**crate::models::FlowOutcomeDivisionViewEntityListing**](FlowOutcomeDivisionViewEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_dependencytracking_build

> post_architect_dependencytracking_build()
Rebuild Dependency Tracking data for an organization

Asynchronous.  Notification topic: v2.architect.dependencytracking.build

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


## post_architect_emergencygroups

> crate::models::EmergencyGroup post_architect_emergencygroups(body)
Creates a new emergency group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**EmergencyGroup**](EmergencyGroup.md) |  | [required] |

### Return type

[**crate::models::EmergencyGroup**](EmergencyGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_ivrs

> crate::models::Ivr post_architect_ivrs(body)
Create IVR config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Ivr**](Ivr.md) |  | [required] |

### Return type

[**crate::models::Ivr**](IVR.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_prompt_history

> crate::models::Operation post_architect_prompt_history(prompt_id)
Generate prompt history

Asynchronous.  Notification topic: v2.architect.prompts.{promptId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_prompt_resources

> crate::models::PromptAsset post_architect_prompt_resources(prompt_id, body)
Create a new user prompt resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**body** | [**PromptAssetCreate**](PromptAssetCreate.md) |  | [required] |

### Return type

[**crate::models::PromptAsset**](PromptAsset.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_prompts

> crate::models::Prompt post_architect_prompts(body)
Create a new user prompt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Prompt**](Prompt.md) |  | [required] |

### Return type

[**crate::models::Prompt**](Prompt.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_schedulegroups

> crate::models::ScheduleGroup post_architect_schedulegroups(body)
Creates a new schedule group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ScheduleGroup**](ScheduleGroup.md) |  | [required] |

### Return type

[**crate::models::ScheduleGroup**](ScheduleGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_schedules

> crate::models::Schedule post_architect_schedules(body)
Create a new schedule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Schedule**](Schedule.md) |  | [required] |

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_systemprompt_history

> crate::models::Operation post_architect_systemprompt_history(prompt_id)
Generate system prompt history

Asynchronous.  Notification topic: v2.architect.systemprompts.{systemPromptId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | promptId | [required] |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_architect_systemprompt_resources

> crate::models::SystemPromptAsset post_architect_systemprompt_resources(prompt_id, body)
Create system prompt resource override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**body** | [**SystemPromptAsset**](SystemPromptAsset.md) |  | [required] |

### Return type

[**crate::models::SystemPromptAsset**](SystemPromptAsset.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flow_history

> crate::models::Operation post_flow_history(flow_id)
Generate flow history

Asynchronous.  Notification topic: v2.flows.{flowId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flow_versions

> crate::models::FlowVersion post_flow_versions(flow_id, body)
Create flow version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [required] |

### Return type

[**crate::models::FlowVersion**](FlowVersion.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows

> crate::models::Flow post_flows(body, language)
Create flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Flow**](Flow.md) |  | [required] |
**language** | Option<**String**> | Language |  |

### Return type

[**crate::models::Flow**](Flow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_actions_checkin

> crate::models::Operation post_flows_actions_checkin(flow)
Check-in flow

Asynchronous.  Notification topic: v2.flows.{flowId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow** | **String** | Flow ID | [required] |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_actions_checkout

> crate::models::Flow post_flows_actions_checkout(flow)
Check-out flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow** | **String** | Flow ID | [required] |

### Return type

[**crate::models::Flow**](Flow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_actions_deactivate

> crate::models::Flow post_flows_actions_deactivate(flow)
Deactivate flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow** | **String** | Flow ID | [required] |

### Return type

[**crate::models::Flow**](Flow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_actions_publish

> crate::models::Operation post_flows_actions_publish(flow, version)
Publish flow

Asynchronous.  Notification topic: v2.flows.{flowId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow** | **String** | Flow ID | [required] |
**version** | Option<**String**> | version |  |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_actions_revert

> crate::models::Flow post_flows_actions_revert(flow)
Revert flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow** | **String** | Flow ID | [required] |

### Return type

[**crate::models::Flow**](Flow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_actions_unlock

> crate::models::Flow post_flows_actions_unlock(flow)
Unlock flow

Allows for unlocking a flow in the case where there is no flow configuration available, and thus a check-in will not unlock the flow. The user must have Architect Admin permissions to perform this action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow** | **String** | Flow ID | [required] |

### Return type

[**crate::models::Flow**](Flow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_datatable_export_jobs

> crate::models::DataTableExportJob post_flows_datatable_export_jobs(datatable_id)
Begin an export process for exporting all rows from a datatable

Create an export job for exporting rows. The caller can then poll for status of the export using the token returned in the response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |

### Return type

[**crate::models::DataTableExportJob**](DataTableExportJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_datatable_import_jobs

> crate::models::DataTableImportJob post_flows_datatable_import_jobs(datatable_id, body)
Begin an import process for importing rows into a datatable

Create an import job for importing rows. The caller can then poll for status of the import using the token returned in the response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**body** | [**DataTableImportJob**](DataTableImportJob.md) | import job information | [required] |

### Return type

[**crate::models::DataTableImportJob**](DataTableImportJob.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_datatable_rows

> ::std::collections::HashMap<String, serde_json::Value> post_flows_datatable_rows(datatable_id, data_table_row)
Create a new row entry for the datatable.

Will add the passed in row entry to the datatable with the given datatableId after verifying it against the schema.  The DataTableRow should be a json-ized' stream of key -> value pairs {      \"Field1\": \"XYZZY\",      \"Field2\": false,      \"KEY\": \"27272\"  }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**data_table_row** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_datatables

> crate::models::DataTable post_flows_datatables(body)
Create a new datatable with the specified json-schema definition

This will create a new datatable with fields that match the property definitions in the JSON schema.  The schema's title field will be overridden by the name field in the DataTable object.  See also http://json-schema.org/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DataTable**](DataTable.md) | datatable json-schema | [required] |

### Return type

[**crate::models::DataTable**](DataTable.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_executions

> crate::models::FlowExecutionLaunchResponse post_flows_executions(flow_launch_request)
Launch an instance of a flow definition, for flow types that support it such as the 'workflow' type.

The launch is asynchronous, it returns as soon as the flow starts. You can use the returned ID to query its status if you need.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_launch_request** | [**FlowExecutionLaunchRequest**](FlowExecutionLaunchRequest.md) |  | [required] |

### Return type

[**crate::models::FlowExecutionLaunchResponse**](FlowExecutionLaunchResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_milestones

> crate::models::FlowMilestone post_flows_milestones(body)
Create a flow milestone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**FlowMilestone**](FlowMilestone.md)> |  |  |

### Return type

[**crate::models::FlowMilestone**](FlowMilestone.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flows_outcomes

> crate::models::FlowOutcome post_flows_outcomes(body)
Create a flow outcome

Asynchronous.  Notification topic: v2.flows.outcomes.{flowOutcomeId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**FlowOutcome**](FlowOutcome.md)> |  |  |

### Return type

[**crate::models::FlowOutcome**](FlowOutcome.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_architect_emergencygroup

> crate::models::EmergencyGroup put_architect_emergencygroup(emergency_group_id, body)
Updates a emergency group by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emergency_group_id** | **String** | Emergency group ID | [required] |
**body** | [**EmergencyGroup**](EmergencyGroup.md) |  | [required] |

### Return type

[**crate::models::EmergencyGroup**](EmergencyGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_architect_ivr

> crate::models::Ivr put_architect_ivr(ivr_id, body)
Update an IVR Config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ivr_id** | **String** | IVR id | [required] |
**body** | [**Ivr**](Ivr.md) |  | [required] |

### Return type

[**crate::models::Ivr**](IVR.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_architect_prompt

> crate::models::Prompt put_architect_prompt(prompt_id, body)
Update specified user prompt

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**body** | [**Prompt**](Prompt.md) |  | [required] |

### Return type

[**crate::models::Prompt**](Prompt.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_architect_prompt_resource

> crate::models::PromptAsset put_architect_prompt_resource(prompt_id, language_code, body)
Update specified user prompt resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**language_code** | **String** | Language | [required] |
**body** | [**PromptAsset**](PromptAsset.md) |  | [required] |

### Return type

[**crate::models::PromptAsset**](PromptAsset.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_architect_schedule

> crate::models::Schedule put_architect_schedule(schedule_id, body)
Update schedule by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_id** | **String** | Schedule ID | [required] |
**body** | [**Schedule**](Schedule.md) |  | [required] |

### Return type

[**crate::models::Schedule**](Schedule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_architect_schedulegroup

> crate::models::ScheduleGroup put_architect_schedulegroup(schedule_group_id, body)
Updates a schedule group by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_group_id** | **String** | Schedule group ID | [required] |
**body** | [**ScheduleGroup**](ScheduleGroup.md) |  | [required] |

### Return type

[**crate::models::ScheduleGroup**](ScheduleGroup.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_architect_systemprompt_resource

> crate::models::SystemPromptAsset put_architect_systemprompt_resource(prompt_id, language_code, body)
Updates a system prompt resource override.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt_id** | **String** | Prompt ID | [required] |
**language_code** | **String** | Language | [required] |
**body** | [**SystemPromptAsset**](SystemPromptAsset.md) |  | [required] |

### Return type

[**crate::models::SystemPromptAsset**](SystemPromptAsset.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_flow

> crate::models::Flow put_flow(flow_id, body)
Update flow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_id** | **String** | Flow ID | [required] |
**body** | [**Flow**](Flow.md) |  | [required] |

### Return type

[**crate::models::Flow**](Flow.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_flows_datatable

> crate::models::DataTable put_flows_datatable(datatable_id, body, expand)
Updates a specific datatable by id

Updates a schema for a datatable with the given datatableId -updates allow only new fields to be added in the schema, no changes or removals of existing fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**body** | [**DataTable**](DataTable.md) | datatable json-schema | [required] |
**expand** | Option<**String**> | Expand instructions for the result |  |

### Return type

[**crate::models::DataTable**](DataTable.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_flows_datatable_row

> ::std::collections::HashMap<String, serde_json::Value> put_flows_datatable_row(datatable_id, row_id, body)
Update a row entry

Updates a row with the given rowId (the value of the key field) to the new values.  The DataTableRow should be a json-ized' stream of key -> value pairs {     \"Field1\": \"XYZZY\",     \"Field2\": false,     \"KEY\": \"27272\" }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datatable_id** | **String** | id of datatable | [required] |
**row_id** | **String** | the key for the row | [required] |
**body** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | datatable row |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_flows_milestone

> crate::models::FlowMilestone put_flows_milestone(milestone_id, body)
Updates a flow milestone

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**milestone_id** | **String** | flow milestone ID | [required] |
**body** | Option<[**FlowMilestone**](FlowMilestone.md)> |  |  |

### Return type

[**crate::models::FlowMilestone**](FlowMilestone.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_flows_outcome

> crate::models::Operation put_flows_outcome(flow_outcome_id, body)
Updates a flow outcome

Updates a flow outcome.  Asynchronous.  Notification topic: v2.flowoutcomes.{flowoutcomeId}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_outcome_id** | **String** | flow outcome ID | [required] |
**body** | Option<[**FlowOutcome**](FlowOutcome.md)> |  |  |

### Return type

[**crate::models::Operation**](Operation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

