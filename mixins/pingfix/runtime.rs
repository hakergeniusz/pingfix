// The pingfix's brain: pure standard library, no game types, so it compiles
// and tests wherever — the game's screens are a world away from this crate.
//
// A server list is pinged once, when it opens. Whatever a server answered
// then — down, unreachable, or timed out because the network changed under
// the game (Wi-Fi switched, IP renewed, VPN closed) — stands for as long as
// the screen stands: vanilla never asks again, and neither does Horizon.
// The original mod's answer was to open the screen anew every 1200 rendered
// frames while it is open; Horizon's own F5 does exactly that (it rebuilds
// the list, which re-pings every server from scratch), and this counter
// says when to press it.
use core::cell::Cell;

/// Rendered frames between automatic refreshes while the server list is
/// open (the original's `REFRESH_INTERVAL_FRAMES`): about 20 seconds at
/// 60 fps, so a list that just refreshed never spins.
pub const REFRESH_FRAMES: u32 = 1200;

thread_local! {
    /// How many rendered frames the server list has been standing open
    /// since the last refresh — or since it was left, which forgets the
    /// count (a list reopened soon after should still get a fair run).
    static FRAMES: Cell<u32> = const { Cell::new(0) };
}

/// One rendered frame on the title screen. `list_open`: the multiplayer
/// server list is the open screen. Returns whether the list should refresh
/// now — the count restarts either way, and a frame with the list closed
/// forgets the count outright.
pub fn frame(list_open: bool) -> bool {
    FRAMES.with(|frames| {
        if !list_open {
            frames.set(0);
            return false;
        }
        let next = frames.get().saturating_add(1);
        let refresh = next >= REFRESH_FRAMES;
        frames.set(if refresh { 0 } else { next });
        refresh
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_refreshes_every_1200_open_frames() {
        for _ in 0..REFRESH_FRAMES - 1 {
            assert!(!frame(true), "an open list counts up before the interval");
        }
        assert!(frame(true), "the interval's last frame refreshes the list");
        for _ in 0..REFRESH_FRAMES - 1 {
            assert!(!frame(true), "the count restarts after a refresh");
        }
        assert!(frame(true), "and the next interval refreshes again");
    }

    #[test]
    fn leaving_the_list_forgets_the_count() {
        for _ in 0..500 {
            frame(true);
        }
        assert!(!frame(false), "a closed list never refreshes");
        // A list reopened right away counts from the start, so it gets a
        // full interval before its first automatic refresh.
        for _ in 0..REFRESH_FRAMES - 1 {
            assert!(!frame(true), "the count started over");
        }
        assert!(frame(true));
    }

    #[test]
    fn a_closed_list_never_refreshes() {
        for _ in 0..(REFRESH_FRAMES * 3).max(10) {
            assert!(
                !frame(false),
                "the title screen's other pages never refresh"
            );
        }
    }
}
