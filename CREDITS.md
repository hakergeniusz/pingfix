# Credits

- **PingFix** ("FIX MY PINGGGGG") for Minecraft Java Edition — by
  [Itamio](https://modrinth.com/user/Itamio), at
  <https://modrinth.com/mod/pingfix>. All Rights Reserved. This Horizon port
  reimplements the mod's observable behavior (periodically refreshing the
  multiplayer server list so stale pings never persist) from scratch, with
  none of the original's code; the 1200-frame interval is the original's
  choice, kept for the same feel.
- **Horizon** — <https://github.com/horizon-rust/horizon>, whose own
  `refresh_servers` (the F5 path) does the actual work.
- Vanilla Minecraft's `ServerStatusPinger` naming, kept by Horizon's
  doc comments, guided the wording here.
