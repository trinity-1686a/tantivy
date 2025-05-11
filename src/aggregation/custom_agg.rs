use serde::{Deserialize, Serialize};

use super::agg_req::Aggregation;
use super::agg_req_with_accessor::AggregationsWithAccessor;
use super::intermediate_agg_result::IntermediateAggregationResults;

pub trait CustomIntermediateRes: 'static + Clone + std::fmt::Debug + Sync + Send {
    fn into_final_metric_result<C>(self, req: &Aggregation<C>) -> crate::Result<C::FinalRes>
    where C: CustomAgg<IntermediateRes = Self>;
}

pub trait CustomRes: std::fmt::Debug + Send {
    fn get_bucket_count(&self) -> u64;
    fn get_value_from_aggregation(
        &self,
        name: &str,
        agg_property: &str,
    ) -> crate::Result<Option<f64>>;
}

pub trait CustomAgg:
    'static + Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug + Sync + Send
{
    type SegmentCollector: super::segment_agg_result::SegmentAggregationCollector<Self>;
    type IntermediateRes: CustomIntermediateRes;
    type FinalRes: CustomRes;

    fn field_names(&self) -> Vec<&str>;
    fn new_segment_collector(
        &self,
        sub_aggregatioan: &mut AggregationsWithAccessor<Self>,
        limits: &mut crate::aggregation::AggregationLimitsGuard,
        field_type: crate::aggregation::ColumnType,
        accessor_idx: usize,
    ) -> crate::Result<Self::SegmentCollector>;
    fn empty_intermediate_res(&self) -> Self::IntermediateRes;
}

/// Type when no custom aggregation is used
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NoCustom {}

impl CustomAgg for NoCustom {
    type SegmentCollector = NoCustom;
    type IntermediateRes = NoCustom;
    type FinalRes = NoCustom;

    fn field_names(&self) -> Vec<&str> {
        match *self {}
    }

    fn new_segment_collector(
        &self,
        _sub_aggregatioan: &mut AggregationsWithAccessor<Self>,
        _limits: &mut crate::aggregation::AggregationLimitsGuard,
        _field_type: crate::aggregation::ColumnType,
        _accessor_idx: usize,
    ) -> crate::Result<Self::SegmentCollector> {
        match *self {}
    }
    fn empty_intermediate_res(&self) -> Self::IntermediateRes {
        match *self {}
    }
}

impl super::segment_agg_result::SegmentAggregationCollector<NoCustom> for NoCustom {
    fn add_intermediate_aggregation_result(
        self: Box<Self>,
        agg_with_accessor: &AggregationsWithAccessor<NoCustom>,
        results: &mut IntermediateAggregationResults<NoCustom>,
    ) -> crate::Result<()> {
        match *self {}
    }

    fn collect(
        &mut self,
        doc: crate::DocId,
        agg_with_accessor: &mut AggregationsWithAccessor<NoCustom>,
    ) -> crate::Result<()> {
        match *self {}
    }

    fn collect_block(
        &mut self,
        docs: &[crate::DocId],
        agg_with_accessor: &mut AggregationsWithAccessor<NoCustom>,
    ) -> crate::Result<()> {
        match *self {}
    }
}

impl CustomIntermediateRes for NoCustom {
    fn into_final_metric_result<C>(self, req: &Aggregation<C>) -> crate::Result<C::FinalRes>
    where C: CustomAgg<IntermediateRes = Self> {
        match self {}
    }
}

impl CustomRes for NoCustom {
    fn get_bucket_count(&self) -> u64 {
        match *self {}
    }
    fn get_value_from_aggregation(
        &self,
        name: &str,
        agg_property: &str,
    ) -> crate::Result<Option<f64>> {
        match *self {}
    }
}
