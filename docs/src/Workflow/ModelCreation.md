# Model Creation

After creating and analyzing the KDEs, we now have for each type of benchmark many points `(access_size, time_of_global_maximum)`.

Now we have to create a predictive model from those points. We currently have 2 different approaches:

## Linear Model

In order to create a linear model we do a [least square linear regression](https://en.wikipedia.org/wiki/Linear_least_squares) on each points of a given type of benchmark.

The regression functionality is provided by the great [`linregress`](https://github.com/n1m3/linregress) library.

## ConstLinear Model

This model works with the following assumption: We should expect linear speed below 4096 bytes, as this is the default page cache size.

Thus we use a piecewise function. The piecewise function is defined as follows:

```
def f(x):
  if x <= 4096:
    return global_maximum(1, 4096)
  else:
    return linear_model_beginning_at(4096)
```
