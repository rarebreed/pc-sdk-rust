# Site

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
**primary_sites** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> |  | [optional]
**secondary_sites** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> |  | [optional]
**primary_edges** | Option<[**Vec<crate::models::Edge>**](Edge.md)> |  | [optional]
**secondary_edges** | Option<[**Vec<crate::models::Edge>**](Edge.md)> |  | [optional]
**addresses** | Option<[**Vec<crate::models::Contact>**](Contact.md)> |  | [optional]
**edges** | Option<[**Vec<crate::models::Edge>**](Edge.md)> |  | [optional]
**edge_auto_update_config** | Option<[**crate::models::EdgeAutoUpdateConfig**](EdgeAutoUpdateConfig.md)> |  | [optional]
**media_regions_use_latency_based** | Option<**bool**> |  | [optional]
**location** | [**crate::models::LocationDefinition**](LocationDefinition.md) |  | 
**managed** | Option<**bool**> |  | [optional]
**ntp_settings** | Option<[**crate::models::NtpSettings**](NTPSettings.md)> |  | [optional]
**media_model** | Option<**String**> | Media model for the site | [optional]
**core_site** | Option<**bool**> | Is this site a core site | [optional]
**site_connections** | Option<[**Vec<crate::models::SiteConnection>**](SiteConnection.md)> | The site connections | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


