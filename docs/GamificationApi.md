# \GamificationApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gamification_leaderboard**](GamificationApi.md#get_gamification_leaderboard) | **GET** /api/v2/gamification/leaderboard | Leaderboard of the requesting user's division or performance profile
[**get_gamification_leaderboard_all**](GamificationApi.md#get_gamification_leaderboard_all) | **GET** /api/v2/gamification/leaderboard/all | Leaderboard by filter type
[**get_gamification_leaderboard_all_bestpoints**](GamificationApi.md#get_gamification_leaderboard_all_bestpoints) | **GET** /api/v2/gamification/leaderboard/all/bestpoints | Best Points by division or performance profile
[**get_gamification_leaderboard_bestpoints**](GamificationApi.md#get_gamification_leaderboard_bestpoints) | **GET** /api/v2/gamification/leaderboard/bestpoints | Best Points of the requesting user's current performance profile or division
[**get_gamification_metric**](GamificationApi.md#get_gamification_metric) | **GET** /api/v2/gamification/metrics/{metricId} | Gamified metric by id
[**get_gamification_metricdefinition**](GamificationApi.md#get_gamification_metricdefinition) | **GET** /api/v2/gamification/metricdefinitions/{metricDefinitionId} | Metric definition by id
[**get_gamification_metricdefinitions**](GamificationApi.md#get_gamification_metricdefinitions) | **GET** /api/v2/gamification/metricdefinitions | All metric definitions
[**get_gamification_metrics**](GamificationApi.md#get_gamification_metrics) | **GET** /api/v2/gamification/metrics | All gamified metrics for a given profile
[**get_gamification_profile**](GamificationApi.md#get_gamification_profile) | **GET** /api/v2/gamification/profiles/{performanceProfileId} | Performance profile by id
[**get_gamification_profile_members**](GamificationApi.md#get_gamification_profile_members) | **GET** /api/v2/gamification/profiles/{performanceProfileId}/members | Members of a given performance profile
[**get_gamification_profile_metric**](GamificationApi.md#get_gamification_profile_metric) | **GET** /api/v2/gamification/profiles/{profileId}/metrics/{metricId} | Performance profile gamified metric by id
[**get_gamification_profile_metrics**](GamificationApi.md#get_gamification_profile_metrics) | **GET** /api/v2/gamification/profiles/{profileId}/metrics | All gamified metrics for a given performance profile
[**get_gamification_profile_metrics_objectivedetails**](GamificationApi.md#get_gamification_profile_metrics_objectivedetails) | **GET** /api/v2/gamification/profiles/{profileId}/metrics/objectivedetails | All metrics for a given performance profile with objective details such as order and maxPoints
[**get_gamification_profiles**](GamificationApi.md#get_gamification_profiles) | **GET** /api/v2/gamification/profiles | All performance profiles
[**get_gamification_profiles_user**](GamificationApi.md#get_gamification_profiles_user) | **GET** /api/v2/gamification/profiles/users/{userId} | Performance profile of a user
[**get_gamification_profiles_users_me**](GamificationApi.md#get_gamification_profiles_users_me) | **GET** /api/v2/gamification/profiles/users/me | Performance profile of the requesting user
[**get_gamification_scorecards**](GamificationApi.md#get_gamification_scorecards) | **GET** /api/v2/gamification/scorecards | Workday performance metrics of the requesting user
[**get_gamification_scorecards_attendance**](GamificationApi.md#get_gamification_scorecards_attendance) | **GET** /api/v2/gamification/scorecards/attendance | Attendance status metrics of the requesting user
[**get_gamification_scorecards_bestpoints**](GamificationApi.md#get_gamification_scorecards_bestpoints) | **GET** /api/v2/gamification/scorecards/bestpoints | Best points of the requesting user
[**get_gamification_scorecards_points_alltime**](GamificationApi.md#get_gamification_scorecards_points_alltime) | **GET** /api/v2/gamification/scorecards/points/alltime | All-time points of the requesting user
[**get_gamification_scorecards_points_average**](GamificationApi.md#get_gamification_scorecards_points_average) | **GET** /api/v2/gamification/scorecards/points/average | Average points of the requesting user's division or performance profile
[**get_gamification_scorecards_points_trends**](GamificationApi.md#get_gamification_scorecards_points_trends) | **GET** /api/v2/gamification/scorecards/points/trends | Points trends of the requesting user
[**get_gamification_scorecards_user**](GamificationApi.md#get_gamification_scorecards_user) | **GET** /api/v2/gamification/scorecards/users/{userId} | Workday performance metrics for a user
[**get_gamification_scorecards_user_attendance**](GamificationApi.md#get_gamification_scorecards_user_attendance) | **GET** /api/v2/gamification/scorecards/users/{userId}/attendance | Attendance status metrics for a user
[**get_gamification_scorecards_user_bestpoints**](GamificationApi.md#get_gamification_scorecards_user_bestpoints) | **GET** /api/v2/gamification/scorecards/users/{userId}/bestpoints | Best points of a user
[**get_gamification_scorecards_user_points_alltime**](GamificationApi.md#get_gamification_scorecards_user_points_alltime) | **GET** /api/v2/gamification/scorecards/users/{userId}/points/alltime | All-time points for a user
[**get_gamification_scorecards_user_points_trends**](GamificationApi.md#get_gamification_scorecards_user_points_trends) | **GET** /api/v2/gamification/scorecards/users/{userId}/points/trends | Points trend for a user
[**get_gamification_scorecards_user_values_trends**](GamificationApi.md#get_gamification_scorecards_user_values_trends) | **GET** /api/v2/gamification/scorecards/users/{userId}/values/trends | Values trends of a user
[**get_gamification_scorecards_users_points_average**](GamificationApi.md#get_gamification_scorecards_users_points_average) | **GET** /api/v2/gamification/scorecards/users/points/average | Workday average points by target group
[**get_gamification_scorecards_users_values_average**](GamificationApi.md#get_gamification_scorecards_users_values_average) | **GET** /api/v2/gamification/scorecards/users/values/average | Workday average values by target group
[**get_gamification_scorecards_users_values_trends**](GamificationApi.md#get_gamification_scorecards_users_values_trends) | **GET** /api/v2/gamification/scorecards/users/values/trends | Values trend by target group
[**get_gamification_scorecards_values_average**](GamificationApi.md#get_gamification_scorecards_values_average) | **GET** /api/v2/gamification/scorecards/values/average | Average values of the requesting user's division or performance profile
[**get_gamification_scorecards_values_trends**](GamificationApi.md#get_gamification_scorecards_values_trends) | **GET** /api/v2/gamification/scorecards/values/trends | Values trends of the requesting user or group
[**get_gamification_status**](GamificationApi.md#get_gamification_status) | **GET** /api/v2/gamification/status | Gamification activation status
[**get_gamification_template**](GamificationApi.md#get_gamification_template) | **GET** /api/v2/gamification/templates/{templateId} | Objective template by id
[**get_gamification_templates**](GamificationApi.md#get_gamification_templates) | **GET** /api/v2/gamification/templates | All objective templates
[**post_gamification_metrics**](GamificationApi.md#post_gamification_metrics) | **POST** /api/v2/gamification/metrics | Creates a gamified metric with a given metric definition and metric objective
[**post_gamification_profile_activate**](GamificationApi.md#post_gamification_profile_activate) | **POST** /api/v2/gamification/profiles/{performanceProfileId}/activate | Activate a performance profile
[**post_gamification_profile_deactivate**](GamificationApi.md#post_gamification_profile_deactivate) | **POST** /api/v2/gamification/profiles/{performanceProfileId}/deactivate | Deactivate a performance profile
[**post_gamification_profile_members**](GamificationApi.md#post_gamification_profile_members) | **POST** /api/v2/gamification/profiles/{performanceProfileId}/members | Assign members to a given performance profile
[**post_gamification_profile_members_validate**](GamificationApi.md#post_gamification_profile_members_validate) | **POST** /api/v2/gamification/profiles/{performanceProfileId}/members/validate | Validate member assignment
[**post_gamification_profile_metric_link**](GamificationApi.md#post_gamification_profile_metric_link) | **POST** /api/v2/gamification/profiles/{sourceProfileId}/metrics/{sourceMetricId}/link | Creates a linked metric
[**post_gamification_profile_metrics**](GamificationApi.md#post_gamification_profile_metrics) | **POST** /api/v2/gamification/profiles/{profileId}/metrics | Creates a gamified metric with a given metric definition and metric objective under in a performance profile
[**post_gamification_profiles**](GamificationApi.md#post_gamification_profiles) | **POST** /api/v2/gamification/profiles | Create a new custom performance profile
[**put_gamification_metric**](GamificationApi.md#put_gamification_metric) | **PUT** /api/v2/gamification/metrics/{metricId} | Updates a metric
[**put_gamification_profile**](GamificationApi.md#put_gamification_profile) | **PUT** /api/v2/gamification/profiles/{performanceProfileId} | Updates a performance profile
[**put_gamification_profile_metric**](GamificationApi.md#put_gamification_profile_metric) | **PUT** /api/v2/gamification/profiles/{profileId}/metrics/{metricId} | Updates a metric in performance profile
[**put_gamification_status**](GamificationApi.md#put_gamification_status) | **PUT** /api/v2/gamification/status | Update gamification activation status



## get_gamification_leaderboard

> crate::models::Leaderboard get_gamification_leaderboard(start_workday, end_workday, metric_id)
Leaderboard of the requesting user's division or performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_workday** | **String** | Start workday to retrieve for the leaderboard. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday to retrieve for the leaderboard. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**metric_id** | Option<**String**> | Metric Id for which the leaderboard is to be generated. The total points is used if nothing is given. |  |

### Return type

[**crate::models::Leaderboard**](Leaderboard.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_leaderboard_all

> crate::models::Leaderboard get_gamification_leaderboard_all(filter_type, filter_id, start_workday, end_workday, metric_id)
Leaderboard by filter type

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_type** | **String** | Filter type for the query request. | [required] |
**filter_id** | **String** | ID for the filter type. For example, division or performance profile Id | [required] |
**start_workday** | **String** | Start workday to retrieve for the leaderboard. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday to retrieve for the leaderboard. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**metric_id** | Option<**String**> | Metric Id for which the leaderboard is to be generated. The total points is used if nothing is given. |  |

### Return type

[**crate::models::Leaderboard**](Leaderboard.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_leaderboard_all_bestpoints

> crate::models::OverallBestPoints get_gamification_leaderboard_all_bestpoints(filter_type, filter_id)
Best Points by division or performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_type** | **String** | Filter type for the query request. | [required] |
**filter_id** | **String** | ID for the filter type. For example, division or performance profile Id | [required] |

### Return type

[**crate::models::OverallBestPoints**](OverallBestPoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_leaderboard_bestpoints

> crate::models::OverallBestPoints get_gamification_leaderboard_bestpoints()
Best Points of the requesting user's current performance profile or division

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OverallBestPoints**](OverallBestPoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_metric

> crate::models::Metric get_gamification_metric(metric_id, workday, performance_profile_id)
Gamified metric by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_id** | **String** | metric Id | [required] |
**workday** | Option<**String**> | The objective query workday. If not specified, then it retrieves the current objective. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |
**performance_profile_id** | Option<**String**> | The profile id of the metrics you are trying to retrieve. The DEFAULT profile is used if nothing is given. |  |

### Return type

[**crate::models::Metric**](Metric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_metricdefinition

> crate::models::MetricDefinition get_gamification_metricdefinition(metric_definition_id)
Metric definition by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_definition_id** | **String** | metric definition id | [required] |

### Return type

[**crate::models::MetricDefinition**](MetricDefinition.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_metricdefinitions

> crate::models::GetMetricDefinitionsResponse get_gamification_metricdefinitions()
All metric definitions

Retrieves the metric definitions and their corresponding default objectives used to create a gamified metric

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetMetricDefinitionsResponse**](GetMetricDefinitionsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_metrics

> crate::models::GetMetricsResponse get_gamification_metrics(performance_profile_id, workday)
All gamified metrics for a given profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | Option<**String**> | The profile id of the metrics you are trying to retrieve. The DEFAULT profile is used if nothing is given. |  |
**workday** | Option<**String**> | The objective query workday. If not specified, then it retrieves the current objective. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |

### Return type

[**crate::models::GetMetricsResponse**](GetMetricsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profile

> crate::models::PerformanceProfile get_gamification_profile(performance_profile_id)
Performance profile by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | **String** | Performance Profile Id | [required] |

### Return type

[**crate::models::PerformanceProfile**](PerformanceProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profile_members

> crate::models::MemberListing get_gamification_profile_members(performance_profile_id)
Members of a given performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | **String** | Performance Profile Id | [required] |

### Return type

[**crate::models::MemberListing**](MemberListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profile_metric

> crate::models::Metric get_gamification_profile_metric(profile_id, metric_id, workday)
Performance profile gamified metric by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Performance Profile Id | [required] |
**metric_id** | **String** | Metric Id | [required] |
**workday** | Option<**String**> | The objective query workday. If not specified, then it retrieves the current objective. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |

### Return type

[**crate::models::Metric**](Metric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profile_metrics

> crate::models::GetMetricResponse get_gamification_profile_metrics(profile_id, expand, workday)
All gamified metrics for a given performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Performance Profile Id | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |
**workday** | Option<**String**> | The objective query workday. If not specified, then it retrieves the current objective. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |

### Return type

[**crate::models::GetMetricResponse**](GetMetricResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profile_metrics_objectivedetails

> crate::models::GetMetricsResponse get_gamification_profile_metrics_objectivedetails(profile_id, workday)
All metrics for a given performance profile with objective details such as order and maxPoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Performance Profile Id | [required] |
**workday** | Option<**String**> | The objective query workday. If not specified, then it retrieves the current objective. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |

### Return type

[**crate::models::GetMetricsResponse**](GetMetricsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profiles

> crate::models::GetProfilesResponse get_gamification_profiles()
All performance profiles

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetProfilesResponse**](GetProfilesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profiles_user

> crate::models::PerformanceProfile get_gamification_profiles_user(user_id, workday)
Performance profile of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**workday** | Option<**String**> | Target querying workday. If not provided, then queries the current performance profile. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |

### Return type

[**crate::models::PerformanceProfile**](PerformanceProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_profiles_users_me

> crate::models::PerformanceProfile get_gamification_profiles_users_me(workday)
Performance profile of the requesting user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workday** | Option<**String**> | Target querying workday. If not provided, then queries the current performance profile. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |

### Return type

[**crate::models::PerformanceProfile**](PerformanceProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards

> crate::models::WorkdayMetricListing get_gamification_scorecards(workday, expand)
Workday performance metrics of the requesting user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workday** | **String** | Target querying workday. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::WorkdayMetricListing**](WorkdayMetricListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_attendance

> crate::models::AttendanceStatusListing get_gamification_scorecards_attendance(start_workday, end_workday)
Attendance status metrics of the requesting user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_workday** | **String** | Start workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |

### Return type

[**crate::models::AttendanceStatusListing**](AttendanceStatusListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_bestpoints

> crate::models::UserBestPoints get_gamification_scorecards_bestpoints()
Best points of the requesting user

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserBestPoints**](UserBestPoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_points_alltime

> crate::models::AllTimePoints get_gamification_scorecards_points_alltime(end_workday)
All-time points of the requesting user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |

### Return type

[**crate::models::AllTimePoints**](AllTimePoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_points_average

> crate::models::SingleWorkdayAveragePoints get_gamification_scorecards_points_average(workday)
Average points of the requesting user's division or performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workday** | **String** | The target workday. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |

### Return type

[**crate::models::SingleWorkdayAveragePoints**](SingleWorkdayAveragePoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_points_trends

> crate::models::WorkdayPointsTrend get_gamification_scorecards_points_trends(start_workday, end_workday, day_of_week)
Points trends of the requesting user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_workday** | **String** | Start workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**day_of_week** | Option<**String**> | Optional filter to specify which day of weeks to be included in the response |  |

### Return type

[**crate::models::WorkdayPointsTrend**](WorkdayPointsTrend.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_user

> crate::models::WorkdayMetricListing get_gamification_scorecards_user(user_id, workday, expand)
Workday performance metrics for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**workday** | **String** | Target querying workday. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::WorkdayMetricListing**](WorkdayMetricListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_user_attendance

> crate::models::AttendanceStatusListing get_gamification_scorecards_user_attendance(user_id, start_workday, end_workday)
Attendance status metrics for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**start_workday** | **String** | Start workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |

### Return type

[**crate::models::AttendanceStatusListing**](AttendanceStatusListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_user_bestpoints

> crate::models::UserBestPoints get_gamification_scorecards_user_bestpoints(user_id)
Best points of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |

### Return type

[**crate::models::UserBestPoints**](UserBestPoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_user_points_alltime

> crate::models::AllTimePoints get_gamification_scorecards_user_points_alltime(user_id, end_workday)
All-time points for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |

### Return type

[**crate::models::AllTimePoints**](AllTimePoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_user_points_trends

> crate::models::WorkdayPointsTrend get_gamification_scorecards_user_points_trends(user_id, start_workday, end_workday, day_of_week)
Points trend for a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**start_workday** | **String** | Start workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**day_of_week** | Option<**String**> | Optional filter to specify which day of weeks to be included in the response |  |

### Return type

[**crate::models::WorkdayPointsTrend**](WorkdayPointsTrend.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_user_values_trends

> crate::models::WorkdayValuesTrend get_gamification_scorecards_user_values_trends(user_id, start_workday, end_workday, time_zone)
Values trends of a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** |  | [required] |
**start_workday** | **String** | Start workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**time_zone** | Option<**String**> | Timezone for the workday. Defaults to UTC |  |[default to UTC]

### Return type

[**crate::models::WorkdayValuesTrend**](WorkdayValuesTrend.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_users_points_average

> crate::models::SingleWorkdayAveragePoints get_gamification_scorecards_users_points_average(filter_type, filter_id, workday)
Workday average points by target group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_type** | **String** | Filter type for the query request. | [required] |
**filter_id** | **String** | ID for the filter type. | [required] |
**workday** | **String** | The target workday. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |

### Return type

[**crate::models::SingleWorkdayAveragePoints**](SingleWorkdayAveragePoints.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_users_values_average

> crate::models::SingleWorkdayAverageValues get_gamification_scorecards_users_values_average(filter_type, filter_id, workday, time_zone)
Workday average values by target group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_type** | **String** | Filter type for the query request. | [required] |
**filter_id** | **String** | ID for the filter type. For example, division Id | [required] |
**workday** | **String** | The target workday. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**time_zone** | Option<**String**> | Timezone for the workday. Defaults to UTC |  |[default to UTC]

### Return type

[**crate::models::SingleWorkdayAverageValues**](SingleWorkdayAverageValues.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_users_values_trends

> crate::models::WorkdayValuesTrend get_gamification_scorecards_users_values_trends(filter_type, filter_id, start_workday, end_workday, time_zone)
Values trend by target group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_type** | **String** | Filter type for the query request. | [required] |
**filter_id** | **String** | ID for the filter type. | [required] |
**start_workday** | **String** | Start workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**time_zone** | Option<**String**> | Timezone for the workday. Defaults to UTC |  |[default to UTC]

### Return type

[**crate::models::WorkdayValuesTrend**](WorkdayValuesTrend.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_values_average

> crate::models::SingleWorkdayAverageValues get_gamification_scorecards_values_average(workday, time_zone)
Average values of the requesting user's division or performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workday** | **String** | The target workday. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**time_zone** | Option<**String**> | Timezone for the workday. Defaults to UTC |  |[default to UTC]

### Return type

[**crate::models::SingleWorkdayAverageValues**](SingleWorkdayAverageValues.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_scorecards_values_trends

> crate::models::WorkdayValuesTrend get_gamification_scorecards_values_trends(start_workday, end_workday, filter_type, reference_workday, time_zone)
Values trends of the requesting user or group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_workday** | **String** | Start workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**end_workday** | **String** | End workday of querying workdays range. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd | [required] |
**filter_type** | Option<**String**> | Filter type for the query request. If not set, then the request is for the requesting user. |  |
**reference_workday** | Option<**String**> | Reference workday for the trend. Used to determine the profile of the user as of this date. If not set, then the user's current profile will be used. Dates are represented as an ISO-8601 string. For example: yyyy-MM-dd |  |
**time_zone** | Option<**String**> | Timezone for the workday. Defaults to UTC |  |[default to UTC]

### Return type

[**crate::models::WorkdayValuesTrend**](WorkdayValuesTrend.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_status

> crate::models::GamificationStatus get_gamification_status()
Gamification activation status

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GamificationStatus**](GamificationStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_template

> crate::models::ObjectiveTemplate get_gamification_template(template_id)
Objective template by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | template id | [required] |

### Return type

[**crate::models::ObjectiveTemplate**](ObjectiveTemplate.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gamification_templates

> crate::models::GetTemplatesResponse get_gamification_templates()
All objective templates

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::GetTemplatesResponse**](GetTemplatesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_metrics

> crate::models::Metric post_gamification_metrics(body)
Creates a gamified metric with a given metric definition and metric objective

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateMetric**](CreateMetric.md) | Metric | [required] |

### Return type

[**crate::models::Metric**](Metric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_profile_activate

> crate::models::PerformanceProfile post_gamification_profile_activate(performance_profile_id)
Activate a performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | **String** | Performance Profile Id | [required] |

### Return type

[**crate::models::PerformanceProfile**](PerformanceProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_profile_deactivate

> crate::models::PerformanceProfile post_gamification_profile_deactivate(performance_profile_id)
Deactivate a performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | **String** | Performance Profile Id | [required] |

### Return type

[**crate::models::PerformanceProfile**](PerformanceProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_profile_members

> crate::models::Assignment post_gamification_profile_members(performance_profile_id, body)
Assign members to a given performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | **String** | Performance Profile Id | [required] |
**body** | [**AssignUsers**](AssignUsers.md) | assignUsers | [required] |

### Return type

[**crate::models::Assignment**](Assignment.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_profile_members_validate

> crate::models::AssignmentValidation post_gamification_profile_members_validate(performance_profile_id, body)
Validate member assignment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | **String** | Performance Profile Id | [required] |
**body** | [**ValidateAssignUsers**](ValidateAssignUsers.md) | memberAssignments | [required] |

### Return type

[**crate::models::AssignmentValidation**](AssignmentValidation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_profile_metric_link

> crate::models::Metric post_gamification_profile_metric_link(source_profile_id, source_metric_id, body)
Creates a linked metric

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_profile_id** | **String** | Source Performance Profile Id | [required] |
**source_metric_id** | **String** | Source Metric Id | [required] |
**body** | [**TargetPerformanceProfile**](TargetPerformanceProfile.md) | linkedMetric | [required] |

### Return type

[**crate::models::Metric**](Metric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_profile_metrics

> crate::models::Metric post_gamification_profile_metrics(profile_id, body)
Creates a gamified metric with a given metric definition and metric objective under in a performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Performance Profile Id | [required] |
**body** | [**CreateMetric**](CreateMetric.md) | Metric | [required] |

### Return type

[**crate::models::Metric**](Metric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_gamification_profiles

> crate::models::GetProfilesResponse post_gamification_profiles(body)
Create a new custom performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreatePerformanceProfile**](CreatePerformanceProfile.md) | performanceProfile | [required] |

### Return type

[**crate::models::GetProfilesResponse**](GetProfilesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_gamification_metric

> crate::models::Metric put_gamification_metric(metric_id, body, performance_profile_id)
Updates a metric

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**metric_id** | **String** | metric Id | [required] |
**body** | [**CreateMetric**](CreateMetric.md) | Metric | [required] |
**performance_profile_id** | Option<**String**> | The profile id of the metrics you are trying to retrieve. The DEFAULT profile is used if nothing is given. |  |

### Return type

[**crate::models::Metric**](Metric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_gamification_profile

> crate::models::PerformanceProfile put_gamification_profile(performance_profile_id, body)
Updates a performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**performance_profile_id** | **String** | Performance Profile Id | [required] |
**body** | Option<[**PerformanceProfile**](PerformanceProfile.md)> | performanceProfile |  |

### Return type

[**crate::models::PerformanceProfile**](PerformanceProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_gamification_profile_metric

> crate::models::Metric put_gamification_profile_metric(profile_id, metric_id, body)
Updates a metric in performance profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Performance Profile Id | [required] |
**metric_id** | **String** | Metric Id | [required] |
**body** | [**CreateMetric**](CreateMetric.md) | Metric | [required] |

### Return type

[**crate::models::Metric**](Metric.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_gamification_status

> crate::models::GamificationStatus put_gamification_status(status)
Update gamification activation status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | [**GamificationStatus**](GamificationStatus.md) | Gamification status | [required] |

### Return type

[**crate::models::GamificationStatus**](GamificationStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

