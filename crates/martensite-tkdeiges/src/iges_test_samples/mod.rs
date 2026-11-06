//! Sample entities for the class tests, and the round they go through.
mod harness;
mod samples_01;
mod samples_02;
mod samples_03;
mod samples_04;
pub(crate) use harness::{
    assert_entity_round, assert_entity_round_after, h_array1_of_h_array1_of_integer_sample,
    h_array1_of_h_array1_of_real_sample, h_array1_of_h_array1_of_xyz_sample, iges_round_read,
    iges_written_file, sample_list,
};
pub(crate) use samples_01::{
    sample_iges_appli_drilled_hole, sample_iges_appli_element_results,
    sample_iges_appli_finite_element, sample_iges_appli_flow, sample_iges_appli_flow_line_spec,
    sample_iges_appli_level_function, sample_iges_appli_level_to_pwb_layer_map,
    sample_iges_appli_line_widening, sample_iges_appli_nodal_constraint,
    sample_iges_appli_nodal_displ_and_rot, sample_iges_appli_nodal_results, sample_iges_appli_node,
    sample_iges_appli_part_number, sample_iges_appli_pin_number, sample_iges_appli_piping_flow,
    sample_iges_appli_pwb_artwork_stackup, sample_iges_appli_pwb_drilled_hole,
    sample_iges_appli_reference_designator, sample_iges_appli_region_restriction,
    sample_iges_basic_subfigure_def, sample_iges_geom_plane,
    sample_iges_geom_transformation_matrix,
};
pub(crate) use samples_02::{
    sample_iges_defs_associativity_def, sample_iges_defs_attribute_def,
    sample_iges_defs_attribute_table, sample_iges_defs_generic_data, sample_iges_defs_macro_def,
    sample_iges_defs_tabular_data, sample_iges_defs_units_data,
    sample_iges_dimen_angular_dimension, sample_iges_dimen_basic_dimension,
    sample_iges_dimen_center_line, sample_iges_dimen_curve_dimension,
    sample_iges_dimen_diameter_dimension, sample_iges_dimen_dimension_display_data,
    sample_iges_dimen_dimension_tolerance, sample_iges_dimen_dimension_units,
    sample_iges_dimen_dimensioned_geometry, sample_iges_dimen_flag_note,
    sample_iges_dimen_general_label, sample_iges_dimen_general_note,
    sample_iges_dimen_general_symbol, sample_iges_dimen_leader_arrow,
    sample_iges_dimen_linear_dimension,
};
pub(crate) use samples_03::{
    sample_iges_dimen_new_dimensioned_geometry, sample_iges_dimen_new_general_note,
    sample_iges_dimen_ordinate_dimension, sample_iges_dimen_point_dimension,
    sample_iges_dimen_radius_dimension, sample_iges_dimen_section,
    sample_iges_dimen_sectioned_area, sample_iges_dimen_witness_line,
    sample_iges_draw_circ_array_subfigure, sample_iges_draw_connect_point,
    sample_iges_draw_drawing, sample_iges_draw_drawing_with_rotation,
    sample_iges_draw_label_display, sample_iges_draw_network_subfigure,
    sample_iges_draw_network_subfigure_def, sample_iges_draw_perspective_view,
    sample_iges_draw_planar, sample_iges_draw_rect_array_subfigure,
    sample_iges_draw_segmented_views_visible, sample_iges_draw_view,
    sample_iges_draw_views_visible, sample_iges_draw_views_visible_with_attr,
};
pub(crate) use samples_04::{
    sample_iges_graph_color, sample_iges_graph_definition_level, sample_iges_graph_drawing_size,
    sample_iges_graph_drawing_units, sample_iges_graph_high_light,
    sample_iges_graph_intercharacter_spacing, sample_iges_graph_line_font_def_pattern,
    sample_iges_graph_line_font_def_template, sample_iges_graph_line_font_predefined,
    sample_iges_graph_nominal_size, sample_iges_graph_pick,
    sample_iges_graph_text_display_template, sample_iges_graph_text_font_def,
    sample_iges_graph_uniform_rect_grid,
};
