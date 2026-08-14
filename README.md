Ataxia
===

Ataxia is an implementation of Dungeons and Dragons 5e mechanics in Rust :rocket:. 

## Project Layout

### Ataxia Types
The home of the core logic.
Should be usable for single classed builds.

Statically typed using Rust's type system. Nearly all mechanics covered and customizable.

- Types implement serde, JsonSchema, and Rhai bindings under the respective `serde`, `schema` and `rhai` features.

### Ataxia Cli
Tui for viewing character sheets.
Abandoned in favor of [Ataxia](#Ataxia).
- To be repurposed into a cli for AI agents to write Ataxia scripts and types.

### Ataxia Scripting
Integration with (rhai)[https://rhai.rs/] for building characters in a scripting language.

- Create a character from rhai script
- Script against a sheet in real time to calculate feature interactions and rolls.

```rust
name = "Bob";
classes[0].set_level(2);
classes[0].set_class("warlock");
classes[0].subclass = "cool subclass";
ability_scores.str = 14;
health.max = 42;
let i = spell("Cool Spell");
i.damage();
i.component("V");
i.desc("Cool spell goes boom");
i.cast_time("1 action");
i.level = 2;
i.ritual = true;
i.ranged();
i.damage("2d4", "fire");
spells.add(i);
```

### Ataxia
The GUI culmination of all the projects.

- [ x ] Hot reloading
  - Hot reloading preserves consumable resources and health across changes.
- [ x ] Basic character information 
- [ ] Inventory (partial)
- [ ] Spells + Meters
- [ x ] Integration with [ataxia_scripting](#ataxia-scripting)


## Building

This project uses info from the D&D Srd. Clone it before building.
```bash
git clone https://github.com/5e-bits/5e-database
cargo build
```

To install ataxia to Path
```bash
git clone https://github.com/5e-bits/5e-database
cargo install --path ataxia --locked
```
