// The mod's row in Options > Mods: the mod's identity and what it does.
// Copyright (c) 2026 hakergeniusz. MIT.
fn pingfix_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "pingfix".into(),
        title: "Pingfix 0.1.1".into(),
        description: "Keeps re-pinging the multiplayer server list while it is open, so servers that were down or unreachable answer again within seconds.".into(),
        keys: vec![],
    });
}
