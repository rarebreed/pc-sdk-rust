# Edge

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | **String** | The name of the entity. | 
**division** | Option<[**crate::models::Division**](Division.md)> |  | [optional]
**description** | Option<**String**> | The resource's description. | [optional]
**version** | Option<**i32**> | The current version of the resource. | [optional]
**date_created** | Option<**String**> | The date the resource was created. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**date_modified** | Option<**String**> | The date of the last modification to the resource. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**modified_by** | Option<**String**> | The ID of the user that last modified the resource. | [optional]
**created_by** | Option<**String**> | The ID of the user that created the resource. | [optional]
**state** | Option<**String**> | Indicates if the resource is active, inactive, or deleted. | [optional][readonly]
**modified_by_app** | Option<**String**> | The application that last modified the resource. | [optional]
**created_by_app** | Option<**String**> | The application that created the resource. | [optional]
**interfaces** | Option<[**Vec<crate::models::EdgeInterface>**](EdgeInterface.md)> | The list of interfaces for the edge. (Deprecated) Replaced by configuring trunks/ip info on the logical interface instead | [optional]
**make** | Option<**String**> |  | [optional]
**model** | Option<**String**> |  | [optional]
**api_version** | Option<**String**> |  | [optional]
**software_version** | Option<**String**> |  | [optional]
**software_version_timestamp** | Option<**String**> |  | [optional]
**software_version_platform** | Option<**String**> |  | [optional]
**software_version_configuration** | Option<**String**> |  | [optional]
**full_software_version** | Option<**String**> |  | [optional]
**pairing_id** | Option<**String**> | The pairing Id for a hardware Edge in the format: 00000-00000-00000-00000-00000. This field is only required when creating an Edge with a deployment type of HARDWARE. | [optional]
**fingerprint** | Option<**String**> |  | [optional]
**fingerprint_hint** | Option<**String**> |  | [optional]
**current_version** | Option<**String**> |  | [optional]
**staged_version** | Option<**String**> |  | [optional]
**patch** | Option<**String**> |  | [optional]
**status_code** | Option<**String**> | The current status of the Edge. | [optional]
**edge_group** | Option<[**crate::models::EdgeGroup**](EdgeGroup.md)> |  | [optional]
**site** | Option<[**crate::models::Site**](Site.md)> |  | [optional]
**software_status** | Option<[**crate::models::DomainEdgeSoftwareUpdateDto**](DomainEdgeSoftwareUpdateDto.md)> |  | [optional]
**online_status** | Option<**String**> |  | [optional]
**serial_number** | Option<**String**> |  | [optional]
**physical_edge** | Option<**bool**> |  | [optional]
**managed** | Option<**bool**> |  | [optional]
**edge_deployment_type** | Option<**String**> |  | [optional]
**call_draining_state** | Option<**String**> | The current state of the Edge's call draining process before it can be safely rebooted or updated. | [optional][readonly]
**conversation_count** | Option<**i32**> | The remaining number of conversations the Edge has to drain before it can be safely rebooted or updated. When an Edge is not draining conversations, this will be NULL or 0. | [optional][readonly]
**proxy** | Option<**String**> | Edge HTTP proxy configuration for the WAN port. The field can be a hostname, FQDN, IPv4 or IPv6 address. If port is not included, port 80 is assumed. | [optional]
**offline_config_called** | Option<**bool**> | True if the offline edge configuration endpoint has been called for this edge. | [optional][readonly]
**os_name** | Option<**String**> | The name provided by the operating system of the Edge. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


