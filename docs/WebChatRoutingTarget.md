# WebChatRoutingTarget

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_type** | **String** | The target type of the routing target, such as 'QUEUE'. | 
**target_address** | **String** | The target of the route, in the format appropriate given the 'targetType'. | 
**skills** | Option<**Vec<String>**> | The list of skill names to use for routing. | [optional]
**language** | Option<**String**> | The language name to use for routing. | [optional]
**priority** | Option<**i64**> | The priority to assign to the conversation for routing. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


