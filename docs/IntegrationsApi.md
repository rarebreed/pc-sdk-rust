# \IntegrationsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_integration**](IntegrationsApi.md#delete_integration) | **DELETE** /api/v2/integrations/{integrationId} | Delete integration.
[**delete_integrations_action**](IntegrationsApi.md#delete_integrations_action) | **DELETE** /api/v2/integrations/actions/{actionId} | Delete an Action
[**delete_integrations_action_draft**](IntegrationsApi.md#delete_integrations_action_draft) | **DELETE** /api/v2/integrations/actions/{actionId}/draft | Delete a Draft
[**delete_integrations_credential**](IntegrationsApi.md#delete_integrations_credential) | **DELETE** /api/v2/integrations/credentials/{credentialId} | Delete a set of credentials
[**get_integration**](IntegrationsApi.md#get_integration) | **GET** /api/v2/integrations/{integrationId} | Get integration.
[**get_integration_config_current**](IntegrationsApi.md#get_integration_config_current) | **GET** /api/v2/integrations/{integrationId}/config/current | Get integration configuration.
[**get_integrations**](IntegrationsApi.md#get_integrations) | **GET** /api/v2/integrations | List integrations
[**get_integrations_action**](IntegrationsApi.md#get_integrations_action) | **GET** /api/v2/integrations/actions/{actionId} | Retrieves a single Action matching id.
[**get_integrations_action_draft**](IntegrationsApi.md#get_integrations_action_draft) | **GET** /api/v2/integrations/actions/{actionId}/draft | Retrieve a Draft
[**get_integrations_action_draft_schema**](IntegrationsApi.md#get_integrations_action_draft_schema) | **GET** /api/v2/integrations/actions/{actionId}/draft/schemas/{fileName} | Retrieve schema for a Draft based on filename.
[**get_integrations_action_draft_template**](IntegrationsApi.md#get_integrations_action_draft_template) | **GET** /api/v2/integrations/actions/{actionId}/draft/templates/{fileName} | Retrieve templates for a Draft based on filename.
[**get_integrations_action_draft_validation**](IntegrationsApi.md#get_integrations_action_draft_validation) | **GET** /api/v2/integrations/actions/{actionId}/draft/validation | Validate current Draft configuration.
[**get_integrations_action_schema**](IntegrationsApi.md#get_integrations_action_schema) | **GET** /api/v2/integrations/actions/{actionId}/schemas/{fileName} | Retrieve schema for an action based on filename.
[**get_integrations_action_template**](IntegrationsApi.md#get_integrations_action_template) | **GET** /api/v2/integrations/actions/{actionId}/templates/{fileName} | Retrieve text of templates for an action based on filename.
[**get_integrations_actions**](IntegrationsApi.md#get_integrations_actions) | **GET** /api/v2/integrations/actions | Retrieves all actions associated with filters passed in via query param.
[**get_integrations_actions_categories**](IntegrationsApi.md#get_integrations_actions_categories) | **GET** /api/v2/integrations/actions/categories | Retrieves all categories of available Actions
[**get_integrations_actions_drafts**](IntegrationsApi.md#get_integrations_actions_drafts) | **GET** /api/v2/integrations/actions/drafts | Retrieves all action drafts associated with the filters passed in via query param.
[**get_integrations_botconnector_integration_id_bot**](IntegrationsApi.md#get_integrations_botconnector_integration_id_bot) | **GET** /api/v2/integrations/botconnector/{integrationId}/bots/{botId} | Get a specific botConnector bot, plus versions, for this integration
[**get_integrations_botconnector_integration_id_bot_versions**](IntegrationsApi.md#get_integrations_botconnector_integration_id_bot_versions) | **GET** /api/v2/integrations/botconnector/{integrationId}/bots/{botId}/versions | Get a list of bot versions for a bot
[**get_integrations_botconnector_integration_id_bots**](IntegrationsApi.md#get_integrations_botconnector_integration_id_bots) | **GET** /api/v2/integrations/botconnector/{integrationId}/bots | Get a list of botConnector bots for this integration
[**get_integrations_botconnector_integration_id_bots_summaries**](IntegrationsApi.md#get_integrations_botconnector_integration_id_bots_summaries) | **GET** /api/v2/integrations/botconnector/{integrationId}/bots/summaries | Get a summary list of botConnector bots for this integration
[**get_integrations_clientapps**](IntegrationsApi.md#get_integrations_clientapps) | **GET** /api/v2/integrations/clientapps | List permitted client app integrations for the logged in user
[**get_integrations_clientapps_unifiedcommunications**](IntegrationsApi.md#get_integrations_clientapps_unifiedcommunications) | **GET** /api/v2/integrations/clientapps/unifiedcommunications | UC integration client application configuration.
[**get_integrations_credential**](IntegrationsApi.md#get_integrations_credential) | **GET** /api/v2/integrations/credentials/{credentialId} | Get a single credential with sensitive fields redacted
[**get_integrations_credentials**](IntegrationsApi.md#get_integrations_credentials) | **GET** /api/v2/integrations/credentials | List multiple sets of credentials
[**get_integrations_credentials_types**](IntegrationsApi.md#get_integrations_credentials_types) | **GET** /api/v2/integrations/credentials/types | List all credential types
[**get_integrations_eventlog**](IntegrationsApi.md#get_integrations_eventlog) | **GET** /api/v2/integrations/eventlog | List all events
[**get_integrations_eventlog_event_id**](IntegrationsApi.md#get_integrations_eventlog_event_id) | **GET** /api/v2/integrations/eventlog/{eventId} | Get a single event
[**get_integrations_speech_dialogflow_agent**](IntegrationsApi.md#get_integrations_speech_dialogflow_agent) | **GET** /api/v2/integrations/speech/dialogflow/agents/{agentId} | Get details about a Dialogflow agent
[**get_integrations_speech_dialogflow_agents**](IntegrationsApi.md#get_integrations_speech_dialogflow_agents) | **GET** /api/v2/integrations/speech/dialogflow/agents | Get a list of Dialogflow agents in the customers' Google accounts
[**get_integrations_speech_lex_bot_alias**](IntegrationsApi.md#get_integrations_speech_lex_bot_alias) | **GET** /api/v2/integrations/speech/lex/bot/alias/{aliasId} | Get details about a Lex bot alias
[**get_integrations_speech_lex_bot_bot_id_aliases**](IntegrationsApi.md#get_integrations_speech_lex_bot_bot_id_aliases) | **GET** /api/v2/integrations/speech/lex/bot/{botId}/aliases | Get a list of aliases for a bot in the customer's AWS accounts
[**get_integrations_speech_lex_bots**](IntegrationsApi.md#get_integrations_speech_lex_bots) | **GET** /api/v2/integrations/speech/lex/bots | Get a list of Lex bots in the customers' AWS accounts
[**get_integrations_speech_tts_engine**](IntegrationsApi.md#get_integrations_speech_tts_engine) | **GET** /api/v2/integrations/speech/tts/engines/{engineId} | Get details about a TTS engine
[**get_integrations_speech_tts_engine_voice**](IntegrationsApi.md#get_integrations_speech_tts_engine_voice) | **GET** /api/v2/integrations/speech/tts/engines/{engineId}/voices/{voiceId} | Get details about a specific voice for a TTS engine
[**get_integrations_speech_tts_engine_voices**](IntegrationsApi.md#get_integrations_speech_tts_engine_voices) | **GET** /api/v2/integrations/speech/tts/engines/{engineId}/voices | Get a list of voices for a TTS engine
[**get_integrations_speech_tts_engines**](IntegrationsApi.md#get_integrations_speech_tts_engines) | **GET** /api/v2/integrations/speech/tts/engines | Get a list of TTS engines enabled for org
[**get_integrations_speech_tts_settings**](IntegrationsApi.md#get_integrations_speech_tts_settings) | **GET** /api/v2/integrations/speech/tts/settings | Get TTS settings for an org
[**get_integrations_type**](IntegrationsApi.md#get_integrations_type) | **GET** /api/v2/integrations/types/{typeId} | Get integration type.
[**get_integrations_type_configschema**](IntegrationsApi.md#get_integrations_type_configschema) | **GET** /api/v2/integrations/types/{typeId}/configschemas/{configType} | Get properties config schema for an integration type.
[**get_integrations_types**](IntegrationsApi.md#get_integrations_types) | **GET** /api/v2/integrations/types | List integration types
[**get_integrations_userapps**](IntegrationsApi.md#get_integrations_userapps) | **GET** /api/v2/integrations/userapps | List permitted user app integrations for the logged in user
[**patch_integration**](IntegrationsApi.md#patch_integration) | **PATCH** /api/v2/integrations/{integrationId} | Update an integration.
[**patch_integrations_action**](IntegrationsApi.md#patch_integrations_action) | **PATCH** /api/v2/integrations/actions/{actionId} | Patch an Action
[**patch_integrations_action_draft**](IntegrationsApi.md#patch_integrations_action_draft) | **PATCH** /api/v2/integrations/actions/{actionId}/draft | Update an existing Draft
[**post_integrations**](IntegrationsApi.md#post_integrations) | **POST** /api/v2/integrations | Create an integration.
[**post_integrations_action_draft**](IntegrationsApi.md#post_integrations_action_draft) | **POST** /api/v2/integrations/actions/{actionId}/draft | Create a new Draft from existing Action
[**post_integrations_action_draft_publish**](IntegrationsApi.md#post_integrations_action_draft_publish) | **POST** /api/v2/integrations/actions/{actionId}/draft/publish | Publish a Draft and make it the active Action configuration
[**post_integrations_action_draft_test**](IntegrationsApi.md#post_integrations_action_draft_test) | **POST** /api/v2/integrations/actions/{actionId}/draft/test | Test the execution of a draft. Responses will show execution steps broken out with intermediate results to help in debugging.
[**post_integrations_action_execute**](IntegrationsApi.md#post_integrations_action_execute) | **POST** /api/v2/integrations/actions/{actionId}/execute | Execute Action and return response from 3rd party.  Responses will follow the schemas defined on the Action for success and error.
[**post_integrations_action_test**](IntegrationsApi.md#post_integrations_action_test) | **POST** /api/v2/integrations/actions/{actionId}/test | Test the execution of an action. Responses will show execution steps broken out with intermediate results to help in debugging.
[**post_integrations_actions**](IntegrationsApi.md#post_integrations_actions) | **POST** /api/v2/integrations/actions | Create a new Action
[**post_integrations_actions_drafts**](IntegrationsApi.md#post_integrations_actions_drafts) | **POST** /api/v2/integrations/actions/drafts | Create a new Draft
[**post_integrations_credentials**](IntegrationsApi.md#post_integrations_credentials) | **POST** /api/v2/integrations/credentials | Create a set of credentials
[**post_integrations_workforcemanagement_vendorconnection**](IntegrationsApi.md#post_integrations_workforcemanagement_vendorconnection) | **POST** /api/v2/integrations/workforcemanagement/vendorconnection | Add a vendor connection
[**put_integration_config_current**](IntegrationsApi.md#put_integration_config_current) | **PUT** /api/v2/integrations/{integrationId}/config/current | Update integration configuration.
[**put_integrations_botconnector_integration_id_bots**](IntegrationsApi.md#put_integrations_botconnector_integration_id_bots) | **PUT** /api/v2/integrations/botconnector/{integrationId}/bots | Set a list of botConnector bots plus versions for this integration
[**put_integrations_credential**](IntegrationsApi.md#put_integrations_credential) | **PUT** /api/v2/integrations/credentials/{credentialId} | Update a set of credentials
[**put_integrations_speech_tts_settings**](IntegrationsApi.md#put_integrations_speech_tts_settings) | **PUT** /api/v2/integrations/speech/tts/settings | Update TTS settings for an org



## delete_integration

> crate::models::Integration delete_integration(integration_id)
Delete integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration Id | [required] |

### Return type

[**crate::models::Integration**](Integration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_integrations_action

> delete_integrations_action(action_id)
Delete an Action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_integrations_action_draft

> delete_integrations_action_draft(action_id)
Delete a Draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_integrations_credential

> delete_integrations_credential(credential_id)
Delete a set of credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | Credential ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integration

> crate::models::Integration get_integration(integration_id, page_size, page_number, sort_by, expand, next_page, previous_page)
Get integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration Id | [required] |
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |

### Return type

[**crate::models::Integration**](Integration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integration_config_current

> crate::models::IntegrationConfiguration get_integration_config_current(integration_id)
Get integration configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration Id | [required] |

### Return type

[**crate::models::IntegrationConfiguration**](IntegrationConfiguration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations

> crate::models::IntegrationEntityListing get_integrations(page_size, page_number, sort_by, expand, next_page, previous_page)
List integrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |

### Return type

[**crate::models::IntegrationEntityListing**](IntegrationEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_action

> crate::models::Action get_integrations_action(action_id, expand, include_config)
Retrieves a single Action matching id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**expand** | Option<**String**> | Indicates a field in the response which should be expanded. |  |
**include_config** | Option<**bool**> | Return config in response. |  |[default to false]

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_action_draft

> crate::models::Action get_integrations_action_draft(action_id, expand, include_config)
Retrieve a Draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**expand** | Option<**String**> | Indicates a field in the response which should be expanded. |  |
**include_config** | Option<**bool**> | Return config in response. |  |[default to false]

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_action_draft_schema

> crate::models::JsonSchemaDocument get_integrations_action_draft_schema(action_id, file_name)
Retrieve schema for a Draft based on filename.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**file_name** | **String** | Name of schema file to be retrieved for this draft. | [required] |

### Return type

[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_action_draft_template

> String get_integrations_action_draft_template(action_id, file_name)
Retrieve templates for a Draft based on filename.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**file_name** | **String** | Name of template file to be retrieved for this action draft. | [required] |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_action_draft_validation

> crate::models::DraftValidationResult get_integrations_action_draft_validation(action_id)
Validate current Draft configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |

### Return type

[**crate::models::DraftValidationResult**](DraftValidationResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_action_schema

> crate::models::JsonSchemaDocument get_integrations_action_schema(action_id, file_name)
Retrieve schema for an action based on filename.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**file_name** | **String** | Name of schema file to be retrieved for this action. | [required] |

### Return type

[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_action_template

> String get_integrations_action_template(action_id, file_name)
Retrieve text of templates for an action based on filename.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**file_name** | **String** | Name of template file to be retrieved for this action. | [required] |

### Return type

**String**

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_actions

> crate::models::ActionEntityListing get_integrations_actions(page_size, page_number, next_page, previous_page, sort_by, sort_order, category, name, secure, include_auth_actions)
Retrieves all actions associated with filters passed in via query param.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**sort_by** | Option<**String**> | Root level field name to sort on. |  |
**sort_order** | Option<**String**> | Direction to sort 'sortBy' field. |  |[default to asc]
**category** | Option<**String**> | Filter by category name |  |
**name** | Option<**String**> | Filter by action name. Provide full or just the first part of name. |  |
**secure** | Option<**String**> | Filter to only include secure actions. True will only include actions marked secured. False will include only unsecure actions. Do not use filter if you want all Actions. |  |
**include_auth_actions** | Option<**String**> | Whether or not to include authentication actions in the response. These actions are not directly executable. Some integrations create them and will run them as needed to refresh authentication information for other actions. |  |[default to false]

### Return type

[**crate::models::ActionEntityListing**](ActionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_actions_categories

> crate::models::CategoryEntityListing get_integrations_actions_categories(page_size, page_number, next_page, previous_page, sort_by, sort_order, secure)
Retrieves all categories of available Actions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**sort_by** | Option<**String**> | Root level field name to sort on.  Only 'name' is supported on this endpoint. |  |
**sort_order** | Option<**String**> | Direction to sort 'sortBy' field. |  |[default to asc]
**secure** | Option<**String**> | Filter to only include secure actions. True will only include actions marked secured. False will include only unsecure actions. Do not use filter if you want all Actions. |  |

### Return type

[**crate::models::CategoryEntityListing**](CategoryEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_actions_drafts

> crate::models::ActionEntityListing get_integrations_actions_drafts(page_size, page_number, next_page, previous_page, sort_by, sort_order, category, name, secure, include_auth_actions)
Retrieves all action drafts associated with the filters passed in via query param.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**sort_by** | Option<**String**> | Root level field name to sort on. |  |
**sort_order** | Option<**String**> | Direction to sort 'sortBy' field. |  |[default to asc]
**category** | Option<**String**> | Filter by category name |  |
**name** | Option<**String**> | Filter by action name. Provide full or just the first part of name. |  |
**secure** | Option<**String**> | Filter to only include secure actions. True will only include actions marked secured. False will include only unsecure actions. Do not use filter if you want all Actions. |  |
**include_auth_actions** | Option<**String**> | Whether or not to include authentication actions in the response. These actions are not directly executable. Some integrations create them and will run them as needed to refresh authentication information for other actions. |  |[default to false]

### Return type

[**crate::models::ActionEntityListing**](ActionEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_botconnector_integration_id_bot

> crate::models::BotConnectorBot get_integrations_botconnector_integration_id_bot(integration_id, bot_id, version)
Get a specific botConnector bot, plus versions, for this integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | The integration ID for this group of bots | [required] |
**bot_id** | **String** | The botID for this bot | [required] |
**version** | Option<**String**> | Specific Version |  |

### Return type

[**crate::models::BotConnectorBot**](BotConnectorBot.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_botconnector_integration_id_bot_versions

> crate::models::BotConnectorBotVersionSummaryEntityListing get_integrations_botconnector_integration_id_bot_versions(integration_id, bot_id, page_number, page_size)
Get a list of bot versions for a bot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | The integration ID for this bot group | [required] |
**bot_id** | **String** | The botID for this bot | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::BotConnectorBotVersionSummaryEntityListing**](BotConnectorBotVersionSummaryEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_botconnector_integration_id_bots

> crate::models::BotList get_integrations_botconnector_integration_id_bots(integration_id)
Get a list of botConnector bots for this integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | The integration ID for this group of bots | [required] |

### Return type

[**crate::models::BotList**](BotList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_botconnector_integration_id_bots_summaries

> crate::models::BotConnectorBotSummaryEntityListing get_integrations_botconnector_integration_id_bots_summaries(integration_id, page_number, page_size)
Get a summary list of botConnector bots for this integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | The integration ID for this group of bots | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::BotConnectorBotSummaryEntityListing**](BotConnectorBotSummaryEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_clientapps

> crate::models::ClientAppEntityListing get_integrations_clientapps(page_size, page_number, sort_by, expand, next_page, previous_page)
List permitted client app integrations for the logged in user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |

### Return type

[**crate::models::ClientAppEntityListing**](ClientAppEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_clientapps_unifiedcommunications

> crate::models::UcIntegrationListing get_integrations_clientapps_unifiedcommunications(page_size, page_number, sort_by, expand, next_page, previous_page)
UC integration client application configuration.

This endpoint returns basic UI configuration data for all Unified Communications integrations client applications enabled for the current organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |

### Return type

[**crate::models::UcIntegrationListing**](UCIntegrationListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_credential

> crate::models::Credential get_integrations_credential(credential_id)
Get a single credential with sensitive fields redacted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | Credential ID | [required] |

### Return type

[**crate::models::Credential**](Credential.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_credentials

> crate::models::CredentialInfoListing get_integrations_credentials(page_number, page_size)
List multiple sets of credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::CredentialInfoListing**](CredentialInfoListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_credentials_types

> crate::models::CredentialTypeListing get_integrations_credentials_types()
List all credential types

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CredentialTypeListing**](CredentialTypeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_eventlog

> crate::models::IntegrationEventEntityListing get_integrations_eventlog(page_size, page_number, sort_by, sort_order, entity_id)
List all events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | Sort by |  |[default to timestamp]
**sort_order** | Option<**String**> | Order by |  |[default to descending]
**entity_id** | Option<**String**> | Include only events with this entity ID |  |

### Return type

[**crate::models::IntegrationEventEntityListing**](IntegrationEventEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_eventlog_event_id

> crate::models::IntegrationEvent get_integrations_eventlog_event_id(event_id)
Get a single event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** | Event Id | [required] |

### Return type

[**crate::models::IntegrationEvent**](IntegrationEvent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_dialogflow_agent

> crate::models::DialogflowAgent get_integrations_speech_dialogflow_agent(agent_id)
Get details about a Dialogflow agent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | **String** | The agent ID | [required] |

### Return type

[**crate::models::DialogflowAgent**](DialogflowAgent.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_dialogflow_agents

> crate::models::DialogflowAgentSummaryEntityListing get_integrations_speech_dialogflow_agents(page_number, page_size, name)
Get a list of Dialogflow agents in the customers' Google accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**name** | Option<**String**> | Filter on agent name |  |

### Return type

[**crate::models::DialogflowAgentSummaryEntityListing**](DialogflowAgentSummaryEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_lex_bot_alias

> crate::models::LexBotAlias get_integrations_speech_lex_bot_alias(alias_id)
Get details about a Lex bot alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias_id** | **String** | The alias ID | [required] |

### Return type

[**crate::models::LexBotAlias**](LexBotAlias.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_lex_bot_bot_id_aliases

> crate::models::LexBotAliasEntityListing get_integrations_speech_lex_bot_bot_id_aliases(bot_id, page_number, page_size, status, name)
Get a list of aliases for a bot in the customer's AWS accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **String** | The bot ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**status** | Option<**String**> | Filter on alias status |  |
**name** | Option<**String**> | Filter on alias name |  |

### Return type

[**crate::models::LexBotAliasEntityListing**](LexBotAliasEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_lex_bots

> crate::models::LexBotEntityListing get_integrations_speech_lex_bots(page_number, page_size, name)
Get a list of Lex bots in the customers' AWS accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**name** | Option<**String**> | Filter on bot name |  |

### Return type

[**crate::models::LexBotEntityListing**](LexBotEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_tts_engine

> crate::models::TtsEngineEntity get_integrations_speech_tts_engine(engine_id, include_voices)
Get details about a TTS engine

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**engine_id** | **String** | The engine ID | [required] |
**include_voices** | Option<**bool**> | Include voices for the engine |  |[default to false]

### Return type

[**crate::models::TtsEngineEntity**](TtsEngineEntity.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_tts_engine_voice

> crate::models::TtsVoiceEntity get_integrations_speech_tts_engine_voice(engine_id, voice_id)
Get details about a specific voice for a TTS engine

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**engine_id** | **String** | The engine ID | [required] |
**voice_id** | **String** | The voice ID | [required] |

### Return type

[**crate::models::TtsVoiceEntity**](TtsVoiceEntity.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_tts_engine_voices

> crate::models::TtsVoiceEntityListing get_integrations_speech_tts_engine_voices(engine_id, page_number, page_size)
Get a list of voices for a TTS engine

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**engine_id** | **String** | The engine ID | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::TtsVoiceEntityListing**](TtsVoiceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_tts_engines

> crate::models::TtsEngineEntityListing get_integrations_speech_tts_engines(page_number, page_size, include_voices, name, language)
Get a list of TTS engines enabled for org

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**include_voices** | Option<**bool**> | Include voices for the engine |  |[default to false]
**name** | Option<**String**> | Filter on engine name |  |
**language** | Option<**String**> | Filter on supported language. If includeVoices=true then the voices are also filtered. |  |

### Return type

[**crate::models::TtsEngineEntityListing**](TtsEngineEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_speech_tts_settings

> crate::models::TtsSettings get_integrations_speech_tts_settings()
Get TTS settings for an org

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TtsSettings**](TtsSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_type

> crate::models::IntegrationType get_integrations_type(type_id)
Get integration type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **String** | Integration Type Id | [required] |

### Return type

[**crate::models::IntegrationType**](IntegrationType.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_type_configschema

> crate::models::JsonSchemaDocument get_integrations_type_configschema(type_id, config_type)
Get properties config schema for an integration type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **String** | Integration Type Id | [required] |
**config_type** | **String** | Config schema type | [required] |

### Return type

[**crate::models::JsonSchemaDocument**](JsonSchemaDocument.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_types

> crate::models::IntegrationTypeEntityListing get_integrations_types(page_size, page_number, sort_by, expand, next_page, previous_page)
List integration types

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |

### Return type

[**crate::models::IntegrationTypeEntityListing**](IntegrationTypeEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_integrations_userapps

> crate::models::UserAppEntityListing get_integrations_userapps(page_size, page_number, sort_by, expand, next_page, previous_page, app_host)
List permitted user app integrations for the logged in user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**app_host** | Option<**String**> | The type of UserApp to filter by |  |

### Return type

[**crate::models::UserAppEntityListing**](UserAppEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_integration

> crate::models::Integration patch_integration(integration_id, page_size, page_number, sort_by, expand, next_page, previous_page, body)
Update an integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration Id | [required] |
**page_size** | Option<**i32**> | The total page size requested |  |[default to 25]
**page_number** | Option<**i32**> | The page number requested |  |[default to 1]
**sort_by** | Option<**String**> | variable name requested to sort by |  |
**expand** | Option<[**Vec<String>**](String.md)> | variable name requested by expand list |  |
**next_page** | Option<**String**> | next page token |  |
**previous_page** | Option<**String**> | Previous page token |  |
**body** | Option<[**Integration**](Integration.md)> | Integration Update |  |

### Return type

[**crate::models::Integration**](Integration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_integrations_action

> crate::models::Action patch_integrations_action(action_id, body)
Patch an Action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**body** | [**UpdateActionInput**](UpdateActionInput.md) | Input used to patch the Action. | [required] |

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_integrations_action_draft

> crate::models::Action patch_integrations_action_draft(action_id, body)
Update an existing Draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**body** | [**UpdateDraftInput**](UpdateDraftInput.md) | Input used to patch the Action Draft. | [required] |

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations

> crate::models::Integration post_integrations(body)
Create an integration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**CreateIntegrationRequest**](CreateIntegrationRequest.md)> | Integration |  |

### Return type

[**crate::models::Integration**](Integration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_action_draft

> crate::models::Action post_integrations_action_draft(action_id)
Create a new Draft from existing Action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_action_draft_publish

> crate::models::Action post_integrations_action_draft_publish(action_id, body)
Publish a Draft and make it the active Action configuration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**body** | [**PublishDraftInput**](PublishDraftInput.md) | Input used to patch the Action. | [required] |

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_action_draft_test

> crate::models::TestExecutionResult post_integrations_action_draft_test(action_id, body)
Test the execution of a draft. Responses will show execution steps broken out with intermediate results to help in debugging.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Map of parameters used for variable substitution. | [required] |

### Return type

[**crate::models::TestExecutionResult**](TestExecutionResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_action_execute

> serde_json::Value post_integrations_action_execute(action_id, body)
Execute Action and return response from 3rd party.  Responses will follow the schemas defined on the Action for success and error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Map of parameters used for variable substitution. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_action_test

> crate::models::TestExecutionResult post_integrations_action_test(action_id, body)
Test the execution of an action. Responses will show execution steps broken out with intermediate results to help in debugging.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_id** | **String** | actionId | [required] |
**body** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Map of parameters used for variable substitution. | [required] |

### Return type

[**crate::models::TestExecutionResult**](TestExecutionResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_actions

> crate::models::Action post_integrations_actions(body)
Create a new Action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PostActionInput**](PostActionInput.md) | Input used to create Action. | [required] |

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_actions_drafts

> crate::models::Action post_integrations_actions_drafts(body)
Create a new Draft

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**PostActionInput**](PostActionInput.md) | Input used to create Action Draft. | [required] |

### Return type

[**crate::models::Action**](Action.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_credentials

> crate::models::CredentialInfo post_integrations_credentials(body)
Create a set of credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Credential**](Credential.md)> | Credential |  |

### Return type

[**crate::models::CredentialInfo**](CredentialInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_integrations_workforcemanagement_vendorconnection

> crate::models::UserActionCategoryEntityListing post_integrations_workforcemanagement_vendorconnection(body)
Add a vendor connection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**VendorConnectionRequest**](VendorConnectionRequest.md)> |  |  |

### Return type

[**crate::models::UserActionCategoryEntityListing**](UserActionCategoryEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_integration_config_current

> crate::models::IntegrationConfiguration put_integration_config_current(integration_id, body)
Update integration configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | Integration Id | [required] |
**body** | Option<[**IntegrationConfiguration**](IntegrationConfiguration.md)> | Integration Configuration |  |

### Return type

[**crate::models::IntegrationConfiguration**](IntegrationConfiguration.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_integrations_botconnector_integration_id_bots

> put_integrations_botconnector_integration_id_bots(integration_id, bot_list)
Set a list of botConnector bots plus versions for this integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**integration_id** | **String** | The integration ID for this group of bots | [required] |
**bot_list** | [**BotList**](BotList.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_integrations_credential

> crate::models::CredentialInfo put_integrations_credential(credential_id, body)
Update a set of credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **String** | Credential ID | [required] |
**body** | Option<[**Credential**](Credential.md)> | Credential |  |

### Return type

[**crate::models::CredentialInfo**](CredentialInfo.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_integrations_speech_tts_settings

> crate::models::TtsSettings put_integrations_speech_tts_settings(body)
Update TTS settings for an org

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**TtsSettings**](TtsSettings.md) | Updated TtsSettings | [required] |

### Return type

[**crate::models::TtsSettings**](TtsSettings.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

