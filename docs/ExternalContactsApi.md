# \ExternalContactsApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_externalcontacts_contact**](ExternalContactsApi.md#delete_externalcontacts_contact) | **DELETE** /api/v2/externalcontacts/contacts/{contactId} | Delete an external contact
[**delete_externalcontacts_contact_note**](ExternalContactsApi.md#delete_externalcontacts_contact_note) | **DELETE** /api/v2/externalcontacts/contacts/{contactId}/notes/{noteId} | Delete a note for an external contact
[**delete_externalcontacts_contacts_schema**](ExternalContactsApi.md#delete_externalcontacts_contacts_schema) | **DELETE** /api/v2/externalcontacts/contacts/schemas/{schemaId} | Delete a schema
[**delete_externalcontacts_organization**](ExternalContactsApi.md#delete_externalcontacts_organization) | **DELETE** /api/v2/externalcontacts/organizations/{externalOrganizationId} | Delete an external organization
[**delete_externalcontacts_organization_note**](ExternalContactsApi.md#delete_externalcontacts_organization_note) | **DELETE** /api/v2/externalcontacts/organizations/{externalOrganizationId}/notes/{noteId} | Delete a note for an external organization
[**delete_externalcontacts_organization_trustor**](ExternalContactsApi.md#delete_externalcontacts_organization_trustor) | **DELETE** /api/v2/externalcontacts/organizations/{externalOrganizationId}/trustor | Unlink the Trustor for this External Organization
[**delete_externalcontacts_relationship**](ExternalContactsApi.md#delete_externalcontacts_relationship) | **DELETE** /api/v2/externalcontacts/relationships/{relationshipId} | Delete a relationship
[**get_externalcontacts_contact**](ExternalContactsApi.md#get_externalcontacts_contact) | **GET** /api/v2/externalcontacts/contacts/{contactId} | Fetch an external contact
[**get_externalcontacts_contact_note**](ExternalContactsApi.md#get_externalcontacts_contact_note) | **GET** /api/v2/externalcontacts/contacts/{contactId}/notes/{noteId} | Fetch a note for an external contact
[**get_externalcontacts_contact_notes**](ExternalContactsApi.md#get_externalcontacts_contact_notes) | **GET** /api/v2/externalcontacts/contacts/{contactId}/notes | List notes for an external contact
[**get_externalcontacts_contacts**](ExternalContactsApi.md#get_externalcontacts_contacts) | **GET** /api/v2/externalcontacts/contacts | Search for external contacts
[**get_externalcontacts_contacts_schema**](ExternalContactsApi.md#get_externalcontacts_contacts_schema) | **GET** /api/v2/externalcontacts/contacts/schemas/{schemaId} | Get a schema
[**get_externalcontacts_contacts_schema_version**](ExternalContactsApi.md#get_externalcontacts_contacts_schema_version) | **GET** /api/v2/externalcontacts/contacts/schemas/{schemaId}/versions/{versionId} | Get a specific version of a schema
[**get_externalcontacts_contacts_schema_versions**](ExternalContactsApi.md#get_externalcontacts_contacts_schema_versions) | **GET** /api/v2/externalcontacts/contacts/schemas/{schemaId}/versions | Get all versions of an external contact's schema
[**get_externalcontacts_contacts_schemas**](ExternalContactsApi.md#get_externalcontacts_contacts_schemas) | **GET** /api/v2/externalcontacts/contacts/schemas | Get a list of schemas.
[**get_externalcontacts_organization**](ExternalContactsApi.md#get_externalcontacts_organization) | **GET** /api/v2/externalcontacts/organizations/{externalOrganizationId} | Fetch an external organization
[**get_externalcontacts_organization_contacts**](ExternalContactsApi.md#get_externalcontacts_organization_contacts) | **GET** /api/v2/externalcontacts/organizations/{externalOrganizationId}/contacts | Search for external contacts in an external organization
[**get_externalcontacts_organization_note**](ExternalContactsApi.md#get_externalcontacts_organization_note) | **GET** /api/v2/externalcontacts/organizations/{externalOrganizationId}/notes/{noteId} | Fetch a note for an external organization
[**get_externalcontacts_organization_notes**](ExternalContactsApi.md#get_externalcontacts_organization_notes) | **GET** /api/v2/externalcontacts/organizations/{externalOrganizationId}/notes | List notes for an external organization
[**get_externalcontacts_organization_relationships**](ExternalContactsApi.md#get_externalcontacts_organization_relationships) | **GET** /api/v2/externalcontacts/organizations/{externalOrganizationId}/relationships | Fetch a relationship for an external organization
[**get_externalcontacts_organizations**](ExternalContactsApi.md#get_externalcontacts_organizations) | **GET** /api/v2/externalcontacts/organizations | Search for external organizations
[**get_externalcontacts_organizations_schema**](ExternalContactsApi.md#get_externalcontacts_organizations_schema) | **GET** /api/v2/externalcontacts/organizations/schemas/{schemaId} | Get a schema
[**get_externalcontacts_organizations_schema_version**](ExternalContactsApi.md#get_externalcontacts_organizations_schema_version) | **GET** /api/v2/externalcontacts/organizations/schemas/{schemaId}/versions/{versionId} | Get a specific version of a schema
[**get_externalcontacts_organizations_schema_versions**](ExternalContactsApi.md#get_externalcontacts_organizations_schema_versions) | **GET** /api/v2/externalcontacts/organizations/schemas/{schemaId}/versions | Get all versions of an external organization's schema
[**get_externalcontacts_organizations_schemas**](ExternalContactsApi.md#get_externalcontacts_organizations_schemas) | **GET** /api/v2/externalcontacts/organizations/schemas | Get a list of schemas.
[**get_externalcontacts_relationship**](ExternalContactsApi.md#get_externalcontacts_relationship) | **GET** /api/v2/externalcontacts/relationships/{relationshipId} | Fetch a relationship
[**get_externalcontacts_reversewhitepageslookup**](ExternalContactsApi.md#get_externalcontacts_reversewhitepageslookup) | **GET** /api/v2/externalcontacts/reversewhitepageslookup | Look up contacts and externalOrganizations based on an attribute. Maximum of 25 values returned.
[**get_externalcontacts_scan_contacts**](ExternalContactsApi.md#get_externalcontacts_scan_contacts) | **GET** /api/v2/externalcontacts/scan/contacts | Scan for external contacts using paging
[**get_externalcontacts_scan_notes**](ExternalContactsApi.md#get_externalcontacts_scan_notes) | **GET** /api/v2/externalcontacts/scan/notes | Scan for notes using paging
[**get_externalcontacts_scan_organizations**](ExternalContactsApi.md#get_externalcontacts_scan_organizations) | **GET** /api/v2/externalcontacts/scan/organizations | Scan for external organizations using paging
[**get_externalcontacts_scan_relationships**](ExternalContactsApi.md#get_externalcontacts_scan_relationships) | **GET** /api/v2/externalcontacts/scan/relationships | Scan for relationships
[**post_externalcontacts_bulk_contacts**](ExternalContactsApi.md#post_externalcontacts_bulk_contacts) | **POST** /api/v2/externalcontacts/bulk/contacts | Bulk fetch contacts
[**post_externalcontacts_bulk_contacts_add**](ExternalContactsApi.md#post_externalcontacts_bulk_contacts_add) | **POST** /api/v2/externalcontacts/bulk/contacts/add | Bulk add contacts
[**post_externalcontacts_bulk_contacts_remove**](ExternalContactsApi.md#post_externalcontacts_bulk_contacts_remove) | **POST** /api/v2/externalcontacts/bulk/contacts/remove | Bulk remove contacts
[**post_externalcontacts_bulk_contacts_update**](ExternalContactsApi.md#post_externalcontacts_bulk_contacts_update) | **POST** /api/v2/externalcontacts/bulk/contacts/update | Bulk update contacts
[**post_externalcontacts_bulk_notes**](ExternalContactsApi.md#post_externalcontacts_bulk_notes) | **POST** /api/v2/externalcontacts/bulk/notes | Bulk fetch notes
[**post_externalcontacts_bulk_notes_add**](ExternalContactsApi.md#post_externalcontacts_bulk_notes_add) | **POST** /api/v2/externalcontacts/bulk/notes/add | Bulk add notes
[**post_externalcontacts_bulk_notes_remove**](ExternalContactsApi.md#post_externalcontacts_bulk_notes_remove) | **POST** /api/v2/externalcontacts/bulk/notes/remove | Bulk remove notes
[**post_externalcontacts_bulk_notes_update**](ExternalContactsApi.md#post_externalcontacts_bulk_notes_update) | **POST** /api/v2/externalcontacts/bulk/notes/update | Bulk update notes
[**post_externalcontacts_bulk_organizations**](ExternalContactsApi.md#post_externalcontacts_bulk_organizations) | **POST** /api/v2/externalcontacts/bulk/organizations | Bulk fetch organizations
[**post_externalcontacts_bulk_organizations_add**](ExternalContactsApi.md#post_externalcontacts_bulk_organizations_add) | **POST** /api/v2/externalcontacts/bulk/organizations/add | Bulk add organizations
[**post_externalcontacts_bulk_organizations_remove**](ExternalContactsApi.md#post_externalcontacts_bulk_organizations_remove) | **POST** /api/v2/externalcontacts/bulk/organizations/remove | Bulk remove organizations
[**post_externalcontacts_bulk_organizations_update**](ExternalContactsApi.md#post_externalcontacts_bulk_organizations_update) | **POST** /api/v2/externalcontacts/bulk/organizations/update | Bulk update organizations
[**post_externalcontacts_bulk_relationships**](ExternalContactsApi.md#post_externalcontacts_bulk_relationships) | **POST** /api/v2/externalcontacts/bulk/relationships | Bulk fetch relationships
[**post_externalcontacts_bulk_relationships_add**](ExternalContactsApi.md#post_externalcontacts_bulk_relationships_add) | **POST** /api/v2/externalcontacts/bulk/relationships/add | Bulk add relationships
[**post_externalcontacts_bulk_relationships_remove**](ExternalContactsApi.md#post_externalcontacts_bulk_relationships_remove) | **POST** /api/v2/externalcontacts/bulk/relationships/remove | Bulk remove relationships
[**post_externalcontacts_bulk_relationships_update**](ExternalContactsApi.md#post_externalcontacts_bulk_relationships_update) | **POST** /api/v2/externalcontacts/bulk/relationships/update | Bulk update relationships
[**post_externalcontacts_contact_notes**](ExternalContactsApi.md#post_externalcontacts_contact_notes) | **POST** /api/v2/externalcontacts/contacts/{contactId}/notes | Create a note for an external contact
[**post_externalcontacts_contacts**](ExternalContactsApi.md#post_externalcontacts_contacts) | **POST** /api/v2/externalcontacts/contacts | Create an external contact
[**post_externalcontacts_contacts_schemas**](ExternalContactsApi.md#post_externalcontacts_contacts_schemas) | **POST** /api/v2/externalcontacts/contacts/schemas | Create a schema
[**post_externalcontacts_organization_notes**](ExternalContactsApi.md#post_externalcontacts_organization_notes) | **POST** /api/v2/externalcontacts/organizations/{externalOrganizationId}/notes | Create a note for an external organization
[**post_externalcontacts_organizations**](ExternalContactsApi.md#post_externalcontacts_organizations) | **POST** /api/v2/externalcontacts/organizations | Create an external organization
[**post_externalcontacts_organizations_schemas**](ExternalContactsApi.md#post_externalcontacts_organizations_schemas) | **POST** /api/v2/externalcontacts/organizations/schemas | Create a schema
[**post_externalcontacts_relationships**](ExternalContactsApi.md#post_externalcontacts_relationships) | **POST** /api/v2/externalcontacts/relationships | Create a relationship
[**put_externalcontacts_contact**](ExternalContactsApi.md#put_externalcontacts_contact) | **PUT** /api/v2/externalcontacts/contacts/{contactId} | Update an external contact
[**put_externalcontacts_contact_note**](ExternalContactsApi.md#put_externalcontacts_contact_note) | **PUT** /api/v2/externalcontacts/contacts/{contactId}/notes/{noteId} | Update a note for an external contact
[**put_externalcontacts_contacts_schema**](ExternalContactsApi.md#put_externalcontacts_contacts_schema) | **PUT** /api/v2/externalcontacts/contacts/schemas/{schemaId} | Update a schema
[**put_externalcontacts_conversation**](ExternalContactsApi.md#put_externalcontacts_conversation) | **PUT** /api/v2/externalcontacts/conversations/{conversationId} | Associate/disassociate an external contact with a conversation
[**put_externalcontacts_organization**](ExternalContactsApi.md#put_externalcontacts_organization) | **PUT** /api/v2/externalcontacts/organizations/{externalOrganizationId} | Update an external organization
[**put_externalcontacts_organization_note**](ExternalContactsApi.md#put_externalcontacts_organization_note) | **PUT** /api/v2/externalcontacts/organizations/{externalOrganizationId}/notes/{noteId} | Update a note for an external organization
[**put_externalcontacts_organization_trustor_trustor_id**](ExternalContactsApi.md#put_externalcontacts_organization_trustor_trustor_id) | **PUT** /api/v2/externalcontacts/organizations/{externalOrganizationId}/trustor/{trustorId} | Links a Trustor with an External Organization
[**put_externalcontacts_organizations_schema**](ExternalContactsApi.md#put_externalcontacts_organizations_schema) | **PUT** /api/v2/externalcontacts/organizations/schemas/{schemaId} | Update a schema
[**put_externalcontacts_relationship**](ExternalContactsApi.md#put_externalcontacts_relationship) | **PUT** /api/v2/externalcontacts/relationships/{relationshipId} | Update a relationship



## delete_externalcontacts_contact

> serde_json::Value delete_externalcontacts_contact(contact_id)
Delete an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_externalcontacts_contact_note

> serde_json::Value delete_externalcontacts_contact_note(contact_id, note_id)
Delete a note for an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact Id | [required] |
**note_id** | **String** | Note Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_externalcontacts_contacts_schema

> delete_externalcontacts_contacts_schema(schema_id)
Delete a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_externalcontacts_organization

> serde_json::Value delete_externalcontacts_organization(external_organization_id)
Delete an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization ID | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_externalcontacts_organization_note

> serde_json::Value delete_externalcontacts_organization_note(external_organization_id, note_id)
Delete a note for an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization Id | [required] |
**note_id** | **String** | Note Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_externalcontacts_organization_trustor

> delete_externalcontacts_organization_trustor(external_organization_id)
Unlink the Trustor for this External Organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_externalcontacts_relationship

> serde_json::Value delete_externalcontacts_relationship(relationship_id)
Delete a relationship

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**relationship_id** | **String** | Relationship Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contact

> crate::models::ExternalContact get_externalcontacts_contact(contact_id, expand)
Fetch an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand (externalOrganization,externalDataSources) |  |

### Return type

[**crate::models::ExternalContact**](ExternalContact.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contact_note

> crate::models::Note get_externalcontacts_contact_note(contact_id, note_id, expand)
Fetch a note for an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact Id | [required] |
**note_id** | **String** | Note Id | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand |  |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contact_notes

> crate::models::NoteListing get_externalcontacts_contact_notes(contact_id, page_size, page_number, sort_order, expand)
List notes for an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact Id | [required] |
**page_size** | Option<**i32**> | Page size (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 20]
**page_number** | Option<**i32**> | Page number (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 1]
**sort_order** | Option<**String**> | Sort order |  |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand |  |

### Return type

[**crate::models::NoteListing**](NoteListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contacts

> crate::models::ContactListing get_externalcontacts_contacts(page_size, page_number, q, sort_order, expand)
Search for external contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 20]
**page_number** | Option<**i32**> | Page number (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 1]
**q** | Option<**String**> | User supplied search keywords (no special syntax is currently supported) |  |
**sort_order** | Option<**String**> | Sort order |  |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand |  |

### Return type

[**crate::models::ContactListing**](ContactListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contacts_schema

> crate::models::DataSchema get_externalcontacts_contacts_schema(schema_id)
Get a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contacts_schema_version

> crate::models::DataSchema get_externalcontacts_contacts_schema_version(schema_id, version_id)
Get a specific version of a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |
**version_id** | **String** | Schema version | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contacts_schema_versions

> crate::models::DataSchema get_externalcontacts_contacts_schema_versions(schema_id)
Get all versions of an external contact's schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_contacts_schemas

> crate::models::DataSchemaListing get_externalcontacts_contacts_schemas()
Get a list of schemas.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataSchemaListing**](DataSchemaListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organization

> crate::models::ExternalOrganization get_externalcontacts_organization(external_organization_id, expand, include_trustors)
Fetch an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization ID | [required] |
**expand** | Option<**String**> | which fields, if any, to expand (externalDataSources) |  |
**include_trustors** | Option<**bool**> | (true or false) whether or not to include trustor information embedded in the externalOrganization |  |

### Return type

[**crate::models::ExternalOrganization**](ExternalOrganization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organization_contacts

> crate::models::ContactListing get_externalcontacts_organization_contacts(external_organization_id, page_size, page_number, q, sort_order, expand)
Search for external contacts in an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization ID | [required] |
**page_size** | Option<**i32**> | Page size (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 20]
**page_number** | Option<**i32**> | Page number (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 1]
**q** | Option<**String**> | User supplied search keywords (no special syntax is currently supported) |  |
**sort_order** | Option<**String**> | Sort order |  |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand |  |

### Return type

[**crate::models::ContactListing**](ContactListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organization_note

> crate::models::Note get_externalcontacts_organization_note(external_organization_id, note_id, expand)
Fetch a note for an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization Id | [required] |
**note_id** | **String** | Note Id | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand |  |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organization_notes

> crate::models::NoteListing get_externalcontacts_organization_notes(external_organization_id, page_size, page_number, sort_order, expand)
List notes for an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization Id | [required] |
**page_size** | Option<**i32**> | Page size (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 20]
**page_number** | Option<**i32**> | Page number (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 1]
**sort_order** | Option<**String**> | Sort order |  |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand |  |

### Return type

[**crate::models::NoteListing**](NoteListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organization_relationships

> crate::models::RelationshipListing get_externalcontacts_organization_relationships(external_organization_id, page_size, page_number, expand, sort_order)
Fetch a relationship for an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization ID | [required] |
**page_size** | Option<**i32**> | Page size (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 20]
**page_number** | Option<**i32**> | Page number (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 1]
**expand** | Option<**String**> | which fields, if any, to expand |  |
**sort_order** | Option<**String**> | Sort order |  |

### Return type

[**crate::models::RelationshipListing**](RelationshipListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organizations

> crate::models::ExternalOrganizationListing get_externalcontacts_organizations(page_size, page_number, q, trustor_id, sort_order, expand, include_trustors)
Search for external organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 20]
**page_number** | Option<**i32**> | Page number (limited to fetching first 1,000 records; pageNumber * pageSize must be <= 1,000) |  |[default to 1]
**q** | Option<**String**> | Search query |  |
**trustor_id** | Option<[**Vec<String>**](String.md)> | Search for external organizations by trustorIds (limit 25). If supplied, the 'q' parameters is ignored. Items are returned in the order requested |  |
**sort_order** | Option<**String**> | Sort order |  |
**expand** | Option<[**Vec<String>**](String.md)> | which fields, if any, to expand |  |
**include_trustors** | Option<**bool**> | (true or false) whether or not to include trustor information embedded in the externalOrganization |  |

### Return type

[**crate::models::ExternalOrganizationListing**](ExternalOrganizationListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organizations_schema

> crate::models::DataSchema get_externalcontacts_organizations_schema(schema_id)
Get a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organizations_schema_version

> crate::models::DataSchema get_externalcontacts_organizations_schema_version(schema_id, version_id)
Get a specific version of a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |
**version_id** | **String** | Schema version | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organizations_schema_versions

> crate::models::DataSchema get_externalcontacts_organizations_schema_versions(schema_id)
Get all versions of an external organization's schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_organizations_schemas

> crate::models::DataSchemaListing get_externalcontacts_organizations_schemas()
Get a list of schemas.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DataSchemaListing**](DataSchemaListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_relationship

> crate::models::Relationship get_externalcontacts_relationship(relationship_id, expand)
Fetch a relationship

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**relationship_id** | **String** | Relationship Id | [required] |
**expand** | Option<**String**> | which fields, if any, to expand |  |

### Return type

[**crate::models::Relationship**](Relationship.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_reversewhitepageslookup

> crate::models::ReverseWhitepagesLookupResult get_externalcontacts_reversewhitepageslookup(lookup_val, expand)
Look up contacts and externalOrganizations based on an attribute. Maximum of 25 values returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lookup_val** | **String** | User supplied value to lookup contacts/externalOrganizations (supports email addresses, e164 phone numbers, Twitter screen names) | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | which field, if any, to expand |  |

### Return type

[**crate::models::ReverseWhitepagesLookupResult**](ReverseWhitepagesLookupResult.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_scan_contacts

> crate::models::CursorContactListing get_externalcontacts_scan_contacts(limit, cursor)
Scan for external contacts using paging

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of contacts per page; must be between 10 and 200, default is 100) |  |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page), each page returns a new cursor with a 24h TTL |  |

### Return type

[**crate::models::CursorContactListing**](CursorContactListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_scan_notes

> crate::models::CursorNoteListing get_externalcontacts_scan_notes(limit, cursor)
Scan for notes using paging

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of notes per page; must be between 10 and 200, default is 100) |  |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page), each page returns a new cursor with a 24h TTL |  |

### Return type

[**crate::models::CursorNoteListing**](CursorNoteListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_scan_organizations

> crate::models::CursorOrganizationListing get_externalcontacts_scan_organizations(limit, cursor)
Scan for external organizations using paging

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of organizations per page; must be between 10 and 200, default is 100) |  |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page), each page returns a new cursor with a 24h TTL |  |

### Return type

[**crate::models::CursorOrganizationListing**](CursorOrganizationListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_externalcontacts_scan_relationships

> crate::models::CursorRelationshipListing get_externalcontacts_scan_relationships(limit, cursor)
Scan for relationships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | The number of relationships per page; must be between 10 and 200, default is 100) |  |
**cursor** | Option<**String**> | Indicates where to resume query results (not required for first page), each page returns a new cursor with a 24h TTL |  |

### Return type

[**crate::models::CursorRelationshipListing**](CursorRelationshipListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_contacts

> crate::models::BulkFetchContactsResponse post_externalcontacts_bulk_contacts(body)
Bulk fetch contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Contact ids | [required] |

### Return type

[**crate::models::BulkFetchContactsResponse**](BulkFetchContactsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_contacts_add

> crate::models::BulkContactsResponse post_externalcontacts_bulk_contacts_add(body)
Bulk add contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkContactsRequest**](BulkContactsRequest.md) | Contacts | [required] |

### Return type

[**crate::models::BulkContactsResponse**](BulkContactsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_contacts_remove

> crate::models::BulkDeleteResponse post_externalcontacts_bulk_contacts_remove(body)
Bulk remove contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Contact ids | [required] |

### Return type

[**crate::models::BulkDeleteResponse**](BulkDeleteResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_contacts_update

> crate::models::BulkContactsResponse post_externalcontacts_bulk_contacts_update(body)
Bulk update contacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkContactsRequest**](BulkContactsRequest.md) | Contacts | [required] |

### Return type

[**crate::models::BulkContactsResponse**](BulkContactsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_notes

> crate::models::BulkFetchNotesResponse post_externalcontacts_bulk_notes(body)
Bulk fetch notes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Note ids | [required] |

### Return type

[**crate::models::BulkFetchNotesResponse**](BulkFetchNotesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_notes_add

> crate::models::BulkNotesResponse post_externalcontacts_bulk_notes_add(body)
Bulk add notes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkNotesRequest**](BulkNotesRequest.md) | Notes | [required] |

### Return type

[**crate::models::BulkNotesResponse**](BulkNotesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_notes_remove

> crate::models::BulkDeleteResponse post_externalcontacts_bulk_notes_remove(body)
Bulk remove notes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Note ids | [required] |

### Return type

[**crate::models::BulkDeleteResponse**](BulkDeleteResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_notes_update

> crate::models::BulkNotesResponse post_externalcontacts_bulk_notes_update(body)
Bulk update notes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkNotesRequest**](BulkNotesRequest.md) | Notes | [required] |

### Return type

[**crate::models::BulkNotesResponse**](BulkNotesResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_organizations

> crate::models::BulkFetchOrganizationsResponse post_externalcontacts_bulk_organizations(body)
Bulk fetch organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Organizations ids | [required] |

### Return type

[**crate::models::BulkFetchOrganizationsResponse**](BulkFetchOrganizationsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_organizations_add

> crate::models::BulkOrganizationsResponse post_externalcontacts_bulk_organizations_add(body)
Bulk add organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkOrganizationsRequest**](BulkOrganizationsRequest.md) | Organizations | [required] |

### Return type

[**crate::models::BulkOrganizationsResponse**](BulkOrganizationsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_organizations_remove

> crate::models::BulkDeleteResponse post_externalcontacts_bulk_organizations_remove(body)
Bulk remove organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Organization ids | [required] |

### Return type

[**crate::models::BulkDeleteResponse**](BulkDeleteResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_organizations_update

> crate::models::BulkOrganizationsResponse post_externalcontacts_bulk_organizations_update(body)
Bulk update organizations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkOrganizationsRequest**](BulkOrganizationsRequest.md) | Organizations | [required] |

### Return type

[**crate::models::BulkOrganizationsResponse**](BulkOrganizationsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_relationships

> crate::models::BulkFetchRelationshipsResponse post_externalcontacts_bulk_relationships(body)
Bulk fetch relationships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Relationships ids | [required] |

### Return type

[**crate::models::BulkFetchRelationshipsResponse**](BulkFetchRelationshipsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_relationships_add

> crate::models::BulkRelationshipsResponse post_externalcontacts_bulk_relationships_add(body)
Bulk add relationships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkRelationshipsRequest**](BulkRelationshipsRequest.md) | Relationships | [required] |

### Return type

[**crate::models::BulkRelationshipsResponse**](BulkRelationshipsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_relationships_remove

> crate::models::BulkDeleteResponse post_externalcontacts_bulk_relationships_remove(body)
Bulk remove relationships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkIdsRequest**](BulkIdsRequest.md) | Relationships ids | [required] |

### Return type

[**crate::models::BulkDeleteResponse**](BulkDeleteResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_bulk_relationships_update

> crate::models::BulkRelationshipsResponse post_externalcontacts_bulk_relationships_update(body)
Bulk update relationships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**BulkRelationshipsRequest**](BulkRelationshipsRequest.md) | Relationships | [required] |

### Return type

[**crate::models::BulkRelationshipsResponse**](BulkRelationshipsResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_contact_notes

> crate::models::Note post_externalcontacts_contact_notes(contact_id, body)
Create a note for an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact Id | [required] |
**body** | [**Note**](Note.md) | ExternalContact | [required] |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_contacts

> crate::models::ExternalContact post_externalcontacts_contacts(body)
Create an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ExternalContact**](ExternalContact.md) | ExternalContact | [required] |

### Return type

[**crate::models::ExternalContact**](ExternalContact.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_contacts_schemas

> crate::models::DataSchema post_externalcontacts_contacts_schemas(body)
Create a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DataSchema**](DataSchema.md) | Schema | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_organization_notes

> crate::models::Note post_externalcontacts_organization_notes(external_organization_id, body)
Create a note for an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization Id | [required] |
**body** | [**Note**](Note.md) | ExternalContact | [required] |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_organizations

> crate::models::ExternalOrganization post_externalcontacts_organizations(body)
Create an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ExternalOrganization**](ExternalOrganization.md) | ExternalOrganization | [required] |

### Return type

[**crate::models::ExternalOrganization**](ExternalOrganization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_organizations_schemas

> crate::models::DataSchema post_externalcontacts_organizations_schemas(body)
Create a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DataSchema**](DataSchema.md) | Schema | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_externalcontacts_relationships

> crate::models::Relationship post_externalcontacts_relationships(body)
Create a relationship

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Relationship**](Relationship.md) | Relationship | [required] |

### Return type

[**crate::models::Relationship**](Relationship.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_contact

> crate::models::ExternalContact put_externalcontacts_contact(contact_id, body)
Update an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact ID | [required] |
**body** | [**ExternalContact**](ExternalContact.md) | ExternalContact | [required] |

### Return type

[**crate::models::ExternalContact**](ExternalContact.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_contact_note

> crate::models::Note put_externalcontacts_contact_note(contact_id, note_id, body)
Update a note for an external contact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | ExternalContact Id | [required] |
**note_id** | **String** | Note Id | [required] |
**body** | [**Note**](Note.md) | Note | [required] |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_contacts_schema

> crate::models::DataSchema put_externalcontacts_contacts_schema(schema_id, body)
Update a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |
**body** | [**DataSchema**](DataSchema.md) | Data Schema | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_conversation

> put_externalcontacts_conversation(conversation_id, body)
Associate/disassociate an external contact with a conversation

To associate, supply a value for the externalContactId.  To disassociate, do not include the property at all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Conversation ID | [required] |
**body** | [**ConversationAssociation**](ConversationAssociation.md) | ConversationAssociation | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_organization

> crate::models::ExternalOrganization put_externalcontacts_organization(external_organization_id, body)
Update an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization ID | [required] |
**body** | [**ExternalOrganization**](ExternalOrganization.md) | ExternalOrganization | [required] |

### Return type

[**crate::models::ExternalOrganization**](ExternalOrganization.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_organization_note

> crate::models::Note put_externalcontacts_organization_note(external_organization_id, note_id, body)
Update a note for an external organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization Id | [required] |
**note_id** | **String** | Note Id | [required] |
**body** | [**Note**](Note.md) | Note | [required] |

### Return type

[**crate::models::Note**](Note.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_organization_trustor_trustor_id

> crate::models::ExternalOrganizationTrustorLink put_externalcontacts_organization_trustor_trustor_id(external_organization_id, trustor_id)
Links a Trustor with an External Organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_organization_id** | **String** | External Organization ID | [required] |
**trustor_id** | **String** | Trustor ID | [required] |

### Return type

[**crate::models::ExternalOrganizationTrustorLink**](ExternalOrganizationTrustorLink.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_organizations_schema

> crate::models::DataSchema put_externalcontacts_organizations_schema(schema_id, body)
Update a schema

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schema_id** | **String** | Schema ID | [required] |
**body** | [**DataSchema**](DataSchema.md) | Data Schema | [required] |

### Return type

[**crate::models::DataSchema**](DataSchema.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_externalcontacts_relationship

> crate::models::Relationship put_externalcontacts_relationship(relationship_id, body)
Update a relationship

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**relationship_id** | **String** | Relationship Id | [required] |
**body** | [**Relationship**](Relationship.md) | Relationship | [required] |

### Return type

[**crate::models::Relationship**](Relationship.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

