


with open("1.txt") as f:
    now = []
    all = []
    for l in f.readlines():
        l = l.strip()
        if l == "":
            all.append(sum(now))
            now = []
        else:
            now.append(int(l))
    print (max(all))
    all.sort()
    print (sum(all[-3:]))
