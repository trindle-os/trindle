// Origin: smithay-drm-extras (MIT, Copyright (c) Bartłomiej Maryńczak), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::collections::HashMap;

use drm::control::{connector, Device as ControlDevice};

#[derive(Debug, Default)]
pub struct ConnectorScanner {
    connectors: HashMap<connector::Handle, connector::Info>,
}

impl ConnectorScanner {
    pub fn scan(&mut self, drm: &impl ControlDevice) -> std::io::Result<ConnectorScanResult> {
        let res_handles = drm.resource_handles()?;
        let connector_handles = res_handles.connectors();

        let mut added = Vec::new();
        let mut removed = Vec::new();

        for conn in connector_handles
            .iter()
            .filter_map(|conn| drm.get_connector(*conn, true).ok())
        {
            let curr_state = conn.state();

            use connector::State;
            if let Some(old) = self.connectors.insert(conn.handle(), conn.clone()) {
                match (old.state(), curr_state) {
                    (State::Connected, State::Disconnected) => removed.push(conn),
                    (State::Disconnected | State::Unknown, State::Connected) => added.push(conn),
                    (State::Connected, State::Connected) => {}
                    (State::Disconnected, State::Disconnected) => {}
                    (State::Unknown, _) => {}
                    (_, State::Unknown) => {}
                }
            } else if curr_state == State::Connected {
                added.push(conn)
            }
        }

        Ok(ConnectorScanResult {
            connected: added,
            disconnected: removed,
        })
    }

    pub fn connectors(&self) -> &HashMap<connector::Handle, connector::Info> {
        &self.connectors
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConnectorScanResult {
    pub connected: Vec<connector::Info>,
    pub disconnected: Vec<connector::Info>,
}
