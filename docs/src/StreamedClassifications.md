# Streamed Multi Node Classifications with iofs

Blackheap also allows for aggregated, streamed classification to a remote TSDB via [iofs](https://github.com/gwdg/iofs). iofs is a FUSE based tool for measuring I/O requests. It works by remounting a file system, then interjecting any incoming I/O requests. See [the iofs documentation](https://gwdg.github.io/iofs/book/) for more information.

This allows for aggregate remote analysis of multiple nodes. It currently supports any monitoring system that is based on Influxdb. In our internal use case, it is used in combinaton with Grafana.

Again, no changes to the recorded software binary is required, although some configuration may be needed in order to write to the correct mount point.

## Setup

Here we have a the chicken and the egg type of problem: On the one hand, we want to do access classification based on the expected values of the FUSE file system. On the other hand, we theoretically already need those classifications before starting iofs. This is a complex [open problem](https://github.com/gwdg/iofs/issues/6).

As mentioned in the issue, there are basically two solutions. For now, this documentation will rely on the first idea since it cannot be assumed that one has that much computing time to create two seperate classification models. Feel free to use the other approach instead if you want, found [in the issue](https://github.com/gwdg/iofs/issues/6) as well.

### Step 0: Build iofs; mount it with fake classifications

Of course, iofs has to be built first. See [the related iofs documentation](https://gwdg.github.io/iofs/book/setup/Installation.html) for more.

After that, iofs should be mounted with a fake set of models in order to have the approximately correct overhead for I/O requests. Download them here related to their CLI parameter name:

- constant-linear (DEFAULT): [here](./DummyModels/constant-linear.csv)
- linear: [here](./DummyModels/linear.csv)

After that, mount iofs via

```
./iofs /new/mountpoint /where/real/data/is --classificationfile=/path/to/dummy.csv
```

or, if InfluxDB should be used:

```
iofs /new/fuse/mountpoint /where/real/data/is --classificationfile=/path/to/dummy.csv --in-server=http://influx_server:8086 --in-db=mydb
```

### Step 1: Create a performance model using the fake initial classifications

Note: This, of course, will stream wrong data to Influx. But that is okay. We just want to have the correct data produced by blackheap, which requires the correct request time overhead.

So, now the correct performance model can be created using blackheap. This requires that the benchmark is done on the iofs FUSE file system, which can be controlled via the `--file` parameter. Therefore, the minimal call would be

```
blackheap --file /path/to/somewhere/within/the/mountpoint
```

Afterwards, a model will be created at `$PWD/default_model`. This can be controlled via `--to`. See `blackheap --help` for more.

### Step 2: Remount iofs with the real classifications

Unmount iofs and remount with the newly created `default_model/iofs.csv`. Otherwise, the parameters are the same as in Step 0.

Afterwards, it should record and classify the I/O requests according to the performance model created.

## Setting up Grafana and InfluxDB for Remote Analysis

We provide a full docker-compose setup for InfluxDB+Grafana [in the iofs repository](https://gwdg.github.io/iofs/book/setup/LocalGrafana.html).

After the setup data can be streamed by mounting iofs with

```
iofs /new/fuse/mountpoint /where/real/data/is --classificationfile=/path/to/dummy.csv --in-server=http://influx_server:8086 --in-db=mydb
```

See [the iofs documentation](https://gwdg.github.io/iofs/book) for more information.
