/*
 * PureCloud Platform API
 *
 * With the PureCloud Platform API, you can control all aspects of your PureCloud environment. With the APIs you can access the system configuration, manage conversations and more.
 *
 * The version of the OpenAPI document: v2
 * Contact: DeveloperEvangelists@genesys.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SelectedColumns {
    /// Indicates the order/position of the selected column
    #[serde(rename = "columnOrder", skip_serializing_if = "Option::is_none")]
    pub column_order: Option<i32>,
    /// Indicates enum name of the column from the export view
    #[serde(rename = "columnName", skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
}

impl SelectedColumns {
    pub fn new() -> SelectedColumns {
        SelectedColumns {
            column_order: None,
            column_name: None,
        }
    }
}


