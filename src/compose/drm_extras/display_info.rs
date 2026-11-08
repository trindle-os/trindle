// Origin: smithay-drm-extras (MIT, Copyright (c) Bartłomiej Maryńczak), changed by Trindle. Licence text: THIRD-PARTY.txt.

use drm::control::{connector, Device as ControlDevice};
use libdisplay_info::info::Info;

pub fn for_connector(device: &impl ControlDevice, connector: connector::Handle) -> Option<Info> {
    let props = device.get_properties(connector).ok()?;

    let (info, value) = props
        .into_iter()
        .filter_map(|(handle, value)| {
            let info = device.get_property(handle).ok()?;

            Some((info, value))
        })
        .find(|(info, _)| info.name().to_str() == Ok("EDID"))?;

    let blob = info.value_type().convert_value(value).as_blob()?;
    let data = device.get_property_blob(blob).ok()?;

    Info::parse_edid(&data).ok()
}
