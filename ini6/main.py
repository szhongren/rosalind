words = open("rosalind_ini6.txt", 'r').readline().strip().split(' ')

counts = {}
for word in words:
    if word in counts:
        counts[word] += 1
    else:
        counts[word] = 1

for word, freq in counts.items():
    print(word, freq)
