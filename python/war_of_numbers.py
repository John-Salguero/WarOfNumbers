import sys

def main(argv):
  even = 0
  odd =  0
  for sNum in argv:
    num = int(sNum)
    if num % 2 == 0:
      even += num
    else:
      odd += num
  print(f'Odd total: {odd}; Even total: {even}')
  if odd > even:
    print('Odd is greater.')
    print(f'Difference: {odd-even}')
  else:
    print('Even is greater.')
    print(f'Difference: {even-odd}')
  

main(sys.argv[1:]);
