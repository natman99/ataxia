# Rhai Engine Function Documentation

The `Engine` exposed by the **ataxia_scripting** crate contains a set of helper functions that can be used from user‑defined scripts.  These functions are registered in [`src/lib.rs`](../src/lib.rs) via `engine.register_fn`.  The following list documents each registered function, including its name, parameters, return type, and any special behaviour (e.g., early returns on parse failure or conversions performed on `ImmutableString`).

> **Note**: The functions are intentionally lightweight.  Many of them return `()`; when a parsing operation fails they simply return early without modifying the target object – this is considered a *failure state*.

## General Helper Functions

| Function | Parameters | Return type | Notes |
|----------|------------|-------------|-------|
| **class** | `level: i64`, `name: ImmutableString` | `Class` | Creates a new `Class`. The Rust function is `Class::new(Level, &str)`; the script receives an integer level and a string name. |

## Spell‑Related Functions

The following functions operate on the `Spells` collection or individual `Spell` objects.

| Function | Parameters | Return type | Notes |
|----------|------------|-------------|-------|
| **add** | `spells: &mut Spells`, `s: Spell` | `()` | Inserts spell into the map keyed by its name. No failure path.
| **spell(name)** | `name: ImmutableString` | `Spell` | Returns a new `Spell` with only the name set; other fields default to zero/empty.
| **spell(name, description)** | `name: ImmutableString`, `description: ImmutableString` | `Spell` | As above but also sets `desc` field.
| **source** | `f: &mut Spell`, `source: ImmutableString` | `()` | Adds a single source string to the spell via `Source::Single`. Conversion uses `to_string()`.
| **component** | `spell: &mut Spell`, `c: ImmutableString` | `()` | Parses a `Component` from the string; on parse failure it returns early (no change). The `ImmutableString` is converted to `&str` with `.as_str()`.
| **area(shape, size)** | `spell: &mut Spell`, `shape: ImmutableString`, `size: i64` | `()` | Sets `spell.area` to a new `Area`. `size` cast to `i32`.
| **desc / description** | `spell: &mut Spell`, `d: ImmutableString` | `()` | Sets the spell's description; conversion via `.to_string()`.
| **cast_time** | `spell: &mut Spell`, `t: ImmutableString` | `()` | Assigns to `spell.cast_time`.
| **range** | `spell: &mut Spell`, `r: ImmutableString` | `()` | Assigns to `spell.range`.
| **duration** | `spell: &mut Spell`, `d: ImmutableString` | `()` | Assigns to `spell.duration`.
| **concentration** | `spell: &mut Spell`, `c: bool` | `()` | Sets `spell.concentration`.
| **ritual** | `spell: &mut Spell`, `r: bool` | `()` | Sets `spell.ritual`.
| **level** | `spell: &mut Spell`, `l: i64` | `()` | Assigns the numeric level to the spell.
| **automatic / melee / ranged** | `spell: &mut Spell` | `()` | Sets `spell.spell_type` to the corresponding variant (`Automatic`, `Melee`, or `Ranged`).
| **saving(dc_type)** | `spell: &mut Spell`, `dc: ImmutableString` | `()` | Parses a `Score` from the string; on failure returns early. If successful, sets `spell.spell_type = Saving { dc_type: parsed, success: Half }`.
| **saving(dc_type, success)** | `spell: &mut Spell`, `dc: ImmutableString`, `s: ImmutableString` | `()` | Parses both `Score` and `Success`. On any parse failure the function returns early. The success variant defaults to `Half` if parsing fails.
| **damage(damage)** | `spell: &mut Spell`, `dmg: ImmutableString` | `()` | Parses a `Roll`; on failure returns early. Sets effect to `Damage { damage: roll, damage_type: Force }`.
| **damage(damage, element)** | `spell: &mut Spell`, `dmg: ImmutableString`, `elem: ImmutableString` | `()` | Parses both `Roll` and `DamageType`; on failure returns early. Sets effect accordingly.
| **element** | `spell: &mut Spell`, `e: ImmutableString` | `()` | If the spell's current effect is a `Damage`, parses `DamageType` and updates it; otherwise does nothing. On parse failure returns early.
| **heal(healing)** | `spell: &mut Spell`, `h: ImmutableString` | `()` | Tries to parse a `Roll`; if successful uses `Heal::Roll`, otherwise stores as `Heal::Static`. Sets effect to the chosen variant.
| **school** | `spell: &mut Spell`, `s: ImmutableString` | `()` | Parses a `School` enum; on failure returns early.

## Item‑Related Functions

These functions create or modify `Item` objects.

| Function | Parameters | Return type | Notes |
|----------|------------|-------------|-------|
| **item(name, description)** | `name: ImmutableString`, `description: ImmutableString` | `Item` | Constructs an `Item` with given name and description; other fields default.
| **item(name, roll)** | `name: ImmutableString`, `roll: &Roll` | `Item` | Constructs an `Item` whose `roll` field is set to a clone of the provided `Roll`.
| **item(name, description, roll)** | `name: ImmutableString`, `description: ImmutableString`, `roll: &Roll` | `Item` | Sets both description and roll.
| **desc / description** | `f: &mut Item`, `d: ImmutableString` | `()` | Updates the item's description. Conversion via `.to_string()`.
| **count / quantity** | `f: &mut Item`, `c: i64` | `()` | Sets `Item::quantity` to `c as i32`.
| **feature** | `f: &mut Item`, `feat: Effect` | `()` | Adds a feature effect to the item via `add()`.
| **source** | `f: &mut Item`, `s: ImmutableString` | `()` | Adds source string via `Source::Single`; conversion uses `.to_string()`.

## Class‑Related Functions

These functions mutate the fields of a `Class` instance.

| Function | Parameters | Return type | Notes |
|----------|------------|-------------|-------|
| **set_class** | `class: &mut Class`, `c: ImmutableString` | `()` | Calls `class.set_class(c.as_str())`. The helper parses a `ClassType`; on parse failure it returns early.
| **set_level** | `class: &mut Class`, `lvl: i64` | `()` | Calls `class.set_level(lvl)`; always succeeds (no early return).
| **set_hit_die** | `class: &mut Class`, `die: ImmutableString` | `()` | Parses a `Die`; on failure returns early. On success assigns to `class.hit_dice`.

## Feature / Feat / Meter / Source Helpers

The following functions add a source string to various structs that implement the `HasSource` trait. They all perform the same conversion: `source.to_string()` and call `add_source(Source::Single(...))`.

| Function | Parameters | Target type | Notes |
|----------|------------|-------------|-------|
| **source** | `f: &mut Feature`, `s: ImmutableString` | `Feature` | Adds source string. Conversion via `.to_string()`.
| **source** | `f: &mut Feat`, `s: ImmutableString` | `Feat` |
| **source** | `f: &mut Meter`, `s: ImmutableString` | `Meter` |

---

### Early‑Return Behavior
Functions that parse input strings (`component`, `saving`, `damage`, `element`, `heal`, `school`, etc.) return early if parsing fails. They do **not** modify the target object and return the unit type `()`. This behaviour is considered a *silent failure* from the scripting perspective.

### ImmutableString Conversion
All functions receiving an `ImmutableString` perform one of the following conversions:
- `.to_string()` – creates a new owned `String`.
- `.as_str()` – provides a borrowed string slice for parsing.
These are documented where relevant in the notes.
