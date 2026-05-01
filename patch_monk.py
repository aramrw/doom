import re

with open('npcs/shotgun_monk.tscn', 'r') as f:
    content = f.read()

# Add new ext_resources
new_resources = """[ext_resource type="Script" path="res://components/HealthComponent.gd" id="health_comp"]
[ext_resource type="Script" path="res://components/AttackComponent.gd" id="attack_comp"]
[ext_resource type="Script" path="res://components/ChaseComponent.gd" id="chase_comp"]
[ext_resource type="Script" path="res://components/AnimationComponent.gd" id="anim_comp"]
"""

# Put them after the last ext_resource
last_ext_idx = content.rfind('[ext_resource')
end_of_line = content.find('\n', last_ext_idx) + 1
content = content[:end_of_line] + new_resources + content[end_of_line:]

# Add the new components right before NavigationAgent3D or at the end of the node list
# Find a good insertion point. Usually before the first child node.
insertion_point = content.find('[node name="NavigationAgent3D"')
if insertion_point == -1:
    insertion_point = content.find('[node name="CollisionShape3D"')

new_nodes = """[node name="HealthComponent" type="Node" parent="."]
script = ExtResource("health_comp")

[node name="AttackComponent" type="Node" parent="."]
script = ExtResource("attack_comp")

[node name="ChaseComponent" type="Node" parent="."]
script = ExtResource("chase_comp")

[node name="AnimationComponent" type="Node" parent="."]
script = ExtResource("anim_comp")

"""

content = content[:insertion_point] + new_nodes + content[insertion_point:]

with open('npcs/shotgun_monk.tscn', 'w') as f:
    f.write(content)

