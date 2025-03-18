---@meta

__engine = {}
__engine.gameobject = {}

--- Gets a parameter of the attached object.
--- @param param string
--- @return any
function __engine.gameobject:Get(param)
end

--- Sets a parameter of the attached object.
--- @param param string
--- @param value any
--- @return any
function __engine.gameobject:Set(param, value)
end

--- Resets a parameter of the attached object.
--- @param param string
--- @param value any
--- @return any
function __engine.gameobject:Reset(param, value)
end

---@class log
Log = {}
function Log.Trace(...) end
function Log.Debug(...) end
function Log.Info(...) end
function Log.Warn(...) end
function Log.eErr(...) end