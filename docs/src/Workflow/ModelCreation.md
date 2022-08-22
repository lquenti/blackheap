# Model Creation

- Now after the KDEs
- For each type of benchmark we have many points
  (`access_size`, `time_of_global_maximum`)
- Now we need to create a model out of those values
- We have 2 different approaches

## Linear Model

- Just a simple linear regression of those points
- We are using the XXX rust library by YYY institute

## ConstLinear Model

- Assumption: We should expect linear speed below 4096 bytes, which is the default page cache size
- thus we use a piecewise function
- under 4096: the biggest local maximum of each KDE.
- over 4096: linear regression as above
