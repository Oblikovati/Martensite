//! The `bugs_modalg_1` cases.
use crate::support::{
    groups, hooks,
    process::{NativeImprovement, dataset_cases, native_improvement_case},
};

dataset_cases!(groups::BUGS, bugs_modalg_1, hooks::no_grid_end;
    buc60409_1, buc60409_2, buc60409_3, buc60462_1, buc60462_2, buc60463, buc60531_1, buc60531_2, buc60532, buc60532_1, buc60532_2, buc60533, buc60555_1, buc60555_2, buc60555_3, buc60585_1, buc60585_2, buc60623_1, buc60623_2, buc60663_1, buc60663_2, buc60668, buc60669, buc60682, buc60690, buc60703_1, buc60703_2, buc60703_3, buc60703_4, buc60704, buc60708, buc60712, buc60728, buc60744, buc60776_1, buc60776_2, buc60787_1, buc60787_2, buc60788_1, buc60788_2, buc60788_3, buc60789_1, buc60789_2, buc60801, buc60803, buc60865, buc60880, buc60899_1, buc60899_2, buc60901, buc60926, bug101, bug102_1, bug102_2, bug10605_1, bug10605_2, bug10605_3, bug10605_4, bug10605_5, bug10606_1, bug10606_2, bug10606_3, bug10606_4, bug10606_5, bug12507, bug12918, bug13116_1, bug13116_2, bug13116_3, bug13116_4, bug13186_1, bug13186_2, bug13186_3, bug13186_4, bug13211_2, bug13211_3, bug13211_4, bug13211_5, bug13211_6, bug13538, bug144, bug14536, bug14777, bug15850, bug15850_1, bug15850_2, bug15850_3, bug15850_4, bug17194_1, bug17194_2);

native_improvement_case!(
    groups::BUGS,
    bugs_modalg_1,
    hooks::no_grid_end,
    bug19071,
    NativeImprovement {
        todo: "Error : The command is not valid. The area is 0.",
        reference_log: "the recorded reference run of bugs_modalg_1/bug19071"
    }
);
