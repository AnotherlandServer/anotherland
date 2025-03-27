---@meta

__engine = {}

----
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

----

__engine.inventory = {}

function __engine.inventory:AddItem()
end

function __engine.inventory:ApplyClassPreset(name)
end

function __engine.inventory:RemoveItem(id)
end

---
---@class Quaternion
---@field x number
---@field y number
---@field z number
---@field w number
local Quaternion = {}

--- Checks if the quaternion is approximately equal to another quaternion within a tolerance.
--- @param rhs Quaternion
--- @param max_abs_diff number
--- @return boolean
function Quaternion:AbsDiffEq(rhs, max_abs_diff) end

--- Computes the angle between this quaternion and another.
--- @param other Quaternion
--- @return number
function Quaternion:AngleBetween(other) end

--- Computes the conjugate of the quaternion.
--- @return Quaternion
function Quaternion:Conjugate() end

--- Computes the dot product with another quaternion.
--- @param other Quaternion
--- @return number
function Quaternion:Dot(other) end

--- Computes the inverse of the quaternion.
--- @return Quaternion
function Quaternion:Inverse() end

--- Checks if all components are finite.
--- @return boolean
function Quaternion:IsFinite() end

--- Checks if any component is NaN.
--- @return boolean
function Quaternion:IsNan() end

--- Checks if the quaternion is near the identity quaternion.
--- @return boolean
function Quaternion:IsNearIdentity() end

--- Checks if the quaternion is normalized.
--- @return boolean
function Quaternion:IsNormalized() end

--- Computes the length of the quaternion.
--- @return number
function Quaternion:Length() end

--- Computes the reciprocal of the length.
--- @return number
function Quaternion:LengthRecip() end

--- Computes the squared length of the quaternion.
--- @return number
function Quaternion:LengthSquared() end

--- Linearly interpolates between this quaternion and another.
--- @param rhs Quaternion
--- @param t number
--- @return Quaternion
function Quaternion:Lerp(rhs, t) end

--- Multiplies this quaternion with another quaternion.
--- @param other Quaternion
--- @return Quaternion
function Quaternion:MulQuat(other) end

--- Rotates a vector by this quaternion.
--- @param vec Vector
--- @return Vector
function Quaternion:MulVec(vec) end

--- Normalizes the quaternion.
--- @return Quaternion
function Quaternion:Normalize() end

--- Rotates this quaternion towards another quaternion by a maximum angle.
--- @param to Quaternion
--- @param max_angle number
--- @return Quaternion
function Quaternion:RotateTowards(to, max_angle) end

--- Spherically interpolates between this quaternion and another.
--- @param rhs Quaternion
--- @param t number
--- @return Quaternion
function Quaternion:Slerp(rhs, t) end

--- Creates a quaternion from an axis and angle.
--- @param axis Vector
--- @param angle number
--- @return Quaternion
function Quaternion:FromAxisAngle(axis, angle) end

--- Creates a quaternion from Euler angles.
--- @param x number
--- @param y number
--- @param z number
--- @return Quaternion
function Quaternion:FromEuler(x, y, z) end

--- Creates a quaternion from a rotation arc.
--- @param from Vector
--- @param to Vector
--- @return Quaternion
function Quaternion:FromRotationArc(from, to) end

--- Creates a quaternion from a rotation arc, allowing colinear vectors.
--- @param from Vector
--- @param to Vector
--- @return Quaternion
function Quaternion:FromRotationArcColinear(from, to) end

--- Creates a quaternion from a rotation around the X axis.
--- @param angle number
--- @return Quaternion
function Quaternion:FromRotationX(angle) end

--- Creates a quaternion from a rotation around the Y axis.
--- @param angle number
--- @return Quaternion
function Quaternion:FromRotationY(angle) end

--- Creates a quaternion from a rotation around the Z axis.
--- @param angle number
--- @return Quaternion
function Quaternion:FromRotationZ(angle) end

--- Creates a quaternion from scaled axis representation.
--- @param axis Vector
--- @return Quaternion
function Quaternion:FromScaledAxis(axis) end

--- Creates a quaternion from its components.
--- @param x number
--- @param y number
--- @param z number
--- @param w number
--- @return Quaternion
function Quaternion:FromXYZW(x, y, z, w) end

---
---@class Vector
---@field x number
---@field y number
---@field z number
local Vector = {}

--- Computes the absolute value of each component.
--- @return Vector
function Vector:Abs() end

--- Checks if the vector is approximately equal to another vector within a tolerance.
--- @param rhs Vector
--- @param max_abs_diff number
--- @return boolean
function Vector:AbsDiffEq(rhs, max_abs_diff) end

--- Computes the angle between this vector and another.
--- @param other Vector
--- @return number
function Vector:AngleBetween(other) end

--- Finds any orthogonal vector.
--- @return Vector
function Vector:AnyOrthogonalVector() end

--- Finds any orthonormal pair of vectors.
--- @return Vector, Vector
function Vector:AnyOrthonormalPair() end

--- Finds any orthonormal vector.
--- @return Vector
function Vector:AnyOrthonormalVector() end

--- Ceils the vector components.
--- @return Vector
function Vector:Ceil() end

--- Clamps the vector components between two vectors.
--- @param min Vector
--- @param max Vector
--- @return Vector
function Vector:Clamp(min, max) end

--- Clamps the vector length between a minimum and maximum.
--- @param min number
--- @param max number
--- @return Vector
function Vector:ClampLength(min, max) end

--- Clamps the vector length to a maximum.
--- @param max number
--- @return Vector
function Vector:ClampLengthMax(max) end

--- Clamps the vector length to a minimum.
--- @param min number
--- @return Vector
function Vector:ClampLengthMin(min) end

