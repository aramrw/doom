import re

with open("rust/src/realm667/resource_gen.rs", "r") as f:
    text = f.read()

# Replace simple loop
text = text.replace(
    "for sprite_rel_path in sprites {",
    "for (sprite_rel_path, duration) in sprites {"
)

text = text.replace(
    "for sprite_rel_path in active_sprites {",
    "for (sprite_rel_path, duration) in active_sprites {"
)

# Replace the frame pushing logic
text = re.sub(
    r'frames\.push\(format!\(\"\\\{\\n\\\"duration\\\": 1\.0,\\n\\\"texture\\\": ExtResource\(\\\"\{\}\\\"\)\\n\\\}\", id\)\);',
    r'let dur = if *duration < 0 { 1.0 } else { *duration as f32 };\n                frames.push(format!("{{\\n\\"duration\\": {},\\n\\"texture\\": ExtResource(\\"{}\\")\\n}}", dur, id));',
    text
)

with open("rust/src/realm667/resource_gen.rs", "w") as f:
    f.write(text)

