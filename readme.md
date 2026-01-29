**reproducible** way to brew villager-exploding potions:

**flake.nix:**
```nix
{
  description = "Deterministic villager explosion infrastructure";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    spongebob-overlay.url = "github:bikinibottom/spongebob-flake";
    tung-tung-tung.url = "github:soundeffects/tung-cubed";
    villager-utils = {
      url = "github:minecraft/villager-hmm";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ inputs.spongebob-overlay.overlays.default ];
      };
    in {
      packages.${system}.villager-boom = pkgs.stdenv.mkDerivation {
        pname = "villager-exploding-potion";
        version = "1.21.0-unstable";
        
        src = ./.;
        
        nativeBuildInputs = with pkgs; [
          spongebob
          tung-tung-tung
          emerald-dust
          cargo
          rustc
        ];
        
        buildInputs = with pkgs; [
          nether-wart
          creeper-hugs  # ethically-sourced = true
          dragons-breath
        ];
        
        buildPhase = ''
          export VILLAGER_HMMS=maximum
          cargo build --release \
            --features "splash,explosive,villager-seeking,pure"
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp target/release/villager-boom $out/bin/
          
          # Generate NixOS module
          cat > $out/share/nixos-module.nix <<EOF
          { config, lib, pkgs, ... }: {
            services.villager-explosion = {
              enable = lib.mkEnableOption "villager chaos";
              splashRadius = lib.mkOption {
                type = lib.types.int;
                default = 420;
              };
            };
          }
          EOF
        '';
        
        meta = with pkgs.lib; {
          description = "Reproducible villager explosion technology";
          license = licenses.unfree;  # Villagers did not consent
          platforms = platforms.minecraft;
          broken = !stdenv.isLinux;  # Works on NixOS btw
        };
      };
      
      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [ self.packages.${system}.villager-boom ];
        shellHook = ''
          echo "Entering pure villager-explosion development environment"
          echo "All dependencies are pinned and cached at /nix/store/..."
          alias hmm="tung-tung-tung --resonance=3"
        '';
      };
    };
}
```

**Usage:**
```bash
nix build .#villager-boom
nix run .#villager-boom -- --splash-radius=69 --particle-effects=maximum
```

**NixOS Configuration:**
```nix
{ config, pkgs, ... }: {
  imports = [ ./flake.nix ];
  
  services.villager-explosion.enable = true;
  
  environment.systemPackages = with pkgs; [
    villager-boom
    spongebob
  ];
  
  # Declarative villager targeting
  systemd.services.villager-chaos = {
    wantedBy = [ "multi-user.target" ];
    serviceConfig = {
      ExecStart = "${pkgs.villager-boom}/bin/villager-boom";
      DynamicUser = true;  # For safety (lol)
      ReadOnlyPaths = [ "/var/lib/villagers" ];
    };
  };
}
```