--- Copies the sign of each component from another vector.
--- @param rhs Vector
--- @return Vector
function Vector:Copysign(rhs) end

--- Computes the cross product with another vector.
--- @param rhs Vector
--- @return Vector
function Vector:Cross(rhs) end

--- Computes the distance to another vector.
--- @param other Vector
--- @return number
function Vector:Distance(other) end

--- Computes the squared distance to another vector.
--- @param other Vector
--- @return number
function Vector:DistanceSquared(other) end

--- Divides the vector components using Euclidean division.
--- @param rhs Vector
--- @return Vector
function Vector:DivEuclid(rhs) end

--- Computes the dot product with another vector.
--- @param rhs Vector
--- @return number
function Vector:Dot(rhs) end

--- Computes the dot product and returns it as a vector.
--- @param rhs Vector
--- @return Vector
function Vector:DotIntoVec(rhs) end

--- Computes the product of all components.
--- @return number
function Vector:ElementProduct() end

--- Computes the sum of all components.
--- @return number
function Vector:ElementSum() end

--- Computes the exponential of each component.
--- @return Vector
function Vector:Exp() end

--- Extends the vector to a 4D vector with a w component.
--- @param w number
--- @return Vector
function Vector:Extend(w) end

--- Floors the vector components.
--- @return Vector
function Vector:Floor() end

--- Computes the fractional part of each component.
--- @return Vector
function Vector:Fract() end

--- Computes the fractional part of each component (GLSL style).
--- @return Vector
function Vector:FractGl() end

--- Checks if all components are finite.
--- @return boolean
function Vector:IsFinite() end

--- Checks if any component is NaN.
--- @return boolean
function Vector:IsNan() end

--- Checks if the vector is normalized.
--- @return boolean
function Vector:IsNormalized() end

--- Computes the length of the vector.
--- @return number
function Vector:Length() end

--- Computes the reciprocal of the length.
--- @return number
function Vector:LengthRecip() end

--- Computes the squared length of the vector.
--- @return number
function Vector:LengthSquared() end

--- Linearly interpolates between this vector and another.
--- @param rhs Vector
--- @param t number
--- @return Vector
function Vector:Lerp(rhs, t) end

--- Computes the maximum of each component with another vector.
--- @param rhs Vector
--- @return Vector
function Vector:Max(rhs) end

--- Finds the maximum component.
--- @return number
function Vector:MaxElement() end

--- Computes the midpoint between this vector and another.
--- @param rhs Vector
--- @return Vector
function Vector:Midpoint(rhs) end

--- Computes the minimum of each component with another vector.
--- @param rhs Vector
--- @return Vector
function Vector:Min(rhs) end

--- Finds the minimum component.
--- @return number
function Vector:MinElement() end

--- Moves the vector towards a target vector by a maximum distance.
--- @param target Vector
--- @param max_distance number
--- @return Vector
function Vector:MoveTowards(target, max_distance) end

--- Creates a new vector.
--- @param x number
--- @param y number
--- @param z number
--- @return Vector
function Vector.New(x, y, z) end

--- Normalizes the vector.
--- @return Vector
function Vector:Normalize() end

--- Normalizes the vector or returns a default vector if the length is zero.
--- @param default Vector
--- @return Vector
function Vector:NormalizeOr(default) end

--- Normalizes the vector or returns a zero vector if the length is zero.
--- @return Vector
function Vector:NormalizeOrZero() end

--- Raises each component to a power.
--- @param n number
--- @return Vector
function Vector:Powf(n) end

--- Projects the vector onto another vector.
--- @param other Vector
--- @return Vector
function Vector:ProjectOnto(other) end

--- Projects the vector onto a normalized vector.
--- @param other Vector
--- @return Vector
function Vector:ProjectOntoNormalized(other) end

--- Computes the reciprocal of each component.
--- @return Vector
function Vector:Recip() end

--- Reflects the vector across a normal.
--- @param normal Vector
--- @return Vector
function Vector:Reflect(normal) end

--- Refracts the vector through a surface with a given refraction index.
--- @param normal Vector
--- @param eta number
--- @return Vector
function Vector:Refract(normal, eta) end

--- Rejects the vector from a normal.
--- @param normal Vector
--- @return Vector
function Vector:RejectFrom(normal) end

--- Rejects the vector from a normalized normal.
--- @param normal Vector
--- @return Vector
function Vector:RejectFromNormalized(normal) end

--- Computes the remainder of the vector components using Euclidean division.
--- @param rhs Vector
--- @return Vector
function Vector:RemEuclid(rhs) end

--- Rounds the vector components to the nearest integer.
--- @return Vector
function Vector:Round() end

--- Returns the sign of each component of the vector.
--- @return Vector
function Vector:Signum() end

--- Truncates the vector components to their integer parts.
--- @return Vector
function Vector:Trunc() end

--- Returns a copy of the vector with the x component replaced.
--- @param x number
--- @return Vector
function Vector:WithX(x) end

--- Returns a copy of the vector with the y component replaced.
--- @param y number
--- @return Vector
function Vector:WithY(y) end

--- Returns a copy of the vector with the z component replaced.
--- @param z number
--- @return Vector
function Vector:WithZ(z) end

---
---@class AbilityRequest
---@field ability_id string
---@field reference_id? string
---@field target? Player|NpcOtherland
---@field prediction_id integer
---@field toggle_mode? integer
---@field combo_stage_id? integer
---@field target_rogation? Quaternion
---@field item? EdnaFunction

---
---@class ContentRef
---@field class string
---@field id string

---comment
---@param id string
---@return EdnaFunction
function __engine.inventory:GetItem(id)
end

---@class log
Log = {}
function Log.Trace(...) end
function Log.Debug(...) end
function Log.Info(...) end
function Log.Warn(...) end
function Log.eErr(...) end