// The mod's row in Options > Mods: the mod's identity and what it does.
// Copyright (c) 2026 hakergeniusz. MIT.
fn pingfix_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "pingfix".into(),
        title: "Pingfix 0.2.0".into(),
        description: "Pings the multiplayer server list the instant it opens and keeps the answers fresh — servers that were down or unreachable recover in seconds.".into(),
        keys: vec![],
    });
}
