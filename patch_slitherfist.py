import re

with open('enemies/slitherfist/Slitherfist.tscn', 'r') as f:
    content = f.read()

# Remove old ext_resources
content = re.sub(r'\[ext_resource type="Script" uid="uid://state_machine_script".*?\n', '', content)
content = re.sub(r'\[ext_resource type="Script" uid="uid://idle_state_script".*?\n', '', content)
content = re.sub(r'\[ext_resource type="Script" uid="uid://chase_state_script".*?\n', '', content)
content = re.sub(r'\[ext_resource type="Script" uid="uid://attack_state_script".*?\n', '', content)

# Add new ext_resources at the top after the first few
new_resources = """[ext_resource type="Script" path="res://components/ChaseComponent.gd" id="chase_comp"]
[ext_resource type="Script" path="res://components/AnimationComponent.gd" id="anim_comp"]
"""

# Just find the last ext_resource and put it after
last_ext_idx = content.rfind('[ext_resource')
end_of_line = content.find('\n', last_ext_idx) + 1
content = content[:end_of_line] + new_resources + content[end_of_line:]

# Remove StateMachine nodes block
state_machine_pattern = r'\[node name="StateMachine" type="Node" parent="\." unique_id=1221744111\]\nscript = ExtResource\("5_state_machine"\)\ninitial_state = NodePath\("Idle"\)\n\n\[node name="Idle" type="Node" parent="StateMachine"\]\nscript = ExtResource\("6_idle_state"\)\n\n\[node name="Chase" type="Node" parent="StateMachine"\]\nscript = ExtResource\("7_chase_state"\)\n\n\[node name="Attack" type="Node" parent="StateMachine"\]\nscript = ExtResource\("8_attack_state"\)\nattack_cooldown = 1\.0\n\n'
content = re.sub(state_machine_pattern, '', content)

# Add the new components right before NavigationAgent3D
new_nodes = """[node name="ChaseComponent" type="Node" parent="."]
script = ExtResource("chase_comp")

[node name="AnimationComponent" type="Node" parent="."]
script = ExtResource("anim_comp")

"""

content = content.replace('[node name="NavigationAgent3D"', new_nodes + '[node name="NavigationAgent3D"')

with open('enemies/slitherfist/Slitherfist.tscn', 'w') as f:
    f.write(content)

