# \AlertingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_alerting_interactionstats_alert**](AlertingApi.md#delete_alerting_interactionstats_alert) | **DELETE** /api/v2/alerting/interactionstats/alerts/{alertId} | Delete an interaction stats alert
[**delete_alerting_interactionstats_rule**](AlertingApi.md#delete_alerting_interactionstats_rule) | **DELETE** /api/v2/alerting/interactionstats/rules/{ruleId} | Delete an interaction stats rule.
[**get_alerting_alerts_active**](AlertingApi.md#get_alerting_alerts_active) | **GET** /api/v2/alerting/alerts/active | Gets active alert count for a user.
[**get_alerting_interactionstats_alert**](AlertingApi.md#get_alerting_interactionstats_alert) | **GET** /api/v2/alerting/interactionstats/alerts/{alertId} | Get an interaction stats alert
[**get_alerting_interactionstats_alerts**](AlertingApi.md#get_alerting_interactionstats_alerts) | **GET** /api/v2/alerting/interactionstats/alerts | Get interaction stats alert list.
[**get_alerting_interactionstats_alerts_unread**](AlertingApi.md#get_alerting_interactionstats_alerts_unread) | **GET** /api/v2/alerting/interactionstats/alerts/unread | Gets user unread count of interaction stats alerts.
[**get_alerting_interactionstats_rule**](AlertingApi.md#get_alerting_interactionstats_rule) | **GET** /api/v2/alerting/interactionstats/rules/{ruleId} | Get an interaction stats rule.
[**get_alerting_interactionstats_rules**](AlertingApi.md#get_alerting_interactionstats_rules) | **GET** /api/v2/alerting/interactionstats/rules | Get an interaction stats rule list.
[**post_alerting_interactionstats_rules**](AlertingApi.md#post_alerting_interactionstats_rules) | **POST** /api/v2/alerting/interactionstats/rules | Create an interaction stats rule.
[**put_alerting_interactionstats_alert**](AlertingApi.md#put_alerting_interactionstats_alert) | **PUT** /api/v2/alerting/interactionstats/alerts/{alertId} | Update an interaction stats alert read status
[**put_alerting_interactionstats_rule**](AlertingApi.md#put_alerting_interactionstats_rule) | **PUT** /api/v2/alerting/interactionstats/rules/{ruleId} | Update an interaction stats rule



## delete_alerting_interactionstats_alert

> delete_alerting_interactionstats_alert(alert_id)
Delete an interaction stats alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | Alert ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alerting_interactionstats_rule

> delete_alerting_interactionstats_rule(rule_id)
Delete an interaction stats rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** | Rule ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alerting_alerts_active

> crate::models::ActiveAlertCount get_alerting_alerts_active()
Gets active alert count for a user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ActiveAlertCount**](ActiveAlertCount.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alerting_interactionstats_alert

> crate::models::InteractionStatsAlert get_alerting_interactionstats_alert(alert_id, expand)
Get an interaction stats alert

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | Alert ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::InteractionStatsAlert**](InteractionStatsAlert.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alerting_interactionstats_alerts

> crate::models::InteractionStatsAlertContainer get_alerting_interactionstats_alerts(expand)
Get interaction stats alert list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::InteractionStatsAlertContainer**](InteractionStatsAlertContainer.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alerting_interactionstats_alerts_unread

> crate::models::UnreadMetric get_alerting_interactionstats_alerts_unread()
Gets user unread count of interaction stats alerts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UnreadMetric**](UnreadMetric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alerting_interactionstats_rule

> crate::models::InteractionStatsRule get_alerting_interactionstats_rule(rule_id, expand)
Get an interaction stats rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** | Rule ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::InteractionStatsRule**](InteractionStatsRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alerting_interactionstats_rules

> crate::models::InteractionStatsRuleContainer get_alerting_interactionstats_rules(expand)
Get an interaction stats rule list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::InteractionStatsRuleContainer**](InteractionStatsRuleContainer.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_alerting_interactionstats_rules

> crate::models::InteractionStatsRule post_alerting_interactionstats_rules(body, expand)
Create an interaction stats rule.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**InteractionStatsRule**](InteractionStatsRule.md) | AlertingRule | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::InteractionStatsRule**](InteractionStatsRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_alerting_interactionstats_alert

> crate::models::UnreadStatus put_alerting_interactionstats_alert(alert_id, body, expand)
Update an interaction stats alert read status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alert_id** | **String** | Alert ID | [required] |
**body** | [**UnreadStatus**](UnreadStatus.md) | InteractionStatsAlert | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::UnreadStatus**](UnreadStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_alerting_interactionstats_rule

> crate::models::InteractionStatsRule put_alerting_interactionstats_rule(rule_id, body, expand)
Update an interaction stats rule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rule_id** | **String** | Rule ID | [required] |
**body** | [**InteractionStatsRule**](InteractionStatsRule.md) | AlertingRule | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand |  |

### Return type

[**crate::models::InteractionStatsRule**](InteractionStatsRule.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

