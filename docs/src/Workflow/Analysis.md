# Analysis

- After each benchmark, we get a lot of points
- Now we need to find out the relevance of those points
- We can't assume that they follow a normal distribution
- Without any further assumptions, the easiest choice would be a histogram
- But they can be decieving, see kdepy docs
- Thus we use a kernel density estimation

## KDE

- In KDEs, you put a function (in our case a gaussian curve) over each point
- then you normalize it to integral 1 to satisfy the PDF laws
- The only question is the bandwidth: how wide is the curve for each point?
- We use the XXX bandwidth estimator
- provided by YYY package

## Cluster detection
- Afterwards, we try to detect significant clusters
- The algortihm is very simple:
  - The first cluster starts at 0, then we go to the first significant maximum.
  - Then, we end the cluster at the next significant minimum
- Significant means, that (FORMULA)
- Currently, we only use the time corresponding to the global maximum of each KDE.
