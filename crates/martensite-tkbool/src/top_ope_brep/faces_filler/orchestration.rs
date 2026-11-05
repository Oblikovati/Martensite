//! Face-pair insertion preserves shared published line slots throughout mutation.
use super::*;
impl<'a> TopOpeBRepFacesFiller<'a> {
    /// Inserts a classified face intersection into shared storage; `filler.insert(&f1,&f2,&mut intersection,&hds)?`.
    pub fn insert(
        &mut self,
        first: &Shape,
        second: &Shape,
        pair: &'a mut TopOpeBRepFacesIntersector,
        hds: &Handle<TopOpeBRepDSHDataStructure>,
    ) -> Result<(), Failure> {
        self.bind_context(first, second, pair, hds)?;
        self.reset_degenerate_context();
        self.kpart_context = Default::default();
        if self.change_faces_intersector()?.same_domain()? {
            self.hds
                .get()
                .ok_or_else(missing_storage)?
                .change_ds()
                .fill_shapes_same_domain(first, second, true)?;
            return Ok(());
        }
        self.initialize_operand_lines()?;
        self.vp_position_intersections()?;
        self.initialize_classified_lines()?;
        self.process_section_edges()?;
        self.first_pair_point = self.hds.get().ok_or_else(missing_storage)?.nb_points() + 1;
        self.collect_restriction_context()?;
        self.process_operand_lines()
    }
    fn initialize_operand_lines(&mut self) -> Result<(), Failure> {
        self.change_faces_intersector()?.init_line()?;
        while self.change_faces_intersector()?.more_line() {
            let faces = self.faces.clone();
            self.change_faces_intersector()?
                .current_line()?
                .set_faces(&faces[0], &faces[1]);
            self.change_faces_intersector()?.next_line()?;
        }
        Ok(())
    }
    fn initialize_classified_lines(&mut self) -> Result<(), Failure> {
        self.change_faces_intersector()?.init_line()?;
        while self.change_faces_intersector()?.more_line() {
            {
                let mut line = self.change_faces_intersector()?.current_line()?;
                line.set_has_vp_on_r()?;
                line.set_inl()?;
                line.set_is_v_closed()?;
            }
            self.change_faces_intersector()?.next_line()?;
        }
        Ok(())
    }
    fn collect_restriction_context(&mut self) -> Result<(), Failure> {
        self.restriction_edges.clear();
        let pair = self
            .intersector
            .as_deref()
            .ok_or_else(classification::missing_intersector)?;
        for edge in pair.restrictions().iter() {
            self.restriction_edges.append(edge.clone());
        }
        Ok(())
    }
    fn process_operand_lines(&mut self) -> Result<(), Failure> {
        self.change_faces_intersector()?.init_line()?;
        while self.change_faces_intersector()?.more_line() {
            let pair = self
                .intersector
                .as_deref()
                .ok_or_else(classification::missing_intersector)?;
            let index = pair.current_line_index();
            let lines = pair.lines();
            {
                let mut array = lines
                    .get()
                    .ok_or_else(classification::missing_intersector)?
                    .change_array1();
                let line = array.change_value(index).map_err(|error| {
                    Failure::with_message("Standard_OutOfRange", &error.to_string())
                })?;
                self.bind_array_line(lines.clone(), index);
                self.load_line(line)?;
                self.process_line(line)?;
            }
            self.change_faces_intersector()?.next_line()?;
        }
        Ok(())
    }
    /// Fills a loaded line and publishes its curve incidences; `filler.process_line(&mut line)?`.
    pub fn process_line(&mut self, line: &mut TopOpeBRepLineInter) -> Result<(), Failure> {
        if !self.line_ok {
            return Ok(());
        }
        self.reset_dsc();
        if line.has_vp_on_r() {
            self.fill_line_vpon_r(line)?;
        } else {
            self.fill_line(line)?;
        }
        if line.inl() {
            return Ok(());
        }
        self.hds
            .get()
            .ok_or_else(missing_storage)?
            .sort_on_parameter_in_place(&mut self.curve_interferences)?;
        self.add_shapes_line(line)
    }
}
