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
pub struct BuForecastTimeSeriesResult {
    /// The metric this result applies to
    #[serde(rename = "metric", skip_serializing_if = "Option::is_none")]
    pub metric: Option<Metric>,
    /// The forecasting method that was used for this metric
    #[serde(rename = "forecastingMethod", skip_serializing_if = "Option::is_none")]
    pub forecasting_method: Option<ForecastingMethod>,
    /// The forecasting type in this forecast result
    #[serde(rename = "forecastType", skip_serializing_if = "Option::is_none")]
    pub forecast_type: Option<ForecastType>,
}

impl BuForecastTimeSeriesResult {
    pub fn new() -> BuForecastTimeSeriesResult {
        BuForecastTimeSeriesResult {
            metric: None,
            forecasting_method: None,
            forecast_type: None,
        }
    }
}

/// The metric this result applies to
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Metric {
    #[serde(rename = "Offered")]
    Offered,
    #[serde(rename = "AverageHandleTimeSeconds")]
    AverageHandleTimeSeconds,
}

impl Default for Metric {
    fn default() -> Metric {
        Self::Offered
    }
}
/// The forecasting method that was used for this metric
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ForecastingMethod {
    #[serde(rename = "AutoRegressiveIntegratedMovingAverage")]
    AutoRegressiveIntegratedMovingAverage,
    #[serde(rename = "MovingAverage")]
    MovingAverage,
    #[serde(rename = "SingleExponentialSmoothing")]
    SingleExponentialSmoothing,
    #[serde(rename = "RandomWalk")]
    RandomWalk,
    #[serde(rename = "DecompositionUsingAdditiveSeasonality")]
    DecompositionUsingAdditiveSeasonality,
    #[serde(rename = "DecompositionUsingMultiplicativeSeasonality")]
    DecompositionUsingMultiplicativeSeasonality,
    #[serde(rename = "HoltWintersAdditiveSeasonality")]
    HoltWintersAdditiveSeasonality,
    #[serde(rename = "HoltWintersAdditiveSeasonalityWithDampedTrend")]
    HoltWintersAdditiveSeasonalityWithDampedTrend,
    #[serde(rename = "HoltWintersMultiplicativeSeasonality")]
    HoltWintersMultiplicativeSeasonality,
    #[serde(rename = "HoltWintersMultiplicativeSeasonalityWithDampedTrend")]
    HoltWintersMultiplicativeSeasonalityWithDampedTrend,
    #[serde(rename = "DampedLinearExponentialSmoothing")]
    DampedLinearExponentialSmoothing,
    #[serde(rename = "DoubleExponentialSmoothing")]
    DoubleExponentialSmoothing,
    #[serde(rename = "DoubleMovingAverage")]
    DoubleMovingAverage,
    #[serde(rename = "LinearExponentialSmoothing")]
    LinearExponentialSmoothing,
    #[serde(rename = "LinearWeightedMovingAverage")]
    LinearWeightedMovingAverage,
    #[serde(rename = "PointEstimateUsingDampedLinearExponentialSmoothing")]
    PointEstimateUsingDampedLinearExponentialSmoothing,
    #[serde(rename = "PointEstimateUsingDoubleExponentialSmoothing")]
    PointEstimateUsingDoubleExponentialSmoothing,
    #[serde(rename = "PointEstimateUsingLatestWeek")]
    PointEstimateUsingLatestWeek,
    #[serde(rename = "PointEstimateUsingLinearExponentialSmoothing")]
    PointEstimateUsingLinearExponentialSmoothing,
    #[serde(rename = "PointEstimateUsingWeightedAverage")]
    PointEstimateUsingWeightedAverage,
    #[serde(rename = "CurveFit")]
    CurveFit,
    #[serde(rename = "MultiLinearRegression")]
    MultiLinearRegression,
    #[serde(rename = "DynamicHarmonicRegression")]
    DynamicHarmonicRegression,
    #[serde(rename = "Theta")]
    Theta,
    #[serde(rename = "Ensemble")]
    Ensemble,
    #[serde(rename = "Other")]
    Other,
}

impl Default for ForecastingMethod {
    fn default() -> ForecastingMethod {
        Self::AutoRegressiveIntegratedMovingAverage
    }
}
/// The forecasting type in this forecast result
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ForecastType {
    #[serde(rename = "LongTerm")]
    LongTerm,
    #[serde(rename = "ShortTerm")]
    ShortTerm,
}

impl Default for ForecastType {
    fn default() -> ForecastType {
        Self::LongTerm
    }
}
