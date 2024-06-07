
result=0
i=2
exponent=0
while [[ $result -lt 1000000 ]]; do
  result=$((2 ** exponent)) 
  echo $result
  exponent=$((exponent + 1))
  sleep 1  # Delays the output by 1 second to simulate a slow stream
done

