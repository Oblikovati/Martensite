//! Caller-owned initialized slots before the sample origin remain real storage.
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarSquareBufferEvaluation as Call, AdvApp2VarSquareIntegerBindings as Bindings,
    AdvApp2VarSquareOperationFrame as Frame, AdvApp2VarSquareSampleFrame as Samples,
    approx_f2var::{SampledSquare, SquareTables},
};
use std::sync::Mutex;
const ROOTS: [f64; 3] = [-0.7745966692414834, 0., 0.7745966692414834];
fn request(iso: i32, negative: bool) -> SampledSquare<'static> {
    SampledSquare {
        ndimen: 1,
        uintfn: [0., 1.],
        vintfn: [0., 1.],
        nbpntu: if negative && iso == 2 { -1 } else { 3 },
        nbpntv: if negative && iso == 1 { -1 } else { 3 },
        urootb: &ROOTS,
        vrootb: &ROOTS,
        isofav: iso,
    }
}
fn caller_frame(request: &SampledSquare<'_>) -> Frame {
    Frame::new(
        vec![1, request.nbpntu, request.nbpntv, request.isofav, 7],
        Bindings {
            dimension: 0,
            root_counts: [1, 2],
            favorite_iso: 3,
        },
    )
    .unwrap()
    .with_owned_status_slot(4)
    .unwrap()
}
struct SamplePrefixFixture {
    request: SampledSquare<'static>,
    frame: Frame,
    samples: Samples,
    soso: [f64; 128],
    diso: [f64; 128],
    sodi: [f64; 128],
    didi: [f64; 128],
}
impl SamplePrefixFixture {
    fn new(iso: i32, negative: bool) -> Self {
        let request = request(iso, negative);
        let frame = caller_frame(&request);
        let mut values = vec![0.; 128];
        values[63] = 42.;
        let samples = Samples::new(values, 96, 64).unwrap();
        Self {
            request,
            frame,
            samples,
            soso: [7.; 128],
            diso: [11.; 128],
            sodi: [13.; 128],
            didi: [17.; 128],
        }
    }
    fn sample(
        &mut self,
        evaluate: &impl martensite_tkgeombase::adv_app2var::AdvApp2VarSquareBufferEvaluator,
    ) {
        let mut tables = SquareTables {
            sosotb: &mut self.soso[32..],
            disotb: &mut self.diso[32..],
            soditb: &mut self.sodi[32..],
            diditb: &mut self.didi[32..],
        };
        self.frame
            .sample_square_owned_samples(
                &mut self.request,
                evaluate,
                &mut tables,
                &mut self.samples,
            )
            .unwrap();
    }
    fn verify(&self, negative: bool) {
        assert_eq!(self.frame.status().unwrap(), 0);
        assert_eq!(
            &self.soso[32..34],
            if negative { &[49., 91.] } else { &[7., 7.] }
        );
        assert!(
            self.soso[..32]
                .iter()
                .chain(&self.soso[34..])
                .all(|value| *value == 7.)
        );
        assert!(self.diso.iter().all(|value| *value == 11.));
        assert!(self.sodi.iter().all(|value| *value == 13.));
        assert!(self.didi.iter().all(|value| *value == 17.));
    }
}
fn run_fixture(iso: i32, negative: bool) {
    let mut fixture = SamplePrefixFixture::new(iso, negative);
    let calls = Mutex::new(0);
    let evaluate = |call: &mut Call<'_, '_>| {
        *calls.lock().unwrap() += 1;
        call.set_status(0).unwrap();
    };
    fixture.sample(&evaluate);
    assert_eq!(*calls.lock().unwrap(), 3);
    fixture.verify(negative);
}
#[test]
fn positive_iso1() {
    run_fixture(1, false);
}
#[test]
fn positive_iso2() {
    run_fixture(2, false);
}
#[test]
fn initialized_sample_prefix_iso1() {
    run_fixture(1, true);
}
#[test]
fn initialized_sample_prefix_iso2() {
    run_fixture(2, true);
}
