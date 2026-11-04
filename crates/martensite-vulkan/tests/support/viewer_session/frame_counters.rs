//! Frame statistics of the active view, read after an immediate redraw.
use super::*;
use martensite_tkservice::graphic3d::{
    Graphic3dFrameStatsDict, Graphic3dRenderingParamsPerfCounters as Perf,
};

impl ViewerSession {
    /// Redraw with the structure counters collected and an immediate
    /// statistics update, then the "Rendered structs" value ("Structs" when every structure
    /// was rendered); the previous counters and update interval are restored.
    /// Example: `session.rendered_structures()? == "2000"`.
    pub fn rendered_structures(&self) -> Result<String, Failure> {
        let dict = query_dictionary(self.view().view_base(), Perf::STRUCTURES);
        let rendered = dict_value(&dict, "Rendered structs");
        Ok(rendered
            .or_else(|| dict_value(&dict, "Structs"))
            .unwrap_or_default())
    }

    /// Query actual non-culled points with the command's temporary counters; `session.rendered_points()?`.
    pub fn rendered_points(&self) -> Result<String, Failure> {
        Ok(query_text(
            self.view().view_base(),
            Perf::POINTS,
            "Rendered points",
        ))
    }

    /// Query actual non-culled triangles with restored settings; `session.rendered_triangles()?`.
    pub fn rendered_triangles(&self) -> Result<String, Failure> {
        Ok(query_text(
            self.view().view_base(),
            Perf::TRIANGLES,
            "Rendered triangles",
        ))
    }
}

fn query_text(view: &V3dView, request: Perf, key: &str) -> String {
    let dict = query_dictionary(view, request);
    dict_value(&dict, key).unwrap_or_default()
}

fn query_dictionary(view: &V3dView, request: Perf) -> Graphic3dFrameStatsDict {
    let stats = view.rendering_params().stats;
    view.change_rendering_params(&mut |params| {
        params.stats.collected_stats = Perf(stats.collected_stats.0 | request.0);
        params.stats.stats_update_interval = -1.0;
    });
    view.redraw();
    view.change_rendering_params(&mut |params| {
        params.stats.stats_update_interval = stats.stats_update_interval
    });
    let mut dict = Graphic3dFrameStatsDict::default();
    view.statistic_information_dict(&mut dict);
    view.change_rendering_params(&mut |params| {
        params.stats.collected_stats = stats.collected_stats
    });
    dict
}

/// The value of `key` in the statistics dictionary, compared without case.
fn dict_value(dict: &Graphic3dFrameStatsDict, key: &str) -> Option<String> {
    let text = |s: &martensite_tkernel::tcollection::AsciiString| {
        String::from_utf8_lossy(s.as_bytes()).into_owned()
    };
    dict.items()
        .find(|(name, _)| text(name).eq_ignore_ascii_case(key))
        .map(|(_, value)| text(value))
}
