# Shared by the scripts in bin/. Sourced, never run.

# Exit 75: the environment cannot run the gate. Never a verdict on the change.
unavailable() {
  printf '%s: %s (not run; exit 75)\n' "$(basename "$0")" "$1" >&2
  exit 75
}

# The tools mise installs for this repository win over whatever else is on PATH. In CI the mise
# action has already put them there. A mise that cannot answer (config not trusted, tools not
# installed) stops the script: the gate never runs on whatever version happens to be around.
use_pinned_tools() {
  export PATH="$HOME/.cargo/bin:$PATH"
  if [ -z "${GITHUB_ACTIONS:-}" ] && command -v mise >/dev/null 2>&1; then
    local pinned
    pinned="$(mise env --shell bash 2>/dev/null)" || unavailable 'mise cannot load .mise.toml: run bin/setup'
    eval "$pinned"
  fi
}

need() {
  local tool
  for tool in "$@"; do
    command -v "$tool" >/dev/null 2>&1 || unavailable "'$tool' is missing: run bin/setup"
  done
}

# SHA-256 of a file, lowercase hex. sha256sum on Linux and Git for Windows, shasum on macOS.
sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | cut -d' ' -f1
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | cut -d' ' -f1
  else
    unavailable "neither sha256sum nor shasum is installed"
  fi
}
