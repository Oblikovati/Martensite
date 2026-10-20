//! The cases of `lowalgos_intss` that need no viewer.
use super::*;

script_cases!(run_case, NoView, Group::Lowalgos, lowalgos_intss;
    bug21750, bug27623, bug106, bug139, bug20524, bug22766, bug23163_2, bug23163_3,
    bug23166, bug237, bug23732, bug25898, bug431, bug504, bug324, bug513, bug531, bug537,
    bug6474, bug23981, bug25952_2, bug25952_3, bug26894, bug28222_1, bug24772,
    bug27079_1, bug27079_2, bug271, bug27950, bug28012, bug286, bug429);

// The original script of this case is defective: it outlasts its 60 s budget in every
// implementation measured, so it only runs on request.
script_cases!(ignore = "the original script outlasts its 60 s budget; run with --ignored",
    run_case, NoView, Group::Lowalgos, lowalgos_intss; bug565);
