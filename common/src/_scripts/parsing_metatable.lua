return function(this, key)
    if not type(key) == "table" then
        return function(r)
            print("Parsing Failure")
            return r
        end
    end
    for k, v in pairs(this) do
        if type(k) == "string" then
            k = table.pack(string.byte(k, 1, string.len(k)))
        end
        local match = true
        for kk, kv in pairs(key) do
            match = match and k[kk] == kv
            if not match then break end
        end
        if match then return v end
    end
end
