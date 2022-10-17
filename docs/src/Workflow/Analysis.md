# Analysis

After each benchmark, we get a lot of points `(access_size, time)` for each characteristics. Now, for each access size, we have to find a good prediction on how much time we can expect the access to take. This way, we can correlate a given operation with the different characteristics we measured.

Importantly, we can't assume that we have a normal distribution. This makes intuitive sense: On the one hand, calculating access times is a highly multivariate problem. On the other hand, things like shared queues will lead to a Gamma distribution, which is also why we can't just just a Gaussian Mixture Model.

Without any further assumptions, the trivial choice would be a histogram. Sadly, this leaves us with the problem of automatic bin number and width. Often times, those automatic rules can often hugely misrepresent the underlying PDF.

Instead we use a so-called [Kernel Density Estimation](https://en.wikipedia.org/wiki/Kernel_density_estimation), which creates a "smoother" histogram by putting a kernel over each point, then summing all kernels and renormalizing it to \\(\int f(x) = 1\\).

A more detailed introduction to KDEs and their advantages [in the great KDEpy documentation](https://kdepy.readthedocs.io/en/latest/introduction.html).

## Kernel Density Estimations (KDEs)

A KDE of points \\(x_1,\dots,x_n\\) can be simplified to

\\[
\hat{f}(x) = \frac{1}{N} \sum_{i=1}^N K(x-x_i)
\\]

where \\(K\\) is a so-called _kernel function_. This function can be understood in the following way:

- For each point \\(x_i, i \in \{1,\dots, n\}\\)
- Move the kernel function \\(x_i\\) to the right, to center it on this point
- Then take the already moved kernel function \\(K\\) with regard to \\(x\\)
- Take the sum of all those evaluated kernels
- And renormalize it so that the integral equals \\(1\\) i.e. the whole domain has a probability of 100%.

In our case, the kernel function is a [Gaussian](https://en.wikipedia.org/wiki/Gaussian_function).

## The bandwidth problem

In real computations, our actual formula is defined as

\\[
\hat{f}(x) = \frac{1}{Nh} \sum_{i=1}^N K\left(\frac{x-x_i}{h}\right)
\\]

where \\(h > 0\\) is the so-called _bandwidth_.

The bandwidth describes how the kernel needs to be scaled. We use the commonly used [Silverman's rule of thumb](https://doi.org/10.1201/9781315140919), provided by the [`criterion_stats`](https://docs.rs/crate/criterion-stats/0.2.1) package.

## Cluster detection

Now that we have a proper kernel density estimation for each characteristic and access time, we can further analyze our measurements.

At the time of this writing, we associate the time upper bound with the most likely access time, i.e. the global maximum of the corresponding KDE.

In the future, we want to use more sophisticated cluster detection for creating multiple regression functions. We currently already create clusters, although they are not yet used in the actual classification. They can be seen via the web interface.

## Our cluster algorithm

A cluster is defined by a local maximum and it's surrounding local minima. Note that the fastest and slowest access times are automatically the outer local minima.

At first, we just naively compute all cluster by numerically approximating their extrema.

Next, we try to find the "significant" clusters. A cluster is significant if and only if

\\[
\max_i - \min_{i+1} \geq 0.1 \cdot \text{global_max}
\\]

If a cluster is not significant, it gets merged into the last significant cluster.
