env_files = [
    { path = "./.env.dist.local", profile = "development" }
]

[tasks.clippy]
command = "cargo"
args = ["clippy", "--", "-D", "warnings"]

[tasks.format-check]
command = "cargo"
args = ["fmt", "--all", "--", "--check"]

[tasks.watch]
description = "Run the app in dev mode."
command = "cargo"
args = ["run", "--features=dev"]
watch = true

[tasks.run]
description = "Run the app in dev mode."
command = "cargo"
args = ["run", "--features=dev"]

[tasks.dotenv-linter]
description = "Runs all dotenv checks"
dependencies = ["dotenv-linter-check", "dotenv-linter-compare"]

[tasks.dotenv-linter-check]
description = "Formally checks dotenv files"
command = "dotenv-linter"
args = ["--skip", "LowercaseKey"]

[tasks.dotenv-linter-compare]
description = "Compare dotenv files"
command = "dotenv-linter"
args = ["compare", ".env.dist.drone", ".env.dist.local", ".env.dist.production", ".env.dist.qa", ".env.dist.staging"]

[tasks.release]
description = "Makes release."
run_task = { name = ["sweep-start", "build-release", "archive"]}

[tasks.sweep-start]
description = "Start cargo sweep"
command = "cargo"
args = ["sweep", "-s"]

[tasks.build-release]
description = "Runs cargo build --release."
command = "cargo"
args = [ "build", "--release", "--features=${ENV}"]

[tasks.archive]
description = "Creates archive with binaries."
script = [
    "cp -p target/release/splash .",
    "cp -p target/release/migrate .",
    "tar cfz ${VERSION}-${ENV}.tar.gz config splash .env.dist.* migrate",
    "rm splash migrate"
]

[tasks.cache-cleanup]
description = "Clean CI cache"
dependencies = ["cargo-prune", "delete-artifacts", "print-stats"]

[tasks.cargo-prune]
description = "Run cargo prune"
command = "cargo"
args = ["prune"]

[tasks.delete-artifacts]
description = "Remove non cachable artifacts"
script = [
'''
#!/bin/bash
set -e
set -x
find ./target/debug -type f -maxdepth 1 -delete || true
rm -rfv ./target/{debug,release}/deps/{*lira*,*decode_key*,*gen_public_key*,*intermediari_importer*,*migrate*,*craft*,*rabbit_worker*,*seed*}
rm -rfv ./target/{debug,release/.fingerprint/*lira*
'''
]

[tasks.print-stats]
description = "Print cache size"
command = "du"
args = ["-sh", "target", ".cargo"]


[tasks.test]
description = "Run tests."
command = "cargo"
args = ["test"]
dependencies=["db-reset-test"]
env={"DB_NAME"="${DB_NAME}_test"}

[tasks.watch-test]
description = "Run tests on file change."
command = "cargo"
args = ["test"]
watch = true
env={"DB_NAME"="${DB_NAME}_test"}

[tasks.test-coverage]
description = "Run tests coverage."
command = "cargo"
args = ["tarpaulin", "--exclude-files", "target"]
