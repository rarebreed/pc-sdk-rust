# Relationship

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**user** | [**crate::models::User**](User.md) |  | 
**external_organization** | [**crate::models::ExternalOrganization**](ExternalOrganization.md) |  | 
**relationship** | **String** | The relationship or role of the user to this external organization.Examples: Account Manager, Sales Engineer, Implementation Consultant | 
**external_data_sources** | Option<[**Vec<crate::models::ExternalDataSource>**](ExternalDataSource.md)> | Links to the sources of data (e.g. one source might be a CRM) that contributed data to this record.  Read-only, and only populated when requested via expand param. | [optional][readonly]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


