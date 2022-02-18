# FaxSendRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The globally unique identifier for the object. | [optional][readonly]
**name** | Option<**String**> |  | [optional]
**addresses** | **Vec<String>** | A list of outbound fax dialing addresses. E.g. +13175555555 or 3175555555 | 
**document_id** | Option<**String**> | DocumentId of Content Management artifact. If Content Management document is not used for faxing, documentId should be null | [optional]
**content_type** | Option<**String**> | The content type that is going to be uploaded. If Content Management document is used for faxing, contentType will be ignored | [optional]
**workspace** | Option<[**crate::models::Workspace**](Workspace.md)> |  | [optional]
**cover_sheet** | Option<[**crate::models::CoverSheet**](CoverSheet.md)> |  | [optional]
**time_zone_offset_minutes** | Option<**i32**> | Time zone offset minutes from GMT | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


