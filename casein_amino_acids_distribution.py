s = """
Results of amino acid determination on casein
           , values expressed in gram per 100 gram of protein, #, #
constituent, present work, gordon et. al. (1949, 1950), sundararajan et. al. (1957)
Total Nitrogen, 15.02, 15.63, 15.65
Glycine, 2.0, 2.0, 1.9
Alanine, 3.4, 3.2, 3.38
Valine, 7.2, 7.2, 6.94
Leucine, 9.7, 9.2, 9.54
Isoleucine, 6.1, 6.1, 6.13
Proline, 10.9, 10.6, 11.72
Phenylalanine, 4.9, 5.0, 4.98
Tyrosine, 5.2, 6.3, 6.32
Serine, 5.0, 6.3, 6.27
Threonine, 4.3, 4.9, 5.28
Cystine, 0.31, 0.34, 0.35
Methionine, 2.9, 2.8, 2.93
Arginine, 3.8, 4.1, 3.92
Histidine, 2.9, 3.1, 3.03
Lysine, 7.6, 8.2, 8.18
Aspartic Acid, 6.5, 7.1, 7.66
Glutamic Acid, 21.4, 22.4, 21.62
"""
s = s.strip()
title = s.split('\n')[0]
rest = s.split('\n')[1:]
print(title)
print(rest)
title = title.strip().capitalize()
total_nitrogen = rest[2]
print(total_nitrogen)
list = rest[3:]
print(title)
print(list)
s1 = 0.0
s2 = 0.0
s3 = 0.0
for line in list:
	u = line.split(',')
	amino = u[0].strip().lower()
	x1, x2, x3 = (u[1], u[2], u[3])
	assert(len(u) == 4)
	print(x1)
	print(x2)
	print(x3)
	x1 = float(x1)
	x2 = float(x2)
	x3 = float(x3)
	s1 = s1 + x1
	s2 = s2 + x2
	s3 = s3 + x3
	print(amino)

print(s1)
print(s2)
print(s3)
#the calculation do not add up to 100. Is it because of the hydrolysis of proteins.
#amino acids when heated will decompose into carbon dioxide and ammonia.


