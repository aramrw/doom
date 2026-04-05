# Sprinting System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a sprinting mechanic to the player that increases movement speed when holding the Shift key and adjusts the FOV/weapon bob for better feedback.

**Architecture:** 
- Add `is_sprinting` state to `player.gd`.
- Implement speed multiplier and FOV interpolation.
- Add "sprint" action to `project.godot` input map.
- Integrate with existing head/weapon bob systems.

**Tech Stack:** Godot 4.6 (GDScript)

---

### Task 1: Add Sprint Input Action

**Files:**
- Modify: `project.godot`

- [ ] **Step 1: Add 'sprint' action to project.godot**

```ini
sprint={
"deadzone": 0.5,
"events": [Object(InputEventKey,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":true,"ctrl_pressed":false,"meta_pressed":false,"pressed":false,"keycode":0,"physical_keycode":4194325,"key_label":0,"unicode":0,"location":0,"echo":false,"script":null)
]
}
```

- [ ] **Step 2: Commit input configuration**

```bash
git add project.godot
git commit -m "feat: add sprint input action"
```

---

### Task 2: Implement Sprint Logic in Player

**Files:**
- Modify: `player/player.gd`

- [ ] **Step 1: Define sprint constants and variables**

Add near existing movement variables:
```gdscript
@export var SPRINT_SPEED = 7.0
@export var NORMAL_SPEED = 5.0 # Update SPEED to this default
@export var SPRINT_FOV_MOD = 1.1 # 10% increase
@export var FOV_CHANGE_SPEED = 5.0

var is_sprinting = false
@onready var default_fov = camera.fov
```

- [ ] **Step 2: Update _physics_process to handle sprinting speed and FOV**

```gdscript
# Inside _physics_process...

# 1. Update sprinting state (only if moving forward and not in dialogue)
var is_moving_forward = Input.is_action_pressed("move_forward")
is_sprinting = Input.is_action_pressed("sprint") and is_moving_forward and not in_dialogue

# 2. Set current speed
SPEED = SPRINT_SPEED if is_sprinting else NORMAL_SPEED

# 3. Handle FOV Change
var target_fov = default_fov * (SPRINT_FOV_MOD if is_sprinting else 1.0)
camera.fov = lerp(camera.fov, target_fov, delta * FOV_CHANGE_SPEED)
```

- [ ] **Step 3: Adjust Head Bob and Weapon Bob frequency**

```gdscript
# In _physics_process where tbob is calculated:
var bob_multiplier = 1.5 if is_sprinting else 1.0
tbob += delta * body.velocity.length() * float(body.is_on_floor()) * bob_multiplier
```

- [ ] **Step 4: Commit player sprint implementation**

```bash
git add player/player.gd
git commit -m "feat: implement sprinting speed and FOV effects"
```

---

### Task 3: Verification

- [ ] **Step 1: Verify in-game**
- Confirm Shift key increases movement speed.
- Confirm FOV zooms out slightly when sprinting.
- Confirm head bob and weapon bob speed up.
- Confirm sprinting is disabled during dialogue.
- Confirm sprinting only works when moving forward (optional/recommended feel).
