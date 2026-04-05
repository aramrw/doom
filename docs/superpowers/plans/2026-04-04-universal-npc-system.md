# Universal NPC System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a flexible NPC system that supports friendly, neutral, and hostile behaviors, including bodyguard following and interactive dialogue.

**Architecture:** `BaseNPC.gd` extends `DoomEnemyBase` with a state machine (IDLE, FOLLOW, COMBAT, TALKING). Dialogue is managed via a modular UI.

**Tech Stack:** Godot 4.x (GDScript)

---

### Task 1: Interaction Setup

**Files:**
- Modify: `project.godot`
- Create: `ui/dialogue_ui.tscn`
- Create: `ui/dialogue_ui.gd`

- [ ] **Step 1: Add `interact` action to Input Map**
Add `interact` with Key `E` to `project.godot`.

- [ ] **Step 2: Create `dialogue_ui.tscn`**
A CanvasLayer with a bottom-aligned Panel, Name Label, and Text Label.

- [ ] **Step 3: Create `dialogue_ui.gd`**

```gdscript
extends CanvasLayer

@onready var name_label = $Panel/NameLabel
@onready var text_label = $Panel/TextLabel

signal dialogue_finished

var current_lines: Array = []
var current_index: int = 0

func _ready():
	hide()

func start_dialogue(npc_name: String, lines: Array):
	current_lines = lines
	current_index = 0
	name_label.text = npc_name
	show()
	display_line()

func display_line():
	if current_index < current_lines.size():
		text_label.text = current_lines[current_index]
	else:
		finish()

func advance():
	current_index += 1
	display_line()

func finish():
	hide()
	dialogue_finished.emit()
```

- [ ] **Step 4: Commit**

```bash
git add project.godot ui/dialogue_ui.tscn ui/dialogue_ui.gd
git commit -m "feat: add interaction input and basic dialogue UI"
```

---

### Task 2: Base NPC Logic

**Files:**
- Create: `npcs/base_npc.gd`

- [ ] **Step 1: Create `BaseNPC.gd`**

```gdscript
extends DoomEnemyBase
class_name BaseNPC

enum NPCState { IDLE, FOLLOW, COMBAT, TALKING }
var npc_state = NPCState.IDLE

@export_group("NPC Behavior")
@export var follows_player: bool = false
@export var attacks_enemies: bool = true
@export var attacks_player: bool = false
@export var become_hostile_on_damage: bool = true

@export_group("Dialogue")
@export var npc_display_name: String = "NPC"
@export var dialogue_lines: Array[String] = ["Hello there!"]

@onready var interaction_zone: Area3D = $InteractionZone

func _ready():
	super._ready()
	add_to_group("NPCs")
	if not attacks_player:
		# If friendly, don't target player by default
		pass

func _physics_process(delta):
	if is_dead:
		super._physics_process(delta)
		return
		
	match npc_state:
		NPCState.IDLE:
			process_idle(delta)
		NPCState.FOLLOW:
			process_follow(delta)
		NPCState.COMBAT:
			super._physics_process(delta)
		NPCState.TALKING:
			process_talking(delta)
	
	# Global target scanning
	check_for_targets()

func process_idle(delta):
	current_anim_state = "walk" # or idle if exists
	velocity = Vector3.ZERO
	move_and_slide()
	if follows_player:
		npc_state = NPCState.FOLLOW

func process_follow(delta):
	var dist = global_position.distance_to(player.global_position)
	if dist > 4.0:
		chase_player()
	else:
		stand_and_stare()

func process_talking(delta):
	velocity = Vector3.ZERO
	move_and_slide()
	stand_and_stare()

func check_for_targets():
	if npc_state == NPCState.TALKING: return
	
	# Find nearest enemy
	var enemies = get_tree().get_nodes_in_group("Enemies")
	var nearest_enemy = null
	var min_dist = detection_range
	
	for enemy in enemies:
		var d = global_position.distance_to(enemy.global_position)
		if d < min_dist:
			min_dist = d
			nearest_enemy = enemy
			
	if nearest_enemy and attacks_enemies:
		npc_state = NPCState.COMBAT
		# We need to tweak DoomEnemyBase to target this enemy instead of just 'player'
	elif attacks_player:
		npc_state = NPCState.COMBAT
	elif follows_player:
		npc_state = NPCState.FOLLOW
	else:
		npc_state = NPCState.IDLE

func interact():
	npc_state = NPCState.TALKING
	# Call global DialogueUI (to be added to main scene or as singleton)
```

- [ ] **Step 2: Commit**

```bash
git add npcs/base_npc.gd
git commit -m "feat: add BaseNPC with state machine and behavior toggles"
```

---

### Task 3: ShotgunMonk Integration

**Files:**
- Create: `npcs/shotgun_monk.tscn`

- [ ] **Step 1: Create `shotgun_monk.tscn`**
Reuse `doom_enemy_template.tscn` structure but use `BaseNPC.gd` and ShotgunMonk sprites.

- [ ] **Step 2: Configure ShotgunMonk**
Set `follows_player = true`, `attacks_enemies = true`, `attacks_player = false`.

- [ ] **Step 3: Commit**

```bash
git add npcs/shotgun_monk.tscn
git commit -m "feat: implement ShotgunMonk bodyguard NPC"
```

---

### Task 4: Verification

- [ ] **Step 1: Verify NPC follows player**
- [ ] **Step 2: Verify NPC attacks enemies**
- [ ] **Step 3: Verify dialogue triggers on interaction**
