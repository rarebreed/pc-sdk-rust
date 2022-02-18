# EdgeLogsJobFile

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
**time_created** | Option<**String**> | Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**time_modified** | Option<**String**> | The time this log file was last modified on the Edge. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]
**size_bytes** | Option<**f64**> | The size of this file in bytes. | [optional]
**upload_status** | Option<**String**> | The status of the upload of this file from the Edge to the cloud.  Use /upload to start an upload. | [optional]
**edge_path** | Option<**String**> | The path of this file on the Edge. | [optional]
**download_id** | Option<**String**> | The download ID to use with the downloads API. | [optional]
**self_uri** | Option<**String**> | The URI for this object | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


