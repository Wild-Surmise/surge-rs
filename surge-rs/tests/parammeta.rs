//! VAE fork additions: `get_parameter_meta` (real engine ranges + stable key)
//! and `save_patch`/`load_patch` (raw patch stream). Headless — no audio device.

use surge_rs::glue::synthesizer::{SurgeId, SurgeSynthesizer};

fn lookup(synth: &SurgeSynthesizer, name: &str) -> Option<SurgeId> {
    for i in 0..800 {
        let mut id = SurgeId::empty();
        if !synth.from_synth_side_id(i, &mut id) {
            continue;
        }
        let mut probe = SurgeId::empty();
        synth.from_synth_side_id(i, &mut probe);
        if synth.get_parameter_name(&mut probe) == name {
            return Some(id);
        }
    }
    None
}

#[test]
fn registry_has_real_engine_ranges() {
    let mut synth = SurgeSynthesizer::new(48000.0);
    synth.process();

    let mut count = 0usize;
    let mut widened = 0usize;
    for i in 0..800 {
        let mut id = SurgeId::empty();
        if !synth.from_synth_side_id(i, &mut id) {
            continue;
        }
        count += 1;
        let m = synth.get_parameter_meta(&mut id);
        assert!(
            m.min.is_finite() && m.max.is_finite(),
            "param {i} has non-finite range {}..{}",
            m.min,
            m.max
        );
        assert!(
            m.max >= m.min,
            "param {i} has inverted range {}..{}",
            m.min,
            m.max
        );
        // Surge's own meta reports 0..1 for everything; a real range proves we
        // are reading `val_min`/`val_max` rather than the zeroed fields.
        if (m.max - m.min) > 1.0 {
            widened += 1;
        }
    }
    assert!(count > 500, "expected a large registry, got {count}");
    assert!(
        widened > 20,
        "expected real (non 0..1) ranges for many params; got {widened}"
    );
}

#[test]
fn known_parameters_report_expected_ranges() {
    let mut synth = SurgeSynthesizer::new(48000.0);
    synth.process();

    let mut octave = lookup(&synth, "A Osc 1 Octave").expect("A Osc 1 Octave");
    let o = synth.get_parameter_meta(&mut octave);
    assert_eq!((o.min, o.max), (-3.0, 3.0), "octave range");
    assert!(o.is_discrete(), "octave must be discrete (valtype {})", o.valtype);

    let mut otype = lookup(&synth, "A Osc 1 Type").expect("A Osc 1 Type");
    let t = synth.get_parameter_meta(&mut otype);
    assert_eq!((t.min, t.max), (0.0, 11.0), "osc type range");

    let mut voices = lookup(&synth, "A Osc 1 Unison Voices").expect("unison voices");
    let v = synth.get_parameter_meta(&mut voices);
    assert_eq!((v.min, v.max), (1.0, 16.0), "unison voices range");
}

#[test]
fn patch_save_load_round_trips_byte_for_byte() {
    let mut a = SurgeSynthesizer::new(48000.0);
    a.process();
    for (name, val) in [("A Osc 1 Octave", 0.9f32), ("A Filter 1 Cutoff", 0.2)] {
        if let Some(mut id) = lookup(&a, name) {
            a.set_parameter01(&mut id, val, None, None);
        }
    }
    let bytes_a = a.save_patch();
    assert!(bytes_a.len() > 100, "patch stream too small: {}", bytes_a.len());

    let mut b = SurgeSynthesizer::new(48000.0);
    b.process();
    b.load_patch(&bytes_a);
    let bytes_b = b.save_patch();
    assert_eq!(
        bytes_a.len(),
        bytes_b.len(),
        "save->load->save changed patch length"
    );
    assert_eq!(bytes_a, bytes_b, "save->load->save must be byte-identical");
}
