# https://just.systems

set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

default:
    just --list

launcher:
    cargo build -p bevy-sandbox-engine
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
    cargo run -p bevy-sandbox-engine-launcher

watch:
    if (-not (Get-Command cargo-watch -ErrorAction SilentlyContinue)) { Write-Error "cargo-watch is required. Install it with: cargo install cargo-watch"; exit 1 }
    cargo watch -x "run -p bevy-sandbox-engine-launcher"

sandbox-watch:
    if (-not (Get-Command cargo-watch -ErrorAction SilentlyContinue)) { Write-Error "cargo-watch is required. Install it with: cargo install cargo-watch"; exit 1 }
    cargo watch -x "run -p bevy-sandbox-engine"

launcher-backend backend="vulkan":
    $env:WGPU_BACKEND = "{{ backend }}"
    cargo run -p bevy-sandbox-engine-launcher

launcher-watch-backend backend="vulkan":
    if (-not (Get-Command cargo-watch -ErrorAction SilentlyContinue)) { Write-Error "cargo-watch is required. Install it with: cargo install cargo-watch"; exit 1 }
    $env:WGPU_BACKEND = "{{ backend }}"
    cargo watch -x "run -p bevy-sandbox-engine-launcher"

editor:
    cargo run --example simple_editor -p bevy-sandbox-engine-launcher

check:
    cargo check

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- --deny warnings

test:
    cargo test --workspace --all-features --all-targets
    cargo test --workspace --all-features --doc

doc:
    cargo doc --workspace --all-features --document-private-items --no-deps

book:
    cd design-book; mdbook build; mdbook test

ci: fmt-check clippy test doc

metadata:
    cargo metadata --no-deps --format-version 1

members:
    cargo metadata --no-deps --format-version 1 | ConvertFrom-Json | Select-Object -ExpandProperty workspace_members

incoming:
    $meta = cargo metadata --no-deps --format-version 1 | ConvertFrom-Json; $members = @{}; foreach ($id in $meta.workspace_members) { $members[$id] = $true }; $pkgs = @{}; foreach ($pkg in $meta.packages) { $pkgs[$pkg.id] = $pkg }; $incoming = @{}; foreach ($id in $meta.workspace_members) { $incoming[$id] = @() }; foreach ($pkg in $meta.packages) { if (-not $members.ContainsKey($pkg.id)) { continue }; foreach ($dep in $pkg.dependencies) { if (-not $dep.path) { continue }; $depManifest = Join-Path $dep.path 'Cargo.toml'; foreach ($candidate in $meta.packages) { if ($candidate.manifest_path -eq $depManifest -and $members.ContainsKey($candidate.id)) { $incoming[$candidate.id] += $pkg.name } } } }; $result = foreach ($id in $meta.workspace_members) { $pkg = $pkgs[$id]; $users = $incoming[$id] | Sort-Object -Unique; [PSCustomObject]@{ name = $pkg.name; manifest = $pkg.manifest_path.Replace($meta.workspace_root + '\\', ''); incoming_count = $users.Count; used_by = if ($users.Count -eq 0) { '' } else { ($users -join ', ') } } }; $result | Sort-Object incoming_count, name | Format-Table -AutoSize
