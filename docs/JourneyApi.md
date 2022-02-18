# \JourneyApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_journey_actionmap**](JourneyApi.md#delete_journey_actionmap) | **DELETE** /api/v2/journey/actionmaps/{actionMapId} | Delete single action map.
[**delete_journey_actiontemplate**](JourneyApi.md#delete_journey_actiontemplate) | **DELETE** /api/v2/journey/actiontemplates/{actionTemplateId} | Delete a single action template.
[**delete_journey_outcome**](JourneyApi.md#delete_journey_outcome) | **DELETE** /api/v2/journey/outcomes/{outcomeId} | Delete an outcome.
[**delete_journey_segment**](JourneyApi.md#delete_journey_segment) | **DELETE** /api/v2/journey/segments/{segmentId} | Delete a segment.
[**get_journey_actionmap**](JourneyApi.md#get_journey_actionmap) | **GET** /api/v2/journey/actionmaps/{actionMapId} | Retrieve a single action map.
[**get_journey_actionmaps**](JourneyApi.md#get_journey_actionmaps) | **GET** /api/v2/journey/actionmaps | Retrieve all action maps.
[**get_journey_actiontarget**](JourneyApi.md#get_journey_actiontarget) | **GET** /api/v2/journey/actiontargets/{actionTargetId} | Retrieve a single action target.
[**get_journey_actiontargets**](JourneyApi.md#get_journey_actiontargets) | **GET** /api/v2/journey/actiontargets | Retrieve all action targets.
[**get_journey_actiontemplate**](JourneyApi.md#get_journey_actiontemplate) | **GET** /api/v2/journey/actiontemplates/{actionTemplateId} | Retrieve a single action template.
[**get_journey_actiontemplates**](JourneyApi.md#get_journey_actiontemplates) | **GET** /api/v2/journey/actiontemplates | Retrieve all action templates.
[**get_journey_outcome**](JourneyApi.md#get_journey_outcome) | **GET** /api/v2/journey/outcomes/{outcomeId} | Retrieve a single outcome.
[**get_journey_outcomes**](JourneyApi.md#get_journey_outcomes) | **GET** /api/v2/journey/outcomes | Retrieve all outcomes.
[**get_journey_segment**](JourneyApi.md#get_journey_segment) | **GET** /api/v2/journey/segments/{segmentId} | Retrieve a single segment.
[**get_journey_segments**](JourneyApi.md#get_journey_segments) | **GET** /api/v2/journey/segments | Retrieve all segments.
[**get_journey_session**](JourneyApi.md#get_journey_session) | **GET** /api/v2/journey/sessions/{sessionId} | Retrieve a single session.
[**get_journey_session_outcomescores**](JourneyApi.md#get_journey_session_outcomescores) | **GET** /api/v2/journey/sessions/{sessionId}/outcomescores | Retrieve latest outcome score associated with a session for all outcomes.
[**patch_journey_actionmap**](JourneyApi.md#patch_journey_actionmap) | **PATCH** /api/v2/journey/actionmaps/{actionMapId} | Update single action map.
[**patch_journey_actiontarget**](JourneyApi.md#patch_journey_actiontarget) | **PATCH** /api/v2/journey/actiontargets/{actionTargetId} | Update a single action target.
[**patch_journey_actiontemplate**](JourneyApi.md#patch_journey_actiontemplate) | **PATCH** /api/v2/journey/actiontemplates/{actionTemplateId} | Update a single action template.
[**patch_journey_outcome**](JourneyApi.md#patch_journey_outcome) | **PATCH** /api/v2/journey/outcomes/{outcomeId} | Update an outcome.
[**patch_journey_segment**](JourneyApi.md#patch_journey_segment) | **PATCH** /api/v2/journey/segments/{segmentId} | Update a segment.
[**post_analytics_journeys_aggregates_query**](JourneyApi.md#post_analytics_journeys_aggregates_query) | **POST** /api/v2/analytics/journeys/aggregates/query | Query for journey aggregates
[**post_journey_actionmaps**](JourneyApi.md#post_journey_actionmaps) | **POST** /api/v2/journey/actionmaps | Create an action map.
[**post_journey_actiontemplates**](JourneyApi.md#post_journey_actiontemplates) | **POST** /api/v2/journey/actiontemplates | Create a single action template.
[**post_journey_outcomes**](JourneyApi.md#post_journey_outcomes) | **POST** /api/v2/journey/outcomes | Create an outcome.
[**post_journey_segments**](JourneyApi.md#post_journey_segments) | **POST** /api/v2/journey/segments | Create a segment.



## delete_journey_actionmap

> delete_journey_actionmap(action_map_id)
Delete single action map.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_map_id** | **String** | ID of the action map. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_journey_actiontemplate

> delete_journey_actiontemplate(action_template_id, hard_delete)
Delete a single action template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_template_id** | **String** | ID of the action template. | [required] |
**hard_delete** | Option<**bool**> | Determines whether Action Template should be soft-deleted (have it's state set to deleted) or hard-deleted (permanently removed). Set to false (soft-delete) by default. |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_journey_outcome

> delete_journey_outcome(outcome_id)
Delete an outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outcome_id** | **String** | ID of the outcome. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_journey_segment

> delete_journey_segment(segment_id)
Delete a segment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | **String** | ID of the segment. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_actionmap

> crate::models::ActionMap get_journey_actionmap(action_map_id)
Retrieve a single action map.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_map_id** | **String** | ID of the action map. | [required] |

### Return type

[**crate::models::ActionMap**](ActionMap.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_actionmaps

> crate::models::ActionMapListing get_journey_actionmaps(page_number, page_size, sort_by, filter_field, filter_value, action_map_ids, query_fields, query_value)
Retrieve all action maps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Field(s) to sort by. Prefix with '-' for descending (e.g. sortBy=displayName,-createdDate). |  |
**filter_field** | Option<**String**> | Field to filter by (e.g. filterField=weight or filterField=action.actionTemplate.id). Requires 'filterField' to also be set. |  |
**filter_value** | Option<**String**> | Value to filter by. Requires 'filterValue' to also be set. |  |
**action_map_ids** | Option<[**Vec<String>**](String.md)> | IDs of action maps to return. Use of this parameter is not compatible with pagination, filtering, sorting or querying. A maximum of 100 action maps are allowed per request. |  |
**query_fields** | Option<[**Vec<String>**](String.md)> | Action Map field(s) to query on. Requires 'queryValue' to also be set. |  |
**query_value** | Option<**String**> | Value to query on. Requires 'queryFields' to also be set. |  |

### Return type

[**crate::models::ActionMapListing**](ActionMapListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_actiontarget

> crate::models::ActionTarget get_journey_actiontarget(action_target_id)
Retrieve a single action target.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_target_id** | **String** | ID of the action target. | [required] |

### Return type

[**crate::models::ActionTarget**](ActionTarget.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_actiontargets

> crate::models::ActionTargetListing get_journey_actiontargets(page_number, page_size)
Retrieve all action targets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::ActionTargetListing**](ActionTargetListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_actiontemplate

> crate::models::ActionTemplate get_journey_actiontemplate(action_template_id)
Retrieve a single action template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_template_id** | **String** | ID of the action template. | [required] |

### Return type

[**crate::models::ActionTemplate**](ActionTemplate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_actiontemplates

> crate::models::ActionTemplateListing get_journey_actiontemplates(page_number, page_size, sort_by, media_type, state, query_fields, query_value)
Retrieve all action templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Field(s) to sort by. Prefix with '-' for descending (e.g. sortBy=name,-createdDate). |  |
**media_type** | Option<**String**> | Media type |  |
**state** | Option<**String**> | Action template state. |  |
**query_fields** | Option<[**Vec<String>**](String.md)> | ActionTemplate field(s) to query on. Requires 'queryValue' to also be set. |  |
**query_value** | Option<**String**> | Value to query on. Requires 'queryFields' to also be set. |  |

### Return type

[**crate::models::ActionTemplateListing**](ActionTemplateListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_outcome

> crate::models::Outcome get_journey_outcome(outcome_id)
Retrieve a single outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outcome_id** | **String** | ID of the outcome. | [required] |

### Return type

[**crate::models::Outcome**](Outcome.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_outcomes

> crate::models::OutcomeListing get_journey_outcomes(page_number, page_size, sort_by, outcome_ids, query_fields, query_value)
Retrieve all outcomes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**sort_by** | Option<**String**> | Field(s) to sort by. The response can be sorted by any first level property on the Outcome response. Prefix with '-' for descending (e.g. sortBy=displayName,-createdDate). |  |
**outcome_ids** | Option<[**Vec<String>**](String.md)> | IDs of outcomes to return. Use of this parameter is not compatible with pagination, sorting or querying. A maximum of 20 outcomes are allowed per request. |  |
**query_fields** | Option<[**Vec<String>**](String.md)> | Outcome field(s) to query on. Requires 'queryValue' to also be set. |  |
**query_value** | Option<**String**> | Value to query on. Requires 'queryFields' to also be set. |  |

### Return type

[**crate::models::OutcomeListing**](OutcomeListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_segment

> crate::models::JourneySegment get_journey_segment(segment_id)
Retrieve a single segment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | **String** | ID of the segment. | [required] |

### Return type

[**crate::models::JourneySegment**](JourneySegment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_segments

> crate::models::SegmentListing get_journey_segments(sort_by, page_size, page_number, is_active, segment_ids, query_fields, query_value)
Retrieve all segments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_by** | Option<**String**> | Field(s) to sort by. The response can be sorted by any first level property on the Outcome response. Prefix with '-' for descending (e.g. sortBy=displayName,-createdDate). |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**is_active** | Option<**bool**> | Determines whether or not to show only active segments. |  |
**segment_ids** | Option<[**Vec<String>**](String.md)> | IDs of segments to return. Use of this parameter is not compatible with pagination, sorting or querying. A maximum of 100 segments are allowed per request. |  |
**query_fields** | Option<[**Vec<String>**](String.md)> | Segment field(s) to query on. Requires 'queryValue' to also be set. |  |
**query_value** | Option<**String**> | Value to query on. Requires 'queryFields' to also be set. |  |

### Return type

[**crate::models::SegmentListing**](SegmentListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_session

> crate::models::Session get_journey_session(session_id)
Retrieve a single session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | ID of the session. | [required] |

### Return type

[**crate::models::Session**](Session.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_journey_session_outcomescores

> crate::models::OutcomeScoresResult get_journey_session_outcomescores(session_id)
Retrieve latest outcome score associated with a session for all outcomes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** | ID of the session. | [required] |

### Return type

[**crate::models::OutcomeScoresResult**](OutcomeScoresResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_journey_actionmap

> crate::models::ActionMap patch_journey_actionmap(action_map_id, body)
Update single action map.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_map_id** | **String** | ID of the action map. | [required] |
**body** | Option<[**PatchActionMap**](PatchActionMap.md)> |  |  |

### Return type

[**crate::models::ActionMap**](ActionMap.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_journey_actiontarget

> crate::models::ActionTarget patch_journey_actiontarget(action_target_id, body)
Update a single action target.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_target_id** | **String** | ID of the action target. | [required] |
**body** | Option<[**PatchActionTarget**](PatchActionTarget.md)> |  |  |

### Return type

[**crate::models::ActionTarget**](ActionTarget.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_journey_actiontemplate

> crate::models::ActionTemplate patch_journey_actiontemplate(action_template_id, body)
Update a single action template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action_template_id** | **String** | ID of the action template. | [required] |
**body** | Option<[**PatchActionTemplate**](PatchActionTemplate.md)> |  |  |

### Return type

[**crate::models::ActionTemplate**](ActionTemplate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_journey_outcome

> crate::models::Outcome patch_journey_outcome(outcome_id, body)
Update an outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outcome_id** | **String** | ID of the outcome. | [required] |
**body** | Option<[**PatchOutcome**](PatchOutcome.md)> |  |  |

### Return type

[**crate::models::Outcome**](Outcome.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_journey_segment

> crate::models::JourneySegment patch_journey_segment(segment_id, body)
Update a segment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_id** | **String** | ID of the segment. | [required] |
**body** | Option<[**PatchSegment**](PatchSegment.md)> |  |  |

### Return type

[**crate::models::JourneySegment**](JourneySegment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_analytics_journeys_aggregates_query

> crate::models::JourneyAggregateQueryResponse post_analytics_journeys_aggregates_query(body)
Query for journey aggregates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**JourneyAggregationQuery**](JourneyAggregationQuery.md) | query | [required] |

### Return type

[**crate::models::JourneyAggregateQueryResponse**](JourneyAggregateQueryResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_journey_actionmaps

> crate::models::ActionMap post_journey_actionmaps(body)
Create an action map.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ActionMap**](ActionMap.md)> |  |  |

### Return type

[**crate::models::ActionMap**](ActionMap.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_journey_actiontemplates

> crate::models::ActionTemplate post_journey_actiontemplates(body)
Create a single action template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**ActionTemplate**](ActionTemplate.md)> |  |  |

### Return type

[**crate::models::ActionTemplate**](ActionTemplate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_journey_outcomes

> crate::models::Outcome post_journey_outcomes(body)
Create an outcome.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**Outcome**](Outcome.md)> |  |  |

### Return type

[**crate::models::Outcome**](Outcome.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_journey_segments

> crate::models::JourneySegment post_journey_segments(body)
Create a segment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<[**JourneySegment**](JourneySegment.md)> |  |  |

### Return type

[**crate::models::JourneySegment**](JourneySegment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

