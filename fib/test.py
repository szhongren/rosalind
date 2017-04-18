
def fib_rabbits(n,k):
    '''Returns the number of rabbits present after n generations with litters of k pairs.'''
    rabbits = [0,1]
    for i in range(n-1):
        rabbits[i % 2] = rabbits[(i-1) % 2] + k*rabbits[i % 2]

    return rabbits[n % 2]


'''Main call. Parses, runs, and saves problem specific data.'''
# Read the input data.
with open('datasets/rosalind_fib.txt') as input_data:
    n, k = map(int, input_data.read().strip().split())

# Get the number of rabbits.
rabbits = str(fib_rabbits(n,k))

# Print and save the answer.
print(rabbits)


