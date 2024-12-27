---@meta

---@class Script
---@field name string
---@field class string
---@field avatar_id integer
---@field placement_guid string | nil
---@field template_guid string | nil
SCRIPT = {}

--- Gets a parameter of the attached object.
--- @param param string
--- @return any
function SCRIPT:Get(param)
end

--- Sets a parameter of the attached object.
--- @param param string
--- @param value any
--- @return any
function SCRIPT:Set(param, value)
end

--- Resets a parameter of the attached object.
--- @param param string
--- @param value any
--- @return any
function SCRIPT:Reset(param, value)
end

---@class Player: Script 
PLAYER = {}

---@class Npc: Script
NPC = {}

---@class log
Log = {}
function Log.Trace(...) end
function Log.Debug(...) end
function Log.Info(...) end
function Log.Warn(...) end
function Log.eErr(...) end