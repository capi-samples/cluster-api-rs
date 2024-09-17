YQ_VERSION := "v4.43.1"
UPDATECLI_VERSION := "v0.82.0"
OUT_DIR := "_out"
ARCH := if arch() == "aarch64" { "arm64"} else { "amd64" }
GO_ARCH := if arch() == "aarch64" { "arm64"} else { "x86_64" }
DIST := os()
REFRESH_BIN := env_var_or_default('REFRESH_BIN', '0')

export PATH := "_out:_out/bin:" + env_var('PATH')

[private]
default:
    @just --list --unsorted --color=always

# generates files for CRDS
generate: _create-out-dir update-version && fmt
    #!/usr/bin/env bash
    set -euxo pipefail
    version=`just current-version ".cluster_api.tag"`
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_clusters.yaml" "src/api/capi_cluster.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_clusterclasses.yaml" "src/api/capi_clusterclass.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_clusterclasses.yaml" "src/api/capi_clusterclass.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_machines.yaml" "src/api/capi_machine.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_machinepools.yaml" "src/api/capi_machinepool.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_machinesets.yaml" "src/api/capi_machineset.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_machinedeployments.yaml" "src/api/capi_machinedeployment.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/cluster.x-k8s.io_machinehealthchecks.yaml" "src/api/capi_machinehealthcheck.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/ipam.cluster.x-k8s.io_ipaddressclaims.yaml" "src/api/capi_ipaddressclaim.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/ipam.cluster.x-k8s.io_ipaddresses.yaml" "src/api/capi_ipaddress.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/runtime.cluster.x-k8s.io_extensionconfigs.yaml" "src/api/capi_extensionconfig.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/addons.cluster.x-k8s.io_clusterresourcesets.yaml" "src/api/capi_clusterresourceset.rs" ""
    just _generate-kopium-url kopium "https://raw.githubusercontent.com/kubernetes-sigs/cluster-api/${version}/config/crd/bases/addons.cluster.x-k8s.io_clusterresourcesetbindings.yaml" "src/api/capi_clusterresourcesetbinding.rs" "" ""

[private]
_generate-kopium-url kpath="" source="" dest="" derive="-D Default" yqexp="." condition="": _download-yq _install-kopium
    curl -sSL {{source}} | yq '{{yqexp}}' | {{kpath}} {{derive}} {{condition}} -A -d -f - > {{dest}}

current-version path: _download-yq
    cat version.yaml | yq '{{path}}'

update-version: _download-updatecli
    updatecli apply --debug

generate-and-commit: generate
    #!/usr/bin/env bash
    set -euxo pipefail
    just add-and-commit `just current-version ".cluster_api.tag"`

add-and-commit version:
    git add .
    -[[ -z "$(git status -s)" ]] || git commit -sm "Bump to version to {{version}}"

# format with nightly rustfmt
fmt:
    cargo fmt

# Install kopium
[private]
_install-kopium:
    #!/usr/bin/env bash
    set -euxo pipefail
    [ -z `which kopium` ] || [ {{REFRESH_BIN}} != "0" ] || exit 0
    cargo install --git https://github.com/kube-rs/kopium.git --root {{OUT_DIR}}

# Download yq
[private]
[linux]
_download-yq:
    #!/usr/bin/env bash
    set -euxo pipefail
    [ -z `which yq` ] || [ {{REFRESH_BIN}} != "0" ] || exit 0
    curl -sSL https://github.com/mikefarah/yq/releases/download/{{YQ_VERSION}}/yq_linux_{{ARCH}} -o {{OUT_DIR}}/yq
    chmod +x {{OUT_DIR}}/yq

[private]
[macos]
_download-yq:
    #!/usr/bin/env bash
    set -euxo pipefail
    [ -z `which yq` ] || [ {{REFRESH_BIN}} != "0" ] || exit 0
    curl -sSL https://github.com/mikefarah/yq/releases/download/{{YQ_VERSION}}/yq_darwin_{{ARCH}} -o {{OUT_DIR}}/yq
    chmod +x {{OUT_DIR}}/yq

[private]
[linux]
_download-updatecli:
    #!/usr/bin/env bash
    set -euxo pipefail
    [ -z `which updatecli` ] || [ {{REFRESH_BIN}} != "0" ] || exit 0
    curl -sSL -o {{OUT_DIR}}/updatecli_{{GO_ARCH}}.tar.gz https://github.com/updatecli/updatecli/releases/download/{{UPDATECLI_VERSION}}/updatecli_Linux_{{GO_ARCH}}.tar.gz
    cd {{OUT_DIR}}
    tar -xzf updatecli_{{GO_ARCH}}.tar.gz
    chmod +x updatecli

[private]
[macos]
_download-updatecli:
    #!/usr/bin/env bash
    set -euxo pipefail
    [ -z `which updatecli` ] || [ {{REFRESH_BIN}} != "0" ] || exit 0
    curl -sSL -o {{OUT_DIR}}/updatecli_{{GO_ARCH}}.tar.gz https://github.com/updatecli/updatecli/releases/download/{{UPDATECLI_VERSION}}/updatecli_Darwin_{{GO_ARCH}}.tar.gz
    cd {{OUT_DIR}}
    tar -xzf updatecli_{{GO_ARCH}}.tar.gz
    chmod +x updatecli

[private]
_create-out-dir:
    mkdir -p {{OUT_DIR}}