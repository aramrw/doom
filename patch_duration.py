import re
import os

# Edit mod.rs
mod_path = "rust/src/realm667/mod.rs"
with open(mod_path, "r") as f:
    content = f.read()

content = content.replace(
    "-> HashMap<String, Vec<String>> {",
    "-> HashMap<String, Vec<(String, i32)>> {"
)
content = content.replace(
    "sprite_paths.push(rel_res);",
    "sprite_paths.push((rel_res, frame.duration));"
)
with open(mod_path, "w") as f:
    f.write(content)

# Edit resource_gen.rs
res_path = "rust/src/realm667/resource_gen.rs"
with open(res_path, "r") as f:
    content = f.read()

content = content.replace(
    "&HashMap<String, Vec<String>>",
    "&HashMap<String, Vec<(String, i32)>>"
)

content = content.replace(
    "fn get_direction_sprites(sprites: &[String], dir: usize) -> Vec<String> {",
    "fn get_direction_sprites(sprites: &[(String, i32)], dir: usize) -> Vec<(String, i32)> {"
)

content = content.replace(
    "sprites.iter().filter(|s| {",
    "sprites.iter().filter(|(s, _)| {"
)

# Fix for loop in weapon gen
content = re.sub(
    r"for sprite_rel_path in sprites \{\s*let id = format!\(\"\{\}_ext\", id_counter\);\s*ext_resources\.push\(format!\(\"\[ext_resource type=\\\"Texture2D\\\" path=\\\"\{\}\\\" id=\\\"\{\}\\\"\]\", sprite_rel_path, id\)\);\s*frames\.push\(format!\(\"\\{\\n\\\"duration\\\": 1\.0,\\n\\\"texture\\\": ExtResource\(\\\"\{\}\\\"\)\\n\\}\", id\)\);\s*id_counter \+= 1;\s*\}",
    r"""for (sprite_rel_path, duration) in sprites {
                let id = format!("{}_ext", id_counter);
                ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, id));
                let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                id_counter += 1;
            }""",
    content
)

# Fix speed in weapon gen
content = content.replace("\"speed\": 5.0", "\"speed\": 35.0")

# Fix for loop in enemy gen (walk, attack)
content = re.sub(
    r"for sprite_rel_path in active_sprites \{\s*let id = texture_to_id\.entry\(sprite_rel_path\.clone\(\)\)\.or_insert_with\(\|\| \{\s*let new_id = format!\(\"\{\}_ext\", id_counter\);\s*ext_resources\.push\(format!\(\"\[ext_resource type=\\\"Texture2D\\\" path=\\\"\{\}\\\" id=\\\"\{\}\\\"\]\", sprite_rel_path, new_id\)\);\s*id_counter \+= 1;\s*new_id\s*\}\);\s*frames\.push\(format!\(\"\\{\\n\\\"duration\\\": 1\.0,\\n\\\"texture\\\": ExtResource\(\\\"\{\}\\\"\)\\n\\}\", id\)\);\s*\}",
    r"""for (sprite_rel_path, duration) in active_sprites {
                        let id = texture_to_id.entry(sprite_rel_path.clone()).or_insert_with(|| {
                            let new_id = format!("{}_ext", id_counter);
                            ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, new_id));
                            id_counter += 1;
                            new_id
                        });
                        let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                        frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                    }""",
    content
)

# Fix other enemy loops (pain, death, xdeath, generic)
content = re.sub(
    r"for sprite_rel_path in sprites \{\s*let id = texture_to_id\.entry\(sprite_rel_path\.clone\(\)\)\.or_insert_with\(\|\| \{\s*let new_id = format!\(\"\{\}_ext\", id_counter\);\s*ext_resources\.push\(format!\(\"\[ext_resource type=\\\"Texture2D\\\" path=\\\"\{\}\\\" id=\\\"\{\}\\\"\]\", sprite_rel_path, new_id\)\);\s*id_counter \+= 1;\s*new_id\s*\}\);\s*frames\.push\(format!\(\"\\{\\n\\\"duration\\\": 1\.0,\\n\\\"texture\\\": ExtResource\(\\\"\{\}\\\"\)\\n\\}\", id\)\);\s*\}",
    r"""for (sprite_rel_path, duration) in sprites {
                    let id = texture_to_id.entry(sprite_rel_path.clone()).or_insert_with(|| {
                        let new_id = format!("{}_ext", id_counter);
                        ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, new_id));
                        id_counter += 1;
                        new_id
                    });
                    let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                    frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                 }""",
    content
)

# Fix speed in enemy gen
content = content.replace("\"speed\": 5.0", "\"speed\": 35.0")

# Fix projectile gen
content = re.sub(
    r"for sprite_rel_path in sprites \{\s*let id = format!\(\"\{\}_ext\", id_counter\);\s*ext_resources\.push\(format!\(\"\[ext_resource type=\\\"Texture2D\\\" path=\\\"\{\}\\\" id=\\\"\{\}\\\"\]\", sprite_rel_path, id\)\);\s*frames\.push\(format!\(\"\\{\\n\\\"duration\\\": 1\.0,\\n\\\"texture\\\": ExtResource\(\\\"\{\}\\\"\)\\n\\}\", id\)\);\s*id_counter \+= 1;\s*\}",
    r"""for (sprite_rel_path, duration) in sprites {
                let id = format!("{}_ext", id_counter);
                ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, id));
                let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                id_counter += 1;
            }""",
    content
)

content = content.replace("\"speed\": 8.0", "\"speed\": 35.0")

with open(res_path, "w") as f:
    f.write(content)

print("Patch applied successfully.")
