import re

with open('node_3d.tscn', 'r') as f:
    content = f.read()

# Find ext_resource ID for the missing scenes
imp_match = re.search(r'\[ext_resource type="PackedScene" uid=".*?" path="res://enemies/dark_gargoyle/godot_data/hereticdarkimp.tscn" id="(.*?)"\]', content)
slither_match = re.search(r'\[ext_resource type="PackedScene" uid=".*?" path="res://enemies/slitherfist/Slitherfist.tscn" id="(.*?)"\]', content)

if imp_match:
    id = imp_match.group(1)
    # Remove the ext_resource
    content = re.sub(r'\[ext_resource type="PackedScene" uid=".*?" path="res://enemies/dark_gargoyle/godot_data/hereticdarkimp.tscn" id="' + id + r'"\]\n', '', content)
    # Remove nodes that use it
    content = re.sub(r'\[node name=".*?" parent="\." instance=ExtResource\("' + id + r'"\)\]\n(?:.*?\n)*?\n', '', content)

# Slitherfist was actually fixed, but the error log said it failed to load.
# Let's see if the Slitherfist.tscn is actually valid now.
# If not, we might need to fix it more.

with open('node_3d.tscn', 'w') as f:
    f.write(content)

