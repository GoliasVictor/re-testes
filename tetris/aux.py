
lenghts = [[[1,2,4]]]
k = 5

for i in range(5):
	new_seqs = []
	for seq in lenghts[-1]:
		n = seq[-1]
		print(n)
		if n % 2 == 0:
			new_seqs.append(seq + [n-1])
		new_seqs.append(seq + [n*2] )
	lenghts.append(new_seqs)
i = 0
for lenght in lenghts:
	i+= 1
	print("tamanho : ", i)
	for seq in lenght:
		print("- ", ".".join(map(str, seq[3:])))
	
