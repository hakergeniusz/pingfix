# PingFix for Horizon

Keeps the multiplayer server list's pings fresh: while the list is open, it
refreshes itself about every 20 seconds (1200 rendered frames — the same
interval the original mod uses), the way the game's own **F5** does. A server
that was down, unreachable, or timed out because the network changed under
the game — Wi-Fi switched, IP renewed, VPN closed — answers again within
seconds, instead of keeping its stale failure for as long as the screen
stands.

A Horizon native mod (a mixin bundle the launcher compiles on the pinned
Horizon revision) ported from [PingFix](https://modrinth.com/mod/pingfix)
("FIX MY PINGGGGG") by Itamio.

## How it works

- Every frame on the title screen, the mod counts frames while the
  multiplayer server list is the open screen (`Title::update_toast`).
- Every 1200 of them it calls the game's own `Title::refresh_servers` —
  the same rebuild the Refresh button and F5 do — which re-pings every
  saved server and LAN game from scratch.
- Leaving the list (or the title's other pages) resets the count; the
  count pauses under a screen opened over the title.

Horizon's own network code is already free of the stale-route disease the
original fixes in Java (every ping resolves addresses fresh and connects
without binding a local address) — what stays is the always-fresh list:
pings run once when the screen opens, so a stale failure never heals on
its own. This mod makes it heal, exactly as the original does.

## Install

Put `pingfix-<version>.hnmod` in your launcher's mods (or install it from
the registry) and rebuild the native client. Settings: none — the interval
is the original's, on purpose.

## License

MIT, like Horizon modding at large. This is an **independent reimplementation**
of the original's observable behavior, written from scratch in Rust against
Horizon's own screens: none of the original's code (a single Java class for
NeoForge) is copied or translated. The original is **All Rights Reserved** —
this port credits it and links it, and exists at its author's discretion:
if Itamio objects, this port will be taken down.

See [CREDITS.md](CREDITS.md).
