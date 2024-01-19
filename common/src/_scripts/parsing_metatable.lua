return function(this, key)
    print("key: "..tostring(key))
    print("key type: "..tostring(type(key)))

    local args = key

    if type(key) == "userdata" then
        local type, _ = pcall(function() return key:_type() end)

        if not type then
            error("UserData '"..tostring(key).."' doesn't have a _type method, maybe forgotten implementation?")
        end

        args = {key:_type()}
    elseif type(key) ~= "table" then
        args = {key}  -- Issue is here, it needs to go into this table to match down low
    end

    for k, v in pairs(this) do
        local match = true
        for _, kv in pairs(args) do
            match = match and math.abs(k - kv) < 0.01
            if not match then break end
        end

        if match then return v end
    end
end
