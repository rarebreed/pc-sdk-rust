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
pub struct AsyncUserDetailsQuery {
    /// Specifies the date and time range of data being queried. Conversations MUST have started within this time range to potentially be included within the result set. Intervals are represented as an ISO-8601 string. For example: YYYY-MM-DDThh:mm:ss/YYYY-MM-DDThh:mm:ss
    #[serde(rename = "interval")]
    pub interval: String,
    /// Filters that target the users to retrieve data for
    #[serde(rename = "userFilters", skip_serializing_if = "Option::is_none")]
    pub user_filters: Option<Vec<crate::models::UserDetailQueryFilter>>,
    /// Filters that target system and organization presence-level data
    #[serde(rename = "presenceFilters", skip_serializing_if = "Option::is_none")]
    pub presence_filters: Option<Vec<crate::models::PresenceDetailQueryFilter>>,
    /// Filters that target agent routing status-level data
    #[serde(rename = "routingStatusFilters", skip_serializing_if = "Option::is_none")]
    pub routing_status_filters: Option<Vec<crate::models::RoutingStatusDetailQueryFilter>>,
    /// Sort the result set in ascending/descending order. Default is ascending
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Specify number of results to be returned
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl AsyncUserDetailsQuery {
    pub fn new(interval: String) -> AsyncUserDetailsQuery {
        AsyncUserDetailsQuery {
            interval,
            user_filters: None,
            presence_filters: None,
            routing_status_filters: None,
            order: None,
            limit: None,
        }
    }
}

/// Sort the result set in ascending/descending order. Default is ascending
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Order {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for Order {
    fn default() -> Order {
        Self::Asc
    }
}

