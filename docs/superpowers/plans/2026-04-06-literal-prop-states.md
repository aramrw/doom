# Literal Prop State Mapping Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Update the Rust importer to map Doom states directly to Godot animations for props, ensuring accuracy and complete frame sets.

**Architecture:** 
- The asset extractor will use lowercase Doom state labels as folder names for props.
- The resource generator will create individual animations for each state and set an appropriate `autoplay` animation.

**Tech Stack:** Rust (godot-rust), GDScript

---

### Task 1: Update Asset Extraction for Props

**Files:**
- Modify: `rust/src/realm667/mod.rs`

- [ ] **Step 1: Modify extract_actor_assets_clean to handle Prop category dynamically**

Replace the fixed mapping for `ActorCategory::Prop` with a loop-based approach that uses state labels.

```rust
// In rust/src/realm667/mod.rs

// Inside extract_actor_assets_clean function
// Replace:
        } else if category == ActorCategory::Prop {
            label_to_folder.insert("Spawn", "idle");
            label_to_folder.insert("Active", "idle");
            label_to_folder.insert("Inactive", "idle");
        }

// With:
        } else if category == ActorCategory::Prop {
            // For props, we use literal state names as folders
            for label in actor.states.keys() {
                label_to_folder.insert(label.as_str(), label.to_lowercase());
            }
        }
```

- [ ] **Step 2: Build and verify compilation**

Run: `./build.sh`
Expected: PASS

- [ ] **Step 3: Commit changes**

```bash
git add rust/src/realm667/mod.rs
git commit -m "feat: use literal state names for prop asset extraction"
```

---

### Task 2: Update Prop Resource Generation

**Files:**
- Modify: `rust/src/realm667/resource_gen.rs`

- [ ] **Step 1: Update generate_prop_resources to handle multiple animations**

Modify the `SpriteFrames` generation and `autoplay` logic.

```rust
// In rust/src/realm667/resource_gen.rs

// Inside generate_prop_resources function
// Update animation loop:
        for anim_name in keys {
            let sprites = label_sprites.get(anim_name).unwrap();
            if sprites.is_empty() { continue; }
            
            let mut frames = Vec::new();
            for (sprite_rel_path, duration) in sprites {
                let id = format!("{}_ext", id_counter);
                ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, id));
                let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                id_counter += 1;
            }
            
            // Loop should be true for all prop animations by default
            animations.push(format!(
r#"{{
"frames": [{}],
"loop": true,
"name": &"{}",
"speed": 35.0
}}"#, frames.join(", "), anim_name)); // Use original lowercase anim_name from keys
        }

// Update autoplay logic:
        let autoplay_anim = if label_sprites.contains_key("spawn") {
            "spawn"
        } else if label_sprites.contains_key("active") {
            "active"
        } else if !label_sprites.is_empty() {
            label_sprites.keys().next().unwrap()
        } else {
            ""
        };
```

- [ ] **Step 2: Update volumetric sprite autoplay**

Ensure the generated `.tscn` uses the new `autoplay_anim` variable.

```rust
// In rust/src/realm667/resource_gen.rs

// Inside generate_prop_resources, the Sprite3D loop:
        for i in 0..4 {
            let rot = (i as f32) * 45.0;
            tscn_content.push_str(&format!(
r#"
[node name="Sprite3D_{}" type="AnimatedSprite3D" parent="."]
transform = Transform3D({:.4}, 0, {:.4}, 0, 1, 0, {:.4}, 0, {:.4}, 0, {:.4}, 0)
shaded = true
texture_filter = 0
sprite_frames = ExtResource("1_sprites")
autoplay = "{}"
"#, 
                i, 
                (rot.to_radians()).cos(), (rot.to_radians()).sin(),
                -(rot.to_radians()).sin(), (rot.to_radians()).cos(),
                h_val / 2.0,
                autoplay_anim
            ));
        }
```

- [ ] **Step 3: Build and verify compilation**

Run: `./build.sh`
Expected: PASS

- [ ] **Step 4: Commit changes**

```bash
git add rust/src/realm667/resource_gen.rs
git commit -m "feat: generate separate animations for each prop state"
```

---

### Task 3: Final Verification

- [ ] **Step 1: Perform full import test**

1. Restart Godot.
2. Select **Prop** mode.
3. Import `demonic_props.zip`.
4. Verify `godot_data/sprites/demonic_props/demonbrazier1/` contains `active/` and `inactive/` folders.
5. Verify `demonbrazier1_spriteframes.tres` has both `active` and `inactive` animations.
6. Verify `demonbrazier1.tscn` auto-plays `active` (or `spawn`).
