# Shotgun Monk Dialogue Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add dialogue lines for the Shotgun Monk to build boss fight tension, using existing project patterns in `npcs/shotgun_monk.tscn`.

**Architecture:** Use Godot Resource/SubResource system (`DialogueLine` and `DialogueChoice` resources) as defined in `npcs/shotgun_monk.tscn`.

**Tech Stack:** Godot 4.x, GDScript.

---

### Task 1: Update Shotgun Monk Dialogue Resources

**Files:**
- Modify: `npcs/shotgun_monk.tscn`

- [ ] **Step 1: Create DialogueLine resources for tension build-up**
Add new `DialogueLine` subresources to `npcs/shotgun_monk.tscn`.

```gdscript
[sub_resource type="DialogueLine" id="DialogueLine_tension1"]
text = "These lands have forgotten the scent of incense. Only the stench of sulfur remains."
# Need to assign an audio resource if required by the system, 
# for now leaving audio blank if the resource system allows it.

[sub_resource type="DialogueLine" id="DialogueLine_tension2"]
text = "My shotgun is consecrated, but even lead cannot kill a ghost that refuses to die."
```

- [ ] **Step 2: Update Choice nodes or link them**
Modify the existing dialogue flow in `npcs/shotgun_monk.tscn` to transition into these new lines when conditions are met or by adding them as options.

- [ ] **Step 3: Commit**

```bash
git add npcs/shotgun_monk.tscn
git commit -m "feat: add tension-building dialogue lines to Shotgun Monk"
```

---

Plan complete and saved to `docs/superpowers/plans/2026-04-29-shotgun-monk-dialogue.md`. Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
