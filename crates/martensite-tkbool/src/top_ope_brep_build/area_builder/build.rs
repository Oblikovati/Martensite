//! Area classification retains stable list nodes while grouping loops.
use super::*;
impl TopOpeBRepBuildAreaBuilder {
    /// The first non-empty area containing the loop (compared with the loops of kind
    /// `what`).
    fn area_containing(
        &self,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        l: &Handle<dyn TopOpeBRepBuildLoop>,
        what: TopOpeBRepBuildLoopEnum,
    ) -> Result<Option<usize>, Failure> {
        for (k, a_area) in self.my_area.iter().enumerate() {
            if a_area.is_empty() {
                continue;
            }
            let state = self.known(self.compare_items(lc, l, a_area.iter(), what)?)?;
            if state == State::In {
                return Ok(Some(k));
            }
        }
        Ok(None)
    }

    /// A block loop inside the area `k`: the loops of the area outside it moved out (to
    /// the boundary loops when they are all shapes, else to a new area), then the loop
    /// added to the area.
    fn insert_block_in_area(
        &mut self,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        l: &Handle<dyn TopOpeBRepBuildLoop>,
        k: usize,
        boundaryloops: &mut LoopList,
    ) -> Result<(), Failure> {
        let (all_shape, removed_loops) = self.remove_outside(lc, l, k)?;
        self.my_area.iter_mut().nth(k).unwrap().append(l.clone());
        if removed_loops.is_empty() {
            return Ok(());
        }
        if all_shape {
            boundaryloops.extend(removed_loops.iter().cloned());
        } else {
            self.my_area.append(removed_loops);
        }
        Ok(())
    }

    fn remove_outside(
        &mut self,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        l: &Handle<dyn TopOpeBRepBuildLoop>,
        k: usize,
    ) -> Result<(bool, AreaLoopList), Failure> {
        let (mut all_shape, mut removed) = (true, AreaLoopList::default());
        let mut cursor = self.my_area.iter().nth(k).unwrap().cursor();
        while let Some(current) = cursor.value(self.my_area.iter().nth(k).unwrap()).cloned() {
            if self.known(lc.compare(&current, l)?)? != State::Out {
                cursor.next(self.my_area.iter().nth(k).unwrap());
                continue;
            }
            all_shape = all_shape && current.get().ok_or_else(null_loop)?.is_shape();
            removed.append(current);
            self.my_area
                .iter_mut()
                .nth(k)
                .unwrap()
                .remove(&mut cursor)?;
        }
        Ok((all_shape, removed))
    }

    /// A block loop outside every area: a new area with the boundary loops that are
    /// inside it and contain it.
    fn new_block_area(
        &mut self,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        l: &Handle<dyn TopOpeBRepBuildLoop>,
        boundaryloops: &mut LoopList,
    ) -> Result<(), Failure> {
        let mut new_area0: LoopList = vec![l.clone()];
        let mut i = 0;
        while i < boundaryloops.len() {
            let lb = boundaryloops[i].clone();
            let ashapeinside = self.known(lc.compare(&lb, l)?)? == State::In;
            let ablockinside = ashapeinside && self.known(lc.compare(l, &lb)?)? == State::In;
            if ashapeinside && ablockinside {
                new_area0.push(lb);
                boundaryloops.remove(i);
            } else {
                i += 1;
            }
        }
        self.my_area.append(new_area0.into_iter().collect());
        Ok(())
    }

    /// The areas of the loops of the set, for the base or the 2d flavor.
    pub(super) fn build_areas(
        &mut self,
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force_class: bool,
        flavor: Flavor,
    ) -> Result<(), Failure> {
        let mut boundaryloops = LoopList::new();
        self.my_area.clear();
        ls.init_loop()?;
        while ls.more_loop() {
            self.classify_next_loop(ls, lc, force_class, &mut boundaryloops)?;
            ls.next_loop()?;
        }
        if flavor == Flavor::TwoD && !boundaryloops.is_empty() && self.my_area.is_empty() {
            self.my_area.append(boundaryloops.into_iter().collect());
        }
        self.init_area();
        Ok(())
    }
    fn classify_next_loop(
        &mut self,
        ls: &mut dyn TopOpeBRepBuildLoopSet,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        force: bool,
        boundary: &mut LoopList,
    ) -> Result<(), Failure> {
        let current = ls.loop_()?;
        if current.get().ok_or_else(null_loop)?.is_shape() && !force {
            return self.insert_boundary(lc, &current, boundary);
        }
        match self.area_containing(lc, &current, TopOpeBRepBuildLoopEnum::AnyLoop)? {
            Some(k) => self.insert_block_in_area(lc, &current, k, boundary),
            None => self.new_block_area(lc, &current, boundary),
        }
    }
    fn insert_boundary(
        &mut self,
        lc: &mut dyn TopOpeBRepBuildLoopClassifier,
        current: &Handle<dyn TopOpeBRepBuildLoop>,
        boundary: &mut LoopList,
    ) -> Result<(), Failure> {
        match self.area_containing(lc, current, TopOpeBRepBuildLoopEnum::Block)? {
            Some(k) => {
                self.my_area
                    .iter_mut()
                    .nth(k)
                    .unwrap()
                    .append(current.clone());
            }
            None => boundary.push(current.clone()),
        }
        Ok(())
    }
}
