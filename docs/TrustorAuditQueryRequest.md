# TrustorAuditQueryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trustor_organization_id** | **String** | Limit returned audits to this trustor organizationId. | 
**trustee_user_ids** | **Vec<String>** | Limit returned audits to these trustee userIds. | 
**start_date** | Option<**String**> | Starting date/time for the audit search. ISO-8601 formatted date-time, UTC. | [optional]
**end_date** | Option<**String**> | Ending date/time for the audit search. ISO-8601 formatted date-time, UTC. | [optional]
**query_phrase** | Option<**String**> | Word or phrase to look for in audit bodies. | [optional]
**facets** | Option<[**Vec<crate::models::Facet>**](Facet.md)> | Facet information to be returned with the query results. | [optional]
**filters** | Option<[**Vec<crate::models::Filter>**](Filter.md)> | Additional custom filters to be applied to the query. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


