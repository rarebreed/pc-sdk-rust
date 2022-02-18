# \ContentManagementApi

All URIs are relative to *https://api.mypurecloud.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_contentmanagement_document**](ContentManagementApi.md#delete_contentmanagement_document) | **DELETE** /api/v2/contentmanagement/documents/{documentId} | Delete a document.
[**delete_contentmanagement_share**](ContentManagementApi.md#delete_contentmanagement_share) | **DELETE** /api/v2/contentmanagement/shares/{shareId} | Deletes an existing share.
[**delete_contentmanagement_status_status_id**](ContentManagementApi.md#delete_contentmanagement_status_status_id) | **DELETE** /api/v2/contentmanagement/status/{statusId} | Cancel the command for this status
[**delete_contentmanagement_workspace**](ContentManagementApi.md#delete_contentmanagement_workspace) | **DELETE** /api/v2/contentmanagement/workspaces/{workspaceId} | Delete a workspace
[**delete_contentmanagement_workspace_member**](ContentManagementApi.md#delete_contentmanagement_workspace_member) | **DELETE** /api/v2/contentmanagement/workspaces/{workspaceId}/members/{memberId} | Delete a member from a workspace
[**delete_contentmanagement_workspace_tagvalue**](ContentManagementApi.md#delete_contentmanagement_workspace_tagvalue) | **DELETE** /api/v2/contentmanagement/workspaces/{workspaceId}/tagvalues/{tagId} | Delete workspace tag
[**get_contentmanagement_document**](ContentManagementApi.md#get_contentmanagement_document) | **GET** /api/v2/contentmanagement/documents/{documentId} | Get a document.
[**get_contentmanagement_document_audits**](ContentManagementApi.md#get_contentmanagement_document_audits) | **GET** /api/v2/contentmanagement/documents/{documentId}/audits | Get a list of audits for a document.
[**get_contentmanagement_document_content**](ContentManagementApi.md#get_contentmanagement_document_content) | **GET** /api/v2/contentmanagement/documents/{documentId}/content | Download a document.
[**get_contentmanagement_documents**](ContentManagementApi.md#get_contentmanagement_documents) | **GET** /api/v2/contentmanagement/documents | Get a list of documents.
[**get_contentmanagement_query**](ContentManagementApi.md#get_contentmanagement_query) | **GET** /api/v2/contentmanagement/query | Query content
[**get_contentmanagement_securityprofile**](ContentManagementApi.md#get_contentmanagement_securityprofile) | **GET** /api/v2/contentmanagement/securityprofiles/{securityProfileId} | Get a Security Profile
[**get_contentmanagement_securityprofiles**](ContentManagementApi.md#get_contentmanagement_securityprofiles) | **GET** /api/v2/contentmanagement/securityprofiles | Get a List of Security Profiles
[**get_contentmanagement_share**](ContentManagementApi.md#get_contentmanagement_share) | **GET** /api/v2/contentmanagement/shares/{shareId} | Retrieve details about an existing share.
[**get_contentmanagement_shared_shared_id**](ContentManagementApi.md#get_contentmanagement_shared_shared_id) | **GET** /api/v2/contentmanagement/shared/{sharedId} | Get shared documents. Securely download a shared document.
[**get_contentmanagement_shares**](ContentManagementApi.md#get_contentmanagement_shares) | **GET** /api/v2/contentmanagement/shares | Gets a list of shares.  You must specify at least one filter (e.g. entityId).
[**get_contentmanagement_status**](ContentManagementApi.md#get_contentmanagement_status) | **GET** /api/v2/contentmanagement/status | Get a list of statuses for pending operations
[**get_contentmanagement_status_status_id**](ContentManagementApi.md#get_contentmanagement_status_status_id) | **GET** /api/v2/contentmanagement/status/{statusId} | Get a status.
[**get_contentmanagement_usage**](ContentManagementApi.md#get_contentmanagement_usage) | **GET** /api/v2/contentmanagement/usage | Get usage details.
[**get_contentmanagement_workspace**](ContentManagementApi.md#get_contentmanagement_workspace) | **GET** /api/v2/contentmanagement/workspaces/{workspaceId} | Get a workspace.
[**get_contentmanagement_workspace_documents**](ContentManagementApi.md#get_contentmanagement_workspace_documents) | **GET** /api/v2/contentmanagement/workspaces/{workspaceId}/documents | Get a list of documents.
[**get_contentmanagement_workspace_member**](ContentManagementApi.md#get_contentmanagement_workspace_member) | **GET** /api/v2/contentmanagement/workspaces/{workspaceId}/members/{memberId} | Get a workspace member
[**get_contentmanagement_workspace_members**](ContentManagementApi.md#get_contentmanagement_workspace_members) | **GET** /api/v2/contentmanagement/workspaces/{workspaceId}/members | Get a list workspace members
[**get_contentmanagement_workspace_tagvalue**](ContentManagementApi.md#get_contentmanagement_workspace_tagvalue) | **GET** /api/v2/contentmanagement/workspaces/{workspaceId}/tagvalues/{tagId} | Get a workspace tag
[**get_contentmanagement_workspace_tagvalues**](ContentManagementApi.md#get_contentmanagement_workspace_tagvalues) | **GET** /api/v2/contentmanagement/workspaces/{workspaceId}/tagvalues | Get a list of workspace tags
[**get_contentmanagement_workspaces**](ContentManagementApi.md#get_contentmanagement_workspaces) | **GET** /api/v2/contentmanagement/workspaces | Get a list of workspaces.
[**post_contentmanagement_auditquery**](ContentManagementApi.md#post_contentmanagement_auditquery) | **POST** /api/v2/contentmanagement/auditquery | Query audits
[**post_contentmanagement_document**](ContentManagementApi.md#post_contentmanagement_document) | **POST** /api/v2/contentmanagement/documents/{documentId} | Update a document.
[**post_contentmanagement_document_content**](ContentManagementApi.md#post_contentmanagement_document_content) | **POST** /api/v2/contentmanagement/documents/{documentId}/content | Replace the contents of a document.
[**post_contentmanagement_documents**](ContentManagementApi.md#post_contentmanagement_documents) | **POST** /api/v2/contentmanagement/documents | Add a document.
[**post_contentmanagement_query**](ContentManagementApi.md#post_contentmanagement_query) | **POST** /api/v2/contentmanagement/query | Query content
[**post_contentmanagement_shares**](ContentManagementApi.md#post_contentmanagement_shares) | **POST** /api/v2/contentmanagement/shares | Creates a new share or updates an existing share if the entity has already been shared
[**post_contentmanagement_workspace_tagvalues**](ContentManagementApi.md#post_contentmanagement_workspace_tagvalues) | **POST** /api/v2/contentmanagement/workspaces/{workspaceId}/tagvalues | Create a workspace tag
[**post_contentmanagement_workspace_tagvalues_query**](ContentManagementApi.md#post_contentmanagement_workspace_tagvalues_query) | **POST** /api/v2/contentmanagement/workspaces/{workspaceId}/tagvalues/query | Perform a prefix query on tags in the workspace
[**post_contentmanagement_workspaces**](ContentManagementApi.md#post_contentmanagement_workspaces) | **POST** /api/v2/contentmanagement/workspaces | Create a group workspace
[**put_contentmanagement_workspace**](ContentManagementApi.md#put_contentmanagement_workspace) | **PUT** /api/v2/contentmanagement/workspaces/{workspaceId} | Update a workspace
[**put_contentmanagement_workspace_member**](ContentManagementApi.md#put_contentmanagement_workspace_member) | **PUT** /api/v2/contentmanagement/workspaces/{workspaceId}/members/{memberId} | Add a member to a workspace
[**put_contentmanagement_workspace_tagvalue**](ContentManagementApi.md#put_contentmanagement_workspace_tagvalue) | **PUT** /api/v2/contentmanagement/workspaces/{workspaceId}/tagvalues/{tagId} | Update a workspace tag. Will update all documents with the new tag value.



## delete_contentmanagement_document

> delete_contentmanagement_document(document_id, _override)
Delete a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**_override** | Option<**bool**> | Override any lock on the document |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contentmanagement_share

> delete_contentmanagement_share(share_id)
Deletes an existing share.

This revokes sharing rights specified in the share record

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**share_id** | **String** | Share ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contentmanagement_status_status_id

> delete_contentmanagement_status_status_id(status_id)
Cancel the command for this status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_id** | **String** | Status ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contentmanagement_workspace

> delete_contentmanagement_workspace(workspace_id, move_children_to_workspace_id)
Delete a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**move_children_to_workspace_id** | Option<**String**> | New location for objects in deleted workspace. |  |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contentmanagement_workspace_member

> delete_contentmanagement_workspace_member(workspace_id, member_id)
Delete a member from a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**member_id** | **String** | Member ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contentmanagement_workspace_tagvalue

> delete_contentmanagement_workspace_tagvalue(workspace_id, tag_id)
Delete workspace tag

Delete a tag from a workspace. Will remove this tag from all documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**tag_id** | **String** | Tag ID | [required] |

### Return type

 (empty response body)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_document

> crate::models::Document get_contentmanagement_document(document_id, expand)
Get a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_document_audits

> crate::models::DocumentAuditEntityListing get_contentmanagement_document_audits(document_id, page_size, page_number, transaction_filter, level, sort_by, sort_order)
Get a list of audits for a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**transaction_filter** | Option<**String**> | Transaction filter |  |
**level** | Option<**String**> | level |  |[default to USER]
**sort_by** | Option<**String**> | Sort by |  |
**sort_order** | Option<**String**> | Sort order |  |[default to ascending]

### Return type

[**crate::models::DocumentAuditEntityListing**](DocumentAuditEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_document_content

> crate::models::DownloadResponse get_contentmanagement_document_content(document_id, disposition, content_type)
Download a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**disposition** | Option<**String**> | Request how the content will be downloaded: a file attachment or inline. Default is attachment. |  |
**content_type** | Option<**String**> | The requested format for the specified document. If supported, the document will be returned in that format. Example contentType=audio/wav |  |

### Return type

[**crate::models::DownloadResponse**](DownloadResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_documents

> crate::models::DocumentEntityListing get_contentmanagement_documents(workspace_id, name, expand, page_size, page_number, sort_by, sort_order)
Get a list of documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**name** | Option<**String**> | Name |  |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | name or dateCreated |  |
**sort_order** | Option<**String**> | ascending or descending |  |[default to ascending]

### Return type

[**crate::models::DocumentEntityListing**](DocumentEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_query

> crate::models::QueryResults get_contentmanagement_query(query_phrase, page_size, page_number, sort_by, sort_order, expand)
Query content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query_phrase** | **String** | Phrase tokens are ANDed together over all searchable fields | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | name or dateCreated |  |[default to name]
**sort_order** | Option<**String**> | ascending or descending |  |[default to ascending]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::QueryResults**](QueryResults.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_securityprofile

> crate::models::SecurityProfile get_contentmanagement_securityprofile(security_profile_id)
Get a Security Profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**security_profile_id** | **String** | Security Profile Id | [required] |

### Return type

[**crate::models::SecurityProfile**](SecurityProfile.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_securityprofiles

> crate::models::SecurityProfileEntityListing get_contentmanagement_securityprofiles()
Get a List of Security Profiles

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SecurityProfileEntityListing**](SecurityProfileEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_share

> crate::models::Share get_contentmanagement_share(share_id, expand)
Retrieve details about an existing share.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**share_id** | **String** | Share ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::Share**](Share.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_shared_shared_id

> crate::models::SharedResponse get_contentmanagement_shared_shared_id(shared_id, redirect, disposition, content_type, expand)
Get shared documents. Securely download a shared document.

This method requires the download sharing URI obtained in the get document response (downloadSharingUri). Documents may be shared between users in the same workspace. Documents may also be shared between any user by creating a content management share.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**shared_id** | **String** | Shared ID | [required] |
**redirect** | Option<**bool**> | Turn on or off redirect |  |[default to true]
**disposition** | Option<**String**> | Request how the share content will be downloaded: attached as a file or inline. Default is attachment. |  |[default to attachment]
**content_type** | Option<**String**> | The requested format for the specified document. If supported, the document will be returned in that format. Example contentType=audio/wav |  |
**expand** | Option<**String**> | Expand some document fields |  |

### Return type

[**crate::models::SharedResponse**](SharedResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_shares

> crate::models::ShareEntityListing get_contentmanagement_shares(entity_id, expand, page_size, page_number)
Gets a list of shares.  You must specify at least one filter (e.g. entityId).

Failing to specify a filter will return 400.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity_id** | Option<**String**> | Filters the shares returned to only the entity specified by the value of this parameter. |  |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::ShareEntityListing**](ShareEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_status

> crate::models::CommandStatusEntityListing get_contentmanagement_status(page_size, page_number)
Get a list of statuses for pending operations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]

### Return type

[**crate::models::CommandStatusEntityListing**](CommandStatusEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_status_status_id

> crate::models::CommandStatus get_contentmanagement_status_status_id(status_id)
Get a status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status_id** | **String** | Status ID | [required] |

### Return type

[**crate::models::CommandStatus**](CommandStatus.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_usage

> crate::models::Usage get_contentmanagement_usage()
Get usage details.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Usage**](Usage.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_workspace

> crate::models::Workspace get_contentmanagement_workspace(workspace_id, expand)
Get a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::Workspace**](Workspace.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_workspace_documents

> crate::models::DocumentEntityListing get_contentmanagement_workspace_documents(workspace_id, expand, page_size, page_number, sort_by, sort_order)
Get a list of documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**sort_by** | Option<**String**> | name or dateCreated |  |
**sort_order** | Option<**String**> | ascending or descending |  |[default to ascending]

### Return type

[**crate::models::DocumentEntityListing**](DocumentEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_workspace_member

> crate::models::WorkspaceMember get_contentmanagement_workspace_member(workspace_id, member_id, expand)
Get a workspace member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**member_id** | **String** | Member ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::WorkspaceMember**](WorkspaceMember.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_workspace_members

> crate::models::WorkspaceMemberEntityListing get_contentmanagement_workspace_members(workspace_id, page_size, page_number, expand)
Get a list workspace members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::WorkspaceMemberEntityListing**](WorkspaceMemberEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_workspace_tagvalue

> crate::models::TagValue get_contentmanagement_workspace_tagvalue(workspace_id, tag_id, expand)
Get a workspace tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**tag_id** | **String** | Tag ID | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::TagValue**](TagValue.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_workspace_tagvalues

> crate::models::TagValueEntityListing get_contentmanagement_workspace_tagvalues(workspace_id, value, page_size, page_number, expand)
Get a list of workspace tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**value** | Option<**String**> | filter the list of tags returned |  |
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::TagValueEntityListing**](TagValueEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contentmanagement_workspaces

> crate::models::WorkspaceEntityListing get_contentmanagement_workspaces(page_size, page_number, access, expand)
Get a list of workspaces.

Specifying 'content' access will return all workspaces the user has document access to, while 'admin' access will return all group workspaces the user has administrative rights to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_size** | Option<**i32**> | Page size |  |[default to 25]
**page_number** | Option<**i32**> | Page number |  |[default to 1]
**access** | Option<[**Vec<String>**](String.md)> | Requested access level. |  |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::WorkspaceEntityListing**](WorkspaceEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_auditquery

> crate::models::QueryResults post_contentmanagement_auditquery(body)
Query audits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**ContentQueryRequest**](ContentQueryRequest.md) | Allows for a filtered query returning facet information | [required] |

### Return type

[**crate::models::QueryResults**](QueryResults.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_document

> crate::models::Document post_contentmanagement_document(document_id, body, expand, _override)
Update a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**body** | [**DocumentUpdate**](DocumentUpdate.md) | Document | [required] |
**expand** | Option<**String**> | Expand some document fields |  |
**_override** | Option<**bool**> | Override any lock on the document |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_document_content

> crate::models::ReplaceResponse post_contentmanagement_document_content(document_id, body, _override)
Replace the contents of a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | **String** | Document ID | [required] |
**body** | [**ReplaceRequest**](ReplaceRequest.md) | Replace Request | [required] |
**_override** | Option<**bool**> | Override any lock on the document |  |

### Return type

[**crate::models::ReplaceResponse**](ReplaceResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_documents

> crate::models::Document post_contentmanagement_documents(body, copy_source, move_source, _override)
Add a document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**DocumentUpload**](DocumentUpload.md) | Document | [required] |
**copy_source** | Option<**String**> | Copy a document within a workspace or to a new workspace. Provide a document ID as the copy source. |  |
**move_source** | Option<**String**> | Move a document to a new workspace. Provide a document ID as the move source. |  |
**_override** | Option<**bool**> | Override any lock on the source document |  |

### Return type

[**crate::models::Document**](Document.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_query

> crate::models::QueryResults post_contentmanagement_query(body, expand)
Query content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**QueryRequest**](QueryRequest.md) | Allows for a filtered query returning facet information | [required] |
**expand** | Option<**String**> | Expand some document fields |  |

### Return type

[**crate::models::QueryResults**](QueryResults.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_shares

> crate::models::CreateShareResponse post_contentmanagement_shares(body)
Creates a new share or updates an existing share if the entity has already been shared

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**CreateShareRequest**](CreateShareRequest.md) | CreateShareRequest - entity id and type and a single member or list of members are required | [required] |

### Return type

[**crate::models::CreateShareResponse**](CreateShareResponse.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_workspace_tagvalues

> crate::models::TagValue post_contentmanagement_workspace_tagvalues(workspace_id, body)
Create a workspace tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**body** | [**TagValue**](TagValue.md) | tag | [required] |

### Return type

[**crate::models::TagValue**](TagValue.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_workspace_tagvalues_query

> crate::models::TagValueEntityListing post_contentmanagement_workspace_tagvalues_query(workspace_id, body, expand)
Perform a prefix query on tags in the workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**body** | [**TagQueryRequest**](TagQueryRequest.md) | query | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Which fields, if any, to expand. |  |

### Return type

[**crate::models::TagValueEntityListing**](TagValueEntityListing.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_contentmanagement_workspaces

> crate::models::Workspace post_contentmanagement_workspaces(body)
Create a group workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**WorkspaceCreate**](WorkspaceCreate.md) | Workspace | [required] |

### Return type

[**crate::models::Workspace**](Workspace.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_contentmanagement_workspace

> crate::models::Workspace put_contentmanagement_workspace(workspace_id, body)
Update a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**body** | [**Workspace**](Workspace.md) | Workspace | [required] |

### Return type

[**crate::models::Workspace**](Workspace.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_contentmanagement_workspace_member

> crate::models::WorkspaceMember put_contentmanagement_workspace_member(workspace_id, member_id, body)
Add a member to a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**member_id** | **String** | Member ID | [required] |
**body** | [**WorkspaceMember**](WorkspaceMember.md) | Workspace Member | [required] |

### Return type

[**crate::models::WorkspaceMember**](WorkspaceMember.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_contentmanagement_workspace_tagvalue

> crate::models::TagValue put_contentmanagement_workspace_tagvalue(workspace_id, tag_id, body)
Update a workspace tag. Will update all documents with the new tag value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace ID | [required] |
**tag_id** | **String** | Tag ID | [required] |
**body** | [**TagValue**](TagValue.md) | Workspace | [required] |

### Return type

[**crate::models::TagValue**](TagValue.md)

### Authorization

[PureCloud OAuth](../README.md#PureCloud OAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

