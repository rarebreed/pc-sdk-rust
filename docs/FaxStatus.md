# FaxStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**direction** | Option<**String**> | The fax direction, either \"send\" or \"receive\". | [optional]
**expected_pages** | Option<**i64**> | Total number of expected pages, if known. | [optional]
**active_page** | Option<**i64**> | Active page of the transmission. | [optional]
**lines_transmitted** | Option<**i64**> | Number of lines that have completed transmission. | [optional]
**bytes_transmitted** | Option<**i64**> | Number of bytes that have competed transmission. | [optional]
**baud_rate** | Option<**i64**> | Current signaling rate of transmission, baud rate. | [optional]
**page_errors** | Option<**i64**> | Number of page errors. | [optional]
**line_errors** | Option<**i64**> | Number of line errors. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


