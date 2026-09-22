import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { repoPath } from "../helpers/repo-root";

const script = await Bun.file(repoPath("desktop", "scripts", "prepare-sidecar.ts")).text();

test("desktop sidecar preparation maps supported Rust targets", () => {
  for (const value of [
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
  ]) expect(script).toContain(value);
  expect(script).toContain("build:standalone");
  expect(script).toContain("binaries");
  expect(script).toContain("resources");
  expect(script).toContain("ocx-${triple}");
});

test("Linux packages the Bun CLI as an executable resource instead of a linuxdeploy sidecar", () => {
  const linuxConfig = JSON.parse(
    readFileSync(repoPath("desktop", "src-tauri", "tauri.linux.conf.json"), "utf8"),
  ) as { bundle?: { externalBin?: string[]; resources?: Record<string, string> } };
  expect(linuxConfig.bundle?.externalBin).toEqual([]);
  expect(linuxConfig.bundle?.resources?.["resources/sidecar/ocx"]).toBe("sidecar/ocx");
  expect(script).toContain('target.startsWith("bun-linux-")');
  expect(script).toContain('"resources", "sidecar", "ocx"');
  expect(script).toContain("chmodSync(destination, 0o755)");

  const launcher = readFileSync(
    repoPath("desktop", "src-tauri", "src", "bundled_cli.rs"),
    "utf8",
  );
  expect(launcher).toContain('resource_dir.join("sidecar").join("ocx")');
  expect(launcher).toContain("app.shell().command(executable)");
  expect(launcher).toContain('.sidecar("ocx")');
});
