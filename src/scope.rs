use flame::{self, Span};

pub struct NormalizationParams {
    pub start: u64,
    pub end: u64,
    pub normalization: f64,
}

// recursibly parse flame tree into region tree (TODO just parse into ScopeTree without RegionTree)
pub fn from_flame(
    spans: &Vec<Span>, 
    norm_params: &NormalizationParams
) -> Vec<RegionTree<f64>> {
    spans
        .iter()
        .map(|span| {
            let start = (span.start_ns - norm_params.start) as f64 / norm_params.normalization;
            let end = start + (span.end_ns - span.start_ns) as f64 / norm_params.normalization;
            RegionTree {
                start,
                end,
                regions: from_flame(&span.children, norm_params),
                desc: span.name.to_string()
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct RegionTree<T> {
    pub start: T,
    pub end: T,
    pub regions: Vec<RegionTree<T>>,
    pub desc: String,
}

impl RegionTree<f64> {
    pub fn from_flame(spans: &Vec<Span>) -> Vec<RegionTree<f64>> {
        // normalization params
        let mut start = std::u64::MAX;
        let mut end = 0u64;
        for span in spans.iter() {
            start = start.min(span.start_ns);
            end = end.max(span.end_ns);
        }
        let normalization = (end - start) as f64;
        // construction region tree
        let norm_params = NormalizationParams {
            start,
            end,
            normalization,
        };
        from_flame(spans, &norm_params)
    }
}
