# hyprqtile
Qtile-like workspaces and monitors management for the Hyprland compositor that uses hyprland-rs to communicate with the compositor's sockets. 

# installation
## Cargo 
```bash
cargo install hyprqtile 
```
> [!WARNING]
> Make sure that .cargo/bin is in your PATH

## arch
### aur
not supported yet jiji
```bash
yay -S hyprqtile
```

### manual
```bash
git clone https://github.com/daniqss/hyprqtile && cd hyprqtile && makepkg -si
```

# usage
```hyprlang
bind = SUPER, 1, exec, hyprqtile --workspace 1
bind = SUPER, 2, exec, hyprqtile --workspace 2
bind = SUPER, 3, exec, hyprqtile --workspace 3
bind = SUPER, 4, exec, hyprqtile --workspace 4

bind = SUPER, right, exec, hyprqtile --next
bind = SUPER, left, exec, hyprqtile --previous
```

# contributing
this program satisfies my needs, but if you want to add a feature, feel free to open a PR.
I don't use (nor understand) nix, so adding some nix magic to support it would be awesome. 

# thanks to
- [hyprland-community](https://github.com/hyprland-community/) mainteiners, who wrote an amazing [hyprland-rs](https://github.com/hyprland-community/hyprland-rs) crate
- [taylor85345](https://github.com/taylor85345), who made this [script](https://github.com/taylor85345/hyprland-dotfiles/blob/master/hypr/scripts/workspace) from which I take the logic
- [donovanglover](https://github.com/donovanglover), who made [hyprnome](https://github.com/donovanglover/hyprnome/) from which I take the idea and I learn how to make a rust program that uses hyprland-rs and some clap tricks.
