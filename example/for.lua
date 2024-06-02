for i = 1, 10, 1 do
    print(i)
end

local t = { 1, 3, 5, 6, 7, 8, 9 }
for index, value in ipairs(t) do
    print("index=" .. index .. ",value=" .. value .. "\n")
end
