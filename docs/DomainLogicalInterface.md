# DomainLogicalInterface

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
**edge_uri** | Option<**String**> |  | [optional]
**edge_assigned_id** | Option<**String**> |  | [optional]
**friendly_name** | **String** | Friendly Name | 
**vlan_tag_id** | Option<**i32**> |  | [optional]
**hardware_address** | **String** | Hardware Address | 
**physical_adapter_id** | **String** | Physical Adapter Id | 
**if_status** | Option<**String**> |  | [optional]
**interface_type** | Option<**String**> | The type of this network interface. | [optional][readonly]
**public_nat_address_ip_v4** | Option<**String**> | IPv4 NENT IP Address | [optional]
**public_nat_address_ip_v6** | Option<**String**> | IPv6 NENT IP Address | [optional]
**routes** | Option<[**Vec<crate::models::DomainNetworkRoute>**](DomainNetworkRoute.md)> | The list of routes assigned to this interface. | [optional]
**addresses** | Option<[**Vec<crate::models::DomainNetworkAddress>**](DomainNetworkAddress.md)> | The list of IP addresses on this interface.  Priority of dns addresses are based on order in the list. | [optional]
**ipv4_capabilities** | Option<[**crate::models::DomainCapabilities**](DomainCapabilities.md)> |  | [optional]
**ipv6_capabilities** | Option<[**crate::models::DomainCapabilities**](DomainCapabilities.md)> |  | [optional]
**current_state** | Option<**String**> |  | [optional]
**last_modified_user_id** | Option<**String**> |  | [optional]
**last_modified_correlation_id** | Option<**String**> |  | [optional]
**command_responses** | Option<[**Vec<crate::models::DomainNetworkCommandResponse>**](DomainNetworkCommandResponse.md)> |  | [optional]
**inherit_phone_trunk_bases_ipv4** | Option<**bool**> | The IPv4 phone trunk base assignment will be inherited from the Edge Group. | [optional]
**inherit_phone_trunk_bases_ipv6** | Option<**bool**> | The IPv6 phone trunk base assignment will be inherited from the Edge Group. | [optional]
**use_for_internal_edge_communication** | Option<**bool**> | This interface will be used for all internal edge-to-edge communication using settings from the edgeTrunkBaseAssignment on the Edge Group. | [optional]
**use_for_indirect_edge_communication** | Option<**bool**> | Site Interconnects using the \"Indirect\" method will communicate using the Public IP Address specified on the interface. Use this option when a NAT enabled firewall is between the Edge and the far end. | [optional]
**use_for_cloud_proxy_edge_communication** | Option<**bool**> | Site Interconnects using the \"Cloud Proxy\" method will broker the connection between them with a Cloud Proxy. This method is required for connections between one or more Sites using Cloud Media, but can optionally be used between two premises Sites if Direct or Indirect are not an option. | [optional]
**use_for_wan_interface** | Option<**bool**> | This interface will be used for all communication with the internet. | [optional][readonly]
**external_trunk_base_assignments** | Option<[**Vec<crate::models::TrunkBaseAssignment>**](TrunkBaseAssignment.md)> | External trunk base settings to use for external communication from this interface. | [optional]
**phone_trunk_base_assignments** | Option<[**Vec<crate::models::TrunkBaseAssignment>**](TrunkBaseAssignment.md)> | Phone trunk base settings to use for phone communication from this interface.  These settings will be ignored when \"inheritPhoneTrunkBases\" is true. | [optional]
**trace_enabled** | Option<**bool**> |  | [optional]
**start_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**end_date** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


