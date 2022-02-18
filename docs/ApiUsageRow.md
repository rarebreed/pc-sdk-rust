# ApiUsageRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<**String**> | Client Id associated with this query result | [optional]
**client_name** | Option<**String**> | Client Name associated with this query result | [optional]
**organization_id** | Option<**String**> | Organization Id associated with this query result | [optional]
**user_id** | Option<**String**> | User Id associated with this query result | [optional]
**template_uri** | Option<**String**> | Template Uri associated with this query result | [optional]
**http_method** | Option<**String**> | HTTP Method associated with this query result | [optional]
**status200** | Option<**i64**> | Number of requests resulting in a 2xx HTTP status code | [optional]
**status300** | Option<**i64**> | Number of requests resulting in a 3xx HTTP status code | [optional]
**status400** | Option<**i64**> | Number of requests resulting in a 4xx HTTP status code | [optional]
**status500** | Option<**i64**> | Number of requests resulting in a 5xx HTTP status code | [optional]
**status429** | Option<**i64**> | Number of requests resulting in a 429 HTTP status code, this is a subset of the count returned with status400 | [optional]
**requests** | Option<**i64**> | Total number of requests | [optional]
**date** | Option<**String**> | Date of requests, based on granularity. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


