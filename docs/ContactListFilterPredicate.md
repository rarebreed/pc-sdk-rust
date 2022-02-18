# ContactListFilterPredicate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**column** | Option<**String**> | Contact list column from the ContactListFilter's contactList. | [optional]
**column_type** | Option<**String**> | The type of data in the contact column. | [optional]
**operator** | Option<**String**> | The operator for this ContactListFilterPredicate. | [optional]
**value** | Option<**String**> | Value with which to compare the contact's data. This could be text, a number, or a relative time. A value for relative time should follow the format PxxDTyyHzzM, where xx, yy, and zz specify the days, hours and minutes. For example, a value of P01DT08H30M corresponds to 1 day, 8 hours, and 30 minutes from now. To specify a time in the past, include a negative sign before each numeric value. For example, a value of P-01DT-08H-30M corresponds to 1 day, 8 hours, and 30 minutes in the past. You can also do things like P01DT00H-30M, which would correspond to 23 hours and 30 minutes from now (1 day - 30 minutes). | [optional]
**range** | Option<[**crate::models::ContactListFilterRange**](ContactListFilterRange.md)> |  | [optional]
**inverted** | Option<**bool**> | Inverts the result of the predicate (i.e., if the predicate returns true, inverting it will return false). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


