# PhoneStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**operational_status** | Option<**String**> | The Operational Status of this phone | [optional]
**edges_status** | Option<**String**> | The status of the primary or secondary Edges assigned to the phone lines. | [optional]
**event_creation_time** | Option<**String**> | Event Creation Time represents an ISO-8601 string. For example: UTC, UTC+01:00, or Europe/London | [optional]
**provision** | Option<[**crate::models::ProvisionInfo**](ProvisionInfo.md)> |  | [optional]
**line_statuses** | Option<[**Vec<crate::models::LineStatus>**](LineStatus.md)> | A list of LineStatus information for each of the lines of this phone | [optional]
**phone_assignment_to_edge_type** | Option<**String**> | The phone status's edge assignment type. | [optional]
**edge** | Option<[**crate::models::DomainEntityRef**](DomainEntityRef.md)> |  | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


