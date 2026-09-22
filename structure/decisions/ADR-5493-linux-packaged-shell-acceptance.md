# ADR-5493 — decision recorded under "Linux packaged-shell acceptance"

- Contract owner: [desktop-shell.md](../desktop-shell.md#linux-packaged-shell-acceptance)

## Decision record

- 목적과 의도: Make ordinary Linux pull requests prove the package users launch instead of proving only that the Rust shell compiles.
- 기존 구현 및 제약 조건: The Rust-only job used empty sidecar and resource stubs, while the real-install gate needs published releases, operator GUI hooks, and a protected self-hosted runner. linuxdeploy also rewrote the Bun external binary into a crashing executable, and sequential Linux formats shared Tauri's patched release binary.
- 검토한 주요 대안: Install deb packages directly on hosted runners; require the privileged installed-artifact gate for every pull request; keep Bun as a Tauri external binary; or extract both package payloads and exercise their shared runtime path with format-local build roots.
- 선택한 방식: Package Bun as an executable Linux resource, build AppImage and deb under independent Cargo targets, stage both outputs, and run the extracted payloads under isolated homes, an isolated loopback port, Xvfb, Openbox, and D-Bus.
- 다른 대안 대신 이 방식을 선택한 이유: The selected path covers bundle layout, WebKit startup, the real bundled sidecar, no-tray behavior, and coordinated exit without changing the hosted runner's package database or granting workflow write permissions.
- 장점, 단점 및 영향: Desktop changes gain bounded Linux package acceptance and diagnostic evidence. The lane does not prove dpkg maintainer scripts, desktop integration, elevation, signed updates, or a physical compositor; those remain the installed-artifact gate's responsibility.
