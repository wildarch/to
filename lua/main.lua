#!/usr/bin/env lua
local rapidjson = require('rapidjson')
local path = require('pl.path')
__ = require('underscore')

local CONF_PATH = os.getenv("HOME") .. "/.config/to"

local file = io.open(CONF_PATH, 'r')
local parsed = rapidjson.decode(file:read("*all"))
file:close()

local directories = parsed["directories"]

function string:starts_with (s)
    return string.sub(self, 1, string.len(s)) == s
end

function save_settings()
    encoded = rapidjson.encode(parsed, {pretty=true})
    file = io.open(CONF_PATH, 'w')
    file:write(encoded)
    file:close()
end

function add(dir)
    if (dir == nil) then
        dir = path.currentdir()
    end
    table.insert(directories, dir)
    save_settings()
end

function remove(index)
    table.remove(directories, index)
    save_settings()
end

function dirs()
    for i, dir in ipairs(directories) do
        print(i, dir)
    end
end

function dir_iter(query)
    local results = {}
    for i, base in ipairs(directories) do
        for dir in path.dir(base) do
            full = path.join(base, dir)
            if path.isdir(full) and dir:starts_with(query) then
                table.insert(results, {
                    base=base,
                    dir=dir,
                    full=full
                })
            end
        end
    end
    -- Sort by length
    table.sort(results, function(a,b) return string.len(a.dir) < string.len(b.dir) end)
    return results
end

function go(query)
    first = dir_iter(query)[1]
    if first then
        print(first.full)
    else
        print("No results found!")
    end
end

function list(query)
    if query == nil then
        query = ""
    end
    for i, res in ipairs(dir_iter(query)) do
        print(res.dir)
    end
end

function version()
    print("To: Lua version")
end

args = {
    ["add"] = add,
    ["remove"] = remove,
    ["dirs"] = dirs,
    ["go"] = go,
    ["list"] = list,
    ["version"] = version
}
args[arg[1]](arg[2])

