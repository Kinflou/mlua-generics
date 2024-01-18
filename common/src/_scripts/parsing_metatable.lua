return function(this, key)
    print("Trying to index:")
    print("this: "..tostring(this))

    print("key: "..tostring(key))

    args = key

    if type(key) == "userdata" then
        local type, _ = pcall(function() return key:_type() end)

        if not type then
            error("UserData '"..tostring(key).."' doesn't have a _type method, maybe forgotten implementation?")
        end

        args = {key:_type()}
    end

    if not type(key) == "table" then
        args = {key}
    end

    for k, v in pairs(this) do
        print("k type: "..tostring(type(k)))
        print("k: "..k.." | v: "..tostring(v))

        if type(k) == "string" then
            k = table.pack(string.byte(k, 1, string.len(k)))
        end

        local match = true
        for kk, _ in pairs(args) do
            print("kk type: "..tostring(type(kk)))
            print("kk: "..k)

            print("k == kk: "..tostring(k==kk))
            match = match and k == kk
            if not match then break end
        end

        --[[
        for kk, kv in pairs(args) do
            print("kk: "..k.."| kv: "..tostring(v))

            match = match and k[kk] == kv
            if not match then break end
        end
        --]]

        if match then return v end
    end
end
