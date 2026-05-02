# Slitherfist Enemy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Integrate the Slitherfist as a fully functional enemy with health, attack, and sounds.

**Architecture:** Use existing components (`HealthComponent`, `AttackComponent`) and follow the patterns found in `enemies/slitherfist/Slitherfist.gd` and `enemies/slitherfist/Slitherfist.tscn`. 

**Tech Stack:** Godot 4.x, GDScript, PhysicsBody3D, Resource-based Components.

---

### Task 1: Initialize Slitherfist Scene Assets and Components
**Files:**
- Modify: `enemies/slitherfist/Slitherfist.tscn`

- [ ] **Step 1: Set unique IDs and fix component references in `Slitherfist.tscn`**
Update `HealthComponent` and `AttackComponent` references and ensure `unique_id`s are valid and consistent.

- [ ] **Step 2: Assign proper EnemySounds resource**
Update `EnemySounds` node in `Slitherfist.tscn` to use the appropriate sounds folder (if available) or create a placeholder.

### Task 2: Configure Slitherfist Logic
**Files:**
- Modify: `enemies/slitherfist/Slitherfist.gd`

- [ ] **Step 1: Verify and adjust constants**
Ensure `speed`, `meleerange`, and `detection_range` are tuned for Slitherfist.

- [ ] **Step 2: Update signal connections**
Verify that all `HealthComponent` and `AttackComponent` signals are connected properly to the script.

### Task 3: Integration and Testing
**Files:**
- Modify: `node_3d.tscn` (or a test scene)

- [ ] **Step 1: Add Slitherfist to a test scene**
Place an instance of `Slitherfist.tscn` into a test scene.

- [ ] **Step 2: Test basic functionality**
Verify the enemy detects the player, chases, and triggers attacks.

- [ ] **Step 3: Test damage and death**
Verify the Slitherfist can be damaged, plays the hit effect, and dies.
