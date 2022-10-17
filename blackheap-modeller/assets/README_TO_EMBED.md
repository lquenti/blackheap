# The model created by blackheap

This folder is a performance model created using [blackheap](https://github.com/lquenti/blackheap). Here is a overview of the files found:

- `blackheap-benchmark.exe`: The benchmarker with which the models were created. This I/O benchmarker can be used as a standalone application as well. See `./blackheap-benchmark.exe --help` for more information.
- `index.html`: The frontend for local analysis. It provides a graphical interface for the `Model.json` as well as a way to manually analyze I/O requests measured via the preloadee. See the blackheap documentation for more information.
- `iofs.csv`: A simple version of the created performance models in order to be used with [iofs](https://github.com/gwdg/iofs) to automatically stream classified I/O requests to a TSDB.
- `Model.json`: A more verbose and complete version of the performance models. Can be viewed with the corresponding `index.html` provided in this folder.
- `RandomUncached/SameOffset...`: The raw, unanalyzed benchmark data. The JSON files correspond to the parameters provided to `blackheap-benchmark.exe`.
