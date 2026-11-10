// Once a frame on the title screen: while the multiplayer server list is
// the open screen, the counter decides when it is time to refresh it —
// the game's own F5 (`refresh_servers` rebuilds the list, which re-pings
// every server from scratch). The title's other pages only reset the
// count, and a count paused under another screen (Options, say) waits
// there until the list is back.
fn pingfix_frame(title: &mut crate::title::Title) {
    let list_open = matches!(
        title.mode,
        crate::title::Mode::Multiplayer(crate::multiplayer::Screen::List { .. })
    );
    if crate::title::pingfix::frame(list_open) {
        title.refresh_servers();
    }
}
