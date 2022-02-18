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
pub struct AnalyticsMediaEndpointStat {
    /// The MIME type(s) of the audio encodings used by the audio streams belonging to this endpoint
    #[serde(rename = "codecs", skip_serializing_if = "Option::is_none")]
    pub codecs: Option<Vec<String>>,
    /// The total number of packets received too late or too early, jitter queue overrun or underrun, for all audio streams belonging to this endpoint
    #[serde(rename = "discardedPackets", skip_serializing_if = "Option::is_none")]
    pub discarded_packets: Option<i64>,
    /// The total number of packets received with the same sequence number as another one recently received (window of 64 packets), for all audio streams belonging to this endpoint
    #[serde(rename = "duplicatePackets", skip_serializing_if = "Option::is_none")]
    pub duplicate_packets: Option<i64>,
    /// Specifies when an event occurred. Date time is represented as an ISO-8601 string. For example: yyyy-MM-ddTHH:mm:ss[.mmm]Z
    #[serde(rename = "eventTime", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// The total number of malformed or not RTP packets, unknown payload type, or discarded probation packets for all audio streams belonging to this endpoint
    #[serde(rename = "invalidPackets", skip_serializing_if = "Option::is_none")]
    pub invalid_packets: Option<i64>,
    /// The maximum latency experienced by any audio stream belonging to this endpoint, in milliseconds
    #[serde(rename = "maxLatencyMs", skip_serializing_if = "Option::is_none")]
    pub max_latency_ms: Option<i64>,
    /// The lowest estimated average MOS among all the audio streams belonging to this endpoint
    #[serde(rename = "minMos", skip_serializing_if = "Option::is_none")]
    pub min_mos: Option<f64>,
    /// The lowest R-factor value among all of the audio streams belonging to this endpoint
    #[serde(rename = "minRFactor", skip_serializing_if = "Option::is_none")]
    pub min_r_factor: Option<f64>,
    /// The total number of packets for which there was no room in the jitter queue when it was received, for all audio streams belonging to this endpoint (also counted in discarded)
    #[serde(rename = "overrunPackets", skip_serializing_if = "Option::is_none")]
    pub overrun_packets: Option<i64>,
    /// The total number of packets received for all audio streams belonging to this endpoint (includes invalid, duplicate, and discarded packets)
    #[serde(rename = "receivedPackets", skip_serializing_if = "Option::is_none")]
    pub received_packets: Option<i64>,
    /// The total number of packets received after their timestamp/seqnum has been played out, for all audio streams belonging to this endpoint (also counted in discarded)
    #[serde(rename = "underrunPackets", skip_serializing_if = "Option::is_none")]
    pub underrun_packets: Option<i64>,
}

impl AnalyticsMediaEndpointStat {
    pub fn new() -> AnalyticsMediaEndpointStat {
        AnalyticsMediaEndpointStat {
            codecs: None,
            discarded_packets: None,
            duplicate_packets: None,
            event_time: None,
            invalid_packets: None,
            max_latency_ms: None,
            min_mos: None,
            min_r_factor: None,
            overrun_packets: None,
            received_packets: None,
            underrun_packets: None,
        }
    }
}


