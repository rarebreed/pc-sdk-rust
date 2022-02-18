# SiteConnection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**self_uri** | Option<**String**> |  | [optional]
**managed** | Option<**bool**> |  | [optional]
**_type** | Option<**String**> | Connection method from site to site (Direct, Indirect, CloudProxy | [optional]
**enabled** | Option<**bool**> | Indicates if the current site is linked | [optional]
**media_model** | Option<**String**> | Media model for the current site. | [optional][readonly]
**edge_list** | Option<[**Vec<crate::models::ConnectedEdge>**](ConnectedEdge.md)> | All of the edges to which the site connects | [optional][readonly]
**core_site** | Option<**bool**> | The core site | [optional][readonly]
**primary_core_sites** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | List of site ids names and selfUris for the primary core sites | [optional][readonly]
**secondary_core_sites** | Option<[**Vec<crate::models::DomainEntityRef>**](DomainEntityRef.md)> | List of site ids names and selfUris for the secondary core sites | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


