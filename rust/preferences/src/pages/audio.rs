// ==========================================
// AudioPage — dispositivos de sonido (salida + micrófono)
// (equivalente a pages/audio.py)
// ==========================================


use crate::services::audio::AudioService;
use crate::widgets::combo_row::ComboRow;
use crate::widgets::group::Group;
use crate::widgets::page::Page;
use crate::widgets::slider_row::SliderRow;
use crate::widgets::switch_row::SwitchRow;

pub fn build(navigator: gtk::Stack) -> Page {
    let page = Page::new(
        Some(navigator),
        "Audio",
        Some("Configura los dispositivos de sonido"),
        None,
    );

    if !AudioService::available() {
        let mut group = Group::new("Audio");
        group.add(&SliderRow::new("Audio", None, None, 0.0, 100.0, 1.0, 0.0, None));
        page.add(group.widget());

        let mut err = Group::new("Estado");
        err.add(&SwitchRow::new("WirePlumber no disponible", None, None, false, None));
        page.add(err.widget());
        return page;
    }

    // ============ Salida ============
    let mut output_group = Group::new("Salida");

    let outputs = AudioService::outputs();

    let mut current_output = None;
    for device in &outputs {
        if device.default {
            current_output = Some(device.name.clone());
            break;
        }
    }

    let output_names: Vec<String> = outputs.iter().map(|d| d.name.clone()).collect();
    let output_refs: Vec<&str> = output_names.iter().map(|s| s.as_str()).collect();

    let outputs_clone = outputs.clone();
    let combo = ComboRow::new(
        "Dispositivo",
        &output_refs,
        current_output.as_deref(),
        None,
        None,
        Some(Box::new(move |value| {
            for device in &outputs_clone {
                if device.name == value {
                    AudioService::set_output(device);
                    break;
                }
            }
        })),
    );
    output_group.add(&combo);

    let slider = SliderRow::new(
        "Volumen",
        None,
        None,
        0.0,
        100.0,
        1.0,
        AudioService::output_volume(),
        Some(Box::new(|value| {
            AudioService::set_output_volume(value);
        })),
    );
    output_group.add(&slider);

    let mute = SwitchRow::new(
        "Silenciar",
        None,
        None,
        AudioService::output_muted(),
        Some(Box::new(|active| {
            AudioService::set_output_mute(active);
        })),
    );
    output_group.add(&mute);

    page.add(output_group.widget());

    // ============ Micrófono ============
    let mut input_group = Group::new("Micrófono");

    let inputs = AudioService::inputs();

    let mut current_input = None;
    for device in &inputs {
        if device.default {
            current_input = Some(device.name.clone());
            break;
        }
    }

    let input_names: Vec<String> = inputs.iter().map(|d| d.name.clone()).collect();
    let input_refs: Vec<&str> = input_names.iter().map(|s| s.as_str()).collect();

    let inputs_clone = inputs.clone();
    let combo = ComboRow::new(
        "Dispositivo",
        &input_refs,
        current_input.as_deref(),
        None,
        None,
        Some(Box::new(move |value| {
            for device in &inputs_clone {
                if device.name == value {
                    AudioService::set_input(device);
                    break;
                }
            }
        })),
    );
    input_group.add(&combo);

    let slider = SliderRow::new(
        "Volumen",
        None,
        None,
        0.0,
        100.0,
        1.0,
        AudioService::input_volume(),
        Some(Box::new(|value| {
            AudioService::set_input_volume(value);
        })),
    );
    input_group.add(&slider);

    let mute = SwitchRow::new(
        "Silenciar",
        None,
        None,
        AudioService::input_muted(),
        Some(Box::new(|active| {
            AudioService::set_input_mute(active);
        })),
    );
    input_group.add(&mute);

    page.add(input_group.widget());

    page
}
