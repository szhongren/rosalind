f = open("rosalind_ini5.txt", 'r')
out = open("new.txt", 'w')
for i, line in enumerate(f):
    if not i % 2:
        continue
    out.write(line)
