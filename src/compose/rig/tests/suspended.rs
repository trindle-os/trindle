//! Windows nobody can see are told so, and told again when they show.

#[cfg(test)]
use crate::rig::*;

fn suspended(rig: &Rig) -> Vec<bool> {
    use smithay::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::State;
    rig.state.xdg_shell_state.toplevel_surfaces().iter()
        .map(|t| t.with_pending_state(|s| s.states.contains(State::Suspended)))
        .collect()
}

#[test]
fn a_window_on_a_workspace_not_showing_is_suspended_until_it_shows() {
    alone("rig::tests::suspended::a_window_on_a_workspace_not_showing_is_suspended_until_it_shows", |dir| {
        let mut rig = Rig::new(dir);
        let who = rig.join(ClientState::default());
        rig.window(who, false);
        rig.window(who, false);
        rig.state.set_workspace_count(2);
        rig.state.tidy_after_a_turn();
        assert_eq!(suspended(&rig), [false, false]);
        rig.state.go_to_workspace(1);
        rig.state.tidy_after_a_turn();
        assert_eq!(suspended(&rig), [true, true], "windows on a workspace not showing were not suspended");
        rig.state.go_to_workspace(0);
        rig.state.tidy_after_a_turn();
        assert_eq!(suspended(&rig), [false, false], "windows back on screen stayed suspended");
    });
}
