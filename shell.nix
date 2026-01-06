# shell.nix
{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = [
    pkgs.pkg-config
    pkgs.gnumake
  ];

  buildInputs = [
    pkgs.rustup
    pkgs.git
    pkgs.espflash
    pkgs.cargo-generate
    pkgs.openssl
    pkgs.ldproxy
    pkgs.python311
    pkgs.libffi
    pkgs.gcc
    pkgs.libxml2
    pkgs.zlib
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.libxml2.out}/lib:${pkgs.zlib}/lib:$LD_LIBRARY_PATH"

    # --- Diagnostic checks ---
    echo "--- Running environment checks ---"
    check_library() {
      local lib_name="$1"
      local found=0
      local OLD_IFS="$IFS"
      IFS=':'
      for path in $LD_LIBRARY_PATH; do
        if [ -e "$path/$lib_name" ]; then
          found=1
          break
        fi
      done
      IFS="$OLD_IFS"

      if [ "$found" -eq 1 ]; then
        echo "Found required library: $lib_name"
      else
        echo "ERROR: Could not find required library '$lib_name'."
      fi
    }

    check_library "libstdc++.so.6"
    #check_library "libxml2.so.2"
    check_library "libxml2.so"
    check_library "libz.so.1"
    echo "--- Checks complete ---"

    export PATH="$HOME/.cargo/bin:$PATH"

    install_if_missing() {
      if ! command -v "$1" &> /dev/null; then
        echo "$1 not found. Installing now..."
        cargo install "$1" --locked
      else
        echo "$1 is already installed."
      fi
    }

    install_if_missing espup
    install_if_missing cargo-generate
    install_if_missing espflash

    echo "Checking for ESP toolchain..."
    if ! ls -d "$HOME/.espressif/tools/xtensa-esp32-elf"* 1>/dev/null 2>&1; then
        echo "ESP toolchain not found. Running 'espup install'..."
        espup install
    else
        echo "ESP toolchain is already installed."
    fi

    echo "Sourcing ESP environment variables..."
    if [ -f "$HOME/export-esp.sh" ]; then
      source "$HOME/export-esp.sh"
    fi
    echo "Nix shell is ready for ESP32 development"
  '';
}

