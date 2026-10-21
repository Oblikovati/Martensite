//! The TKShHealing (no view) tests of tests/bugs/heal.
use super::*;

dataset_cases!(run_case, NoView, bugs_heal, BUGS_SECONDS;
    bug210_2, bug21317, bug22, bug23944, bug25967, bug28523, bug28768, bug22602, bug22642, bug22919, bug24, bug24008_1, bug24008_2,
    bug24218, bug24370, bug24658, bug24934, bug25553_1, bug25553_2, bug25553_3, bug26708,
    bug26930_1, bug26930_2, bug27246, bug28529, bug29504_1, bug29504_2, bug29544_1,
    bug30158_1, bug30158_2, bug30174, bug31202, bug32332_1, bug32332_2, bug329, bug482,
    bug7570);
