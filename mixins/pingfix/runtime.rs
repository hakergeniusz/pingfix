// The pingfix's brain: pure standard library, no game types, so it compiles
// and tests wherever — the game's screens are a world away from this crate.
//
// A server list is pinged once, when it opens. Whatever a server answered
// then — down, unreachable, or timed out because the network changed under
// the game (Wi-Fi switched, IP renewed, VPN closed) — stands for as long as
// the screen stands: vanilla never asks again, and neither does Horizon.
// 0.1.1 answered with a refresh every 1200 frames; the user's spec is
// sharper: the pane's answers should be there the moment it opens, so the
// very first frame of an open list fires the game's own F5 (`refresh_servers`
// rebuilds the list, which re-pings every server from scratch), and the
// interval then keeps the answers fresh at half the old wait.
use core::cell::Cell;

/// Rendered frames between automatic refreshes while the server list is
/// open: about 10 seconds at 60 fps, so a list that just refreshed never
/// spins and the scroll position is not yanked around too often.
pub const REFRESH_FRAMES: u32 = 600;

thread_local! {
    /// How many rendered frames the server list has been standing open —
    /// 0 means it is not open (a frame with the list closed resets this,
    /// so the next opening counts as the first frame again).
    static OPEN: Cell<u32> = const { Cell::new(0) };
}

/// One rendered frame on the title screen. `list_open`: the multiplayer
/// server list is the open screen. Returns whether the list should refresh
/// now: the frame that opens it (an instant ping, per the mod's spec), and
/// then every interval frame while it stands.
pub fn frame(list_open: bool) -> bool {
    if !list_open {
        OPEN.set(0);
        return false;
    }
    let open = OPEN.get().saturating_add(1);
    OPEN.set(open);
    let refresh = open == 1 || open > REFRESH_FRAMES;
    if refresh {
        OPEN.set(0);
    }
    refresh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_the_list_pings_instantly() {
        assert!(frame(true), "the first open frame refreshes right away");
    }

    #[test]
    fn the_interval_holds_after_the_instant_ping() {
        assert!(frame(true), "instant ping on open");
        for _ in 0..REFRESH_FRAMES {
            assert!(!frame(true), "the interval holds before the next ping");
        }
        assert!(frame(true), "and the next interval refreshes again");
    }

    #[test]
    fn reopening_the_list_pings_instantly_again() {
        for _ in 0..500 {
            frame(true);
        }
        assert!(!frame(false), "a closed list never refreshes");
        assert!(frame(true), "a reopened list pings instantly, not 20 s in");
    }

    #[test]
    fn a_closed_list_never_refreshes() {
        for _ in 0..(REFRESH_FRAMES * 3).max(10) {
            assert!(
                !frame(false),
                "the title screen's other pages never refresh"
            );
        }
        assert_eq!(OPEN.get(), 0, "a closed list forgets its open count");
    }
}
