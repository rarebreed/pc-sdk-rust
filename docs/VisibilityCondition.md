# VisibilityCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**combining_operation** | Option<**String**> |  | [optional]
**predicates** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | A list of strings, each representing the location in the form of the Answer Option to depend on. In the format of \"/form/questionGroup/{questionGroupIndex}/question/{questionIndex}/answer/{answerIndex}\" or, to assume the current question group, \"../question/{questionIndex}/answer/{answerIndex}\". Note: Indexes are zero-based | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


