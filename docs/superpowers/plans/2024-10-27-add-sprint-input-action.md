# Add Sprint Input Action Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a 'sprint' input action to the Godot project configuration.

**Architecture:** Modify `project.godot` to include a new input action mapped to the Shift key.

**Tech Stack:** Godot 4.x, Git

---

### Task 1: Add 'sprint' action to project.godot

**Files:**
- Modify: `/Users/aramsamifanni/new-game/project.godot`

- [ ] **Step 1: Append 'sprint' action to the [input] section**

```ini
sprint={
"deadzone": 0.5,
"events": [Object(InputEventKey,"resource_local_to_scene":false,"resource_name":"","device":-1,"window_id":0,"alt_pressed":false,"shift_pressed":true,"ctrl_pressed":false,"meta_pressed":false,"pressed":false,"keycode":0,"physical_keycode":4194325,"key_label":0,"unicode":0,"location":0,"echo":false,"script":null)
]
}
```

- [ ] **Step 2: Verify the file is still valid**
(Since we can't easily run Godot to verify, we'll manually check the file content and structure)

- [ ] **Step 3: Commit the change**

```bash
git add project.godot
git commit -m "feat: add sprint input action"
```
