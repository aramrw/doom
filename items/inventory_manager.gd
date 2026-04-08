extends Node
class_name InventoryManager

signal inventory_changed
signal item_activated(item: InventoryItemData)

var items: Array[InventoryItemData] = []
var uses: Dictionary = {} # item_name -> total uses/count
var active_index: int = -1

func add_item(item: InventoryItemData, amount: int = -1) -> bool:
	# If amount is -1, use the item's default_uses
	var add_amount = amount if amount != -1 else item.default_uses
	
	if item.item_name in uses:
		if uses[item.item_name] < item.max_stack:
			uses[item.item_name] = min(uses[item.item_name] + add_amount, item.max_stack)
			inventory_changed.emit()
			return true
		else:
			return false # Already at max stack
	else:
		items.append(item)
		uses[item.item_name] = add_amount
		if active_index == -1:
			active_index = 0
		inventory_changed.emit()
		return true

func consume_item(item_name: String, amount: int = 1) -> bool:
	if item_name in uses and uses[item_name] >= amount:
		uses[item_name] -= amount
		if uses[item_name] <= 0:
			_remove_from_list(item_name)
		inventory_changed.emit()
		return true
	return false

func _remove_from_list(item_name: String):
	uses.erase(item_name)
	var to_remove = -1
	for i in range(items.size()):
		if items[i].item_name == item_name:
			to_remove = i
			break
	if to_remove != -1:
		items.remove_at(to_remove)
		if active_index >= items.size():
			active_index = items.size() - 1
			if active_index < 0 and items.size() > 0:
				active_index = 0

func has_item(item_name: String, amount: int = 1) -> bool:
	return item_name in uses and uses[item_name] >= amount

func get_active_item() -> InventoryItemData:
	if active_index >= 0 and active_index < items.size():
		return items[active_index]
	return null

func cycle_next():
	if items.size() > 1:
		active_index = (active_index + 1) % items.size()
		inventory_changed.emit()

func cycle_prev():
	if items.size() > 1:
		active_index = (active_index - 1 + items.size()) % items.size()
		inventory_changed.emit()

func use_active_item(player: Node):
	var item = get_active_item()
	if item:
		item.use(player)
		if item.is_consumable:
			consume_item(item.item_name, 1)
