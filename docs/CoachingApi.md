# \CoachingApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_coaching_appointment**](CoachingApi.md#delete_coaching_appointment) | **DELETE** /api/v2/coaching/appointments/{appointmentId} | Delete an existing appointment
[**delete_coaching_appointment_annotation**](CoachingApi.md#delete_coaching_appointment_annotation) | **DELETE** /api/v2/coaching/appointments/{appointmentId}/annotations/{annotationId} | Delete an existing annotation
[**get_coaching_appointment**](CoachingApi.md#get_coaching_appointment) | **GET** /api/v2/coaching/appointments/{appointmentId} | Retrieve an appointment
[**get_coaching_appointment_annotation**](CoachingApi.md#get_coaching_appointment_annotation) | **GET** /api/v2/coaching/appointments/{appointmentId}/annotations/{annotationId} | Retrieve an annotation.
[**get_coaching_appointment_annotations**](CoachingApi.md#get_coaching_appointment_annotations) | **GET** /api/v2/coaching/appointments/{appointmentId}/annotations | Get a list of annotations.
[**get_coaching_appointment_statuses**](CoachingApi.md#get_coaching_appointment_statuses) | **GET** /api/v2/coaching/appointments/{appointmentId}/statuses | Get the list of status changes for a coaching appointment.
[**get_coaching_appointments**](CoachingApi.md#get_coaching_appointments) | **GET** /api/v2/coaching/appointments | Get appointments for users and optional date range
[**get_coaching_appointments_me**](CoachingApi.md#get_coaching_appointments_me) | **GET** /api/v2/coaching/appointments/me | Get my appointments for a given date range
[**get_coaching_notification**](CoachingApi.md#get_coaching_notification) | **GET** /api/v2/coaching/notifications/{notificationId} | Get an existing notification
[**get_coaching_notifications**](CoachingApi.md#get_coaching_notifications) | **GET** /api/v2/coaching/notifications | Retrieve the list of your notifications.
[**patch_coaching_appointment**](CoachingApi.md#patch_coaching_appointment) | **PATCH** /api/v2/coaching/appointments/{appointmentId} | Update an existing appointment
[**patch_coaching_appointment_annotation**](CoachingApi.md#patch_coaching_appointment_annotation) | **PATCH** /api/v2/coaching/appointments/{appointmentId}/annotations/{annotationId} | Update an existing annotation.
[**patch_coaching_appointment_status**](CoachingApi.md#patch_coaching_appointment_status) | **PATCH** /api/v2/coaching/appointments/{appointmentId}/status | Update the status of a coaching appointment
[**patch_coaching_notification**](CoachingApi.md#patch_coaching_notification) | **PATCH** /api/v2/coaching/notifications/{notificationId} | Update an existing notification.
[**post_coaching_appointment_annotations**](CoachingApi.md#post_coaching_appointment_annotations) | **POST** /api/v2/coaching/appointments/{appointmentId}/annotations | Create a new annotation.
[**post_coaching_appointment_conversations**](CoachingApi.md#post_coaching_appointment_conversations) | **POST** /api/v2/coaching/appointments/{appointmentId}/conversations | Add a conversation to an appointment
[**post_coaching_appointments**](CoachingApi.md#post_coaching_appointments) | **POST** /api/v2/coaching/appointments | Create a new appointment
[**post_coaching_appointments_aggregates_query**](CoachingApi.md#post_coaching_appointments_aggregates_query) | **POST** /api/v2/coaching/appointments/aggregates/query | Retrieve aggregated appointment data
[**post_coaching_scheduleslots_query**](CoachingApi.md#post_coaching_scheduleslots_query) | **POST** /api/v2/coaching/scheduleslots/query | Get list of possible slots where a coaching appointment can be scheduled.



## delete_coaching_appointment

> crate::models::CoachingAppointmentReference delete_coaching_appointment(appointment_id)
Delete an existing appointment

Permission not required if you are the creator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |

### Return type

[**crate::models::CoachingAppointmentReference**](CoachingAppointmentReference.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_coaching_appointment_annotation

> delete_coaching_appointment_annotation(appointment_id, annotation_id)
Delete an existing annotation

You must have the appropriate permission for the type of annotation you are updating. Permission not required if you are the creator or facilitator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**annotation_id** | **String** | The ID of the annotation. | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_appointment

> crate::models::CoachingAppointmentResponse get_coaching_appointment(appointment_id)
Retrieve an appointment

Permission not required if you are the attendee, creator or facilitator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |

### Return type

[**crate::models::CoachingAppointmentResponse**](CoachingAppointmentResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_appointment_annotation

> crate::models::CoachingAnnotation get_coaching_appointment_annotation(appointment_id, annotation_id)
Retrieve an annotation.

You must have the appropriate permission for the type of annotation you are creating. Permission not required if you are related to the appointment (only the creator or facilitator can view private annotations).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**annotation_id** | **String** | The ID of the annotation. | [required] |

### Return type

[**crate::models::CoachingAnnotation**](CoachingAnnotation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_appointment_annotations

> crate::models::CoachingAnnotationList get_coaching_appointment_annotations(appointment_id, page_number, page_size)
Get a list of annotations.

You must have the appropriate permission for the type of annotation you are creating. Permission not required if you are related to the appointment (only the creator or facilitator can view private annotations).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::CoachingAnnotationList**](CoachingAnnotationList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_appointment_statuses

> crate::models::CoachingAppointmentStatusResponseList get_coaching_appointment_statuses(appointment_id, page_number, page_size)
Get the list of status changes for a coaching appointment.

Permission not required if you are an attendee, creator or facilitator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]

### Return type

[**crate::models::CoachingAppointmentStatusResponseList**](CoachingAppointmentStatusResponseList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_appointments

> crate::models::CoachingAppointmentResponseList get_coaching_appointments(user_ids, interval, page_number, page_size, statuses, facilitator_ids, sort_order, relationships, completion_interval, overdue, interval_condition)
Get appointments for users and optional date range

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ids** | [**Vec<String>**](String.md) | The user IDs for which to retrieve appointments | [required] |
**interval** | Option<**String**> | Interval to filter data by. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**statuses** | Option<[**Vec<String>**](String.md)> | Appointment Statuses to filter by |  |
**facilitator_ids** | Option<[**Vec<String>**](String.md)> | The facilitator IDs for which to retrieve appointments |  |
**sort_order** | Option<**String**> | Sort (by due date) either Asc or Desc |  |
**relationships** | Option<[**Vec<String>**](String.md)> | Relationships to filter by |  |
**completion_interval** | Option<**String**> | Appointment completion start and end to filter by. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**overdue** | Option<**String**> | Overdue status to filter by |  |
**interval_condition** | Option<**String**> | Filter condition for interval |  |

### Return type

[**crate::models::CoachingAppointmentResponseList**](CoachingAppointmentResponseList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_appointments_me

> crate::models::CoachingAppointmentResponseList get_coaching_appointments_me(interval, page_number, page_size, statuses, facilitator_ids, sort_order, relationships, completion_interval, overdue, interval_condition)
Get my appointments for a given date range

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interval** | Option<**String**> | Interval to filter data by. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**statuses** | Option<[**Vec<String>**](String.md)> | Appointment Statuses to filter by |  |
**facilitator_ids** | Option<[**Vec<String>**](String.md)> | The facilitator IDs for which to retrieve appointments |  |
**sort_order** | Option<**String**> | Sort (by due date) either Asc or Desc |  |
**relationships** | Option<[**Vec<String>**](String.md)> | Relationships to filter by |  |
**completion_interval** | Option<**String**> | Appointment completion start and end to filter by. End date is not inclusive. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss |  |
**overdue** | Option<**String**> | Overdue status to filter by |  |
**interval_condition** | Option<**String**> | Filter condition for interval |  |

### Return type

[**crate::models::CoachingAppointmentResponseList**](CoachingAppointmentResponseList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_notification

> crate::models::CoachingNotification get_coaching_notification(notification_id, expand)
Get an existing notification

Permission not required if you are the owner of the notification.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | The ID of the notification. | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Indicates a field in the response which should be expanded. |  |

### Return type

[**crate::models::CoachingNotification**](CoachingNotification.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_coaching_notifications

> crate::models::CoachingNotificationList get_coaching_notifications(page_number, page_size, expand)
Retrieve the list of your notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**expand** | Option<[**Vec<String>**](String.md)> | Indicates a field in the response which should be expanded. |  |

### Return type

[**crate::models::CoachingNotificationList**](CoachingNotificationList.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_coaching_appointment

> crate::models::CoachingAppointmentResponse patch_coaching_appointment(appointment_id, body)
Update an existing appointment

Permission not required if you are the creator or facilitator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**body** | [**UpdateCoachingAppointmentRequest**](UpdateCoachingAppointmentRequest.md) | The new version of the appointment | [required] |

### Return type

[**crate::models::CoachingAppointmentResponse**](CoachingAppointmentResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_coaching_appointment_annotation

> crate::models::CoachingAnnotation patch_coaching_appointment_annotation(appointment_id, annotation_id, body)
Update an existing annotation.

You must have the appropriate permission for the type of annotation you are updating. Permission not required if you are the creator or facilitator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**annotation_id** | **String** | The ID of the annotation. | [required] |
**body** | [**CoachingAnnotation**](CoachingAnnotation.md) | The new version of the annotation | [required] |

### Return type

[**crate::models::CoachingAnnotation**](CoachingAnnotation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_coaching_appointment_status

> crate::models::CoachingAppointmentStatusResponse patch_coaching_appointment_status(appointment_id, body)
Update the status of a coaching appointment

Permission not required if you are an attendee, creator or facilitator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**body** | [**CoachingAppointmentStatusRequest**](CoachingAppointmentStatusRequest.md) | Updated status of the coaching appointment | [required] |

### Return type

[**crate::models::CoachingAppointmentStatusResponse**](CoachingAppointmentStatusResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_coaching_notification

> crate::models::CoachingNotification patch_coaching_notification(notification_id, body)
Update an existing notification.

Can only update your own notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** | The ID of the notification. | [required] |
**body** | [**CoachingNotification**](CoachingNotification.md) | Change the read state of a notification | [required] |

### Return type

[**crate::models::CoachingNotification**](CoachingNotification.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_coaching_appointment_annotations

> crate::models::CoachingAnnotation post_coaching_appointment_annotations(appointment_id, body)
Create a new annotation.

You must have the appropriate permission for the type of annotation you are creating. Permission not required if you are related to the appointment (only the creator or facilitator can create private annotations).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**body** | [**CoachingAnnotationCreateRequest**](CoachingAnnotationCreateRequest.md) | The annotation to add | [required] |

### Return type

[**crate::models::CoachingAnnotation**](CoachingAnnotation.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_coaching_appointment_conversations

> crate::models::AddConversationResponse post_coaching_appointment_conversations(appointment_id, body)
Add a conversation to an appointment

Permission not required if you are the creator or facilitator of the appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appointment_id** | **String** | The ID of the coaching appointment. | [required] |
**body** | [**AddConversationRequest**](AddConversationRequest.md) | body | [required] |

### Return type

[**crate::models::AddConversationResponse**](AddConversationResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_coaching_appointments

> crate::models::CoachingAppointmentResponse post_coaching_appointments(body)
Create a new appointment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateCoachingAppointmentRequest**](CreateCoachingAppointmentRequest.md) | The appointment to add | [required] |

### Return type

[**crate::models::CoachingAppointmentResponse**](CoachingAppointmentResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_coaching_appointments_aggregates_query

> crate::models::CoachingAppointmentAggregateResponse post_coaching_appointments_aggregates_query(body)
Retrieve aggregated appointment data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CoachingAppointmentAggregateRequest**](CoachingAppointmentAggregateRequest.md) | Aggregate Request | [required] |

### Return type

[**crate::models::CoachingAppointmentAggregateResponse**](CoachingAppointmentAggregateResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_coaching_scheduleslots_query

> crate::models::CoachingSlotsResponse post_coaching_scheduleslots_query(body)
Get list of possible slots where a coaching appointment can be scheduled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CoachingSlotsRequest**](CoachingSlotsRequest.md) | The slot search request | [required] |

### Return type

[**crate::models::CoachingSlotsResponse**](CoachingSlotsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

