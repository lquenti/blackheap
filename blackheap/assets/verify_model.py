import csv
import os
from dataclasses import dataclass
from typing import List, Optional, Union

@dataclass
class LinearModel:
    benchmark_type: str
    is_read_op: bool
    slope: float
    y_intercept: float
    left_bound: int
    right_bound: int

    def evaluate(self, x: float) -> float:
        return self.slope * x + self.y_intercept

@dataclass
class ConstLinearModel:
    benchmark_type: str
    is_read_op: bool
    parts: List[LinearModel]

    def evaluate(self, x: float) -> float:
        for part in self.parts:
            if (part.left_bound == 0 or x >= part.left_bound) and (part.right_bound == 0 or x < part.right_bound):
                return part.evaluate(x)
        raise Exception()


@dataclass
class CsvModels:
    filename: str
    models: List[Union[LinearModel, ConstLinearModel]]

    def classify(self, op: str, bytes: int, time: float) -> Optional[Union[LinearModel, ConstLinearModel]]:
        is_read_op = op == "r"
        filtered_models = [model for model in self.models if model.is_read_op == is_read_op]

        tightest_model = None
        tightest_upper_bound = float('inf')

        for model in filtered_models:
            try:
                evaluated_time = model.evaluate(bytes)
                if time < evaluated_time < tightest_upper_bound:
                    tightest_upper_bound = evaluated_time
                    tightest_model = model
            except Exception as e:
                print(f"Error evaluating model: {e}")

        return tightest_model

def detect_model_type(path: str) -> str:
    with open(path, newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        seen = set()
        for row in reader:
            key = (row['benchmark_type'], row['is_read_op'])
            if key in seen:
                return 'constlinear'
            seen.add(key)
    return 'linear'

def parse_csv(path: str, model_type: str):
    models = []
    with open(path, newline='') as csvfile:
        reader = csv.DictReader(csvfile)
        if model_type == 'linear':
            for row in reader:
                model = LinearModel(
                    benchmark_type=row['benchmark_type'],
                    is_read_op=bool(int(row['is_read_op'])),
                    slope=float(row['slope']),
                    y_intercept=float(row['y_intercept']),
                    left_bound=int(row['left_bound']),
                    right_bound=int(row['right_bound'])
                )
                models.append(model)
        elif model_type == 'constlinear':
            temp = {}
            for row in reader:
                key = (row['benchmark_type'], row['is_read_op'])
                if key not in temp:
                    temp[key] = []
                temp[key].append(LinearModel(
                    benchmark_type=row['benchmark_type'],
                    is_read_op=bool(int(row['is_read_op'])),
                    slope=float(row['slope']),
                    y_intercept=float(row['y_intercept']),
                    left_bound=int(row['left_bound']),
                    right_bound=int(row['right_bound'])
                ))
            for key, parts in temp.items():
                models.append(ConstLinearModel(
                    benchmark_type=key[0],
                    is_read_op=bool(int(key[1])),
                    parts=parts
                ))
    return models

def parse_models_from_csvs(directory_path: str) -> List[CsvModels]:
    csv_files = [f for f in os.listdir(directory_path) if f.endswith('.csv')]
    models_list = []

    for filename in csv_files:
        full_path = os.path.join(directory_path, filename)
        model_type = detect_model_type(full_path)
        models = parse_csv(full_path, model_type)
        models_list.append(CsvModels(filename, models))

    return models_list

@dataclass
class IORecord:
    classification: str
    io_type: str
    bytes: int
    sec: float

    @classmethod
    def parse_io_record(cls, line: str) -> "IORecord":
        # expecting the format
        # classification,io_type,bytes,sec
        fields = line.strip().split(',')
        classification, io_type, bytes_str, sec_str = fields
        return cls(
            classification=classification,
            io_type=io_type,
            bytes=int(bytes_str),
            sec=float(sec_str)
        )

def evaluate_model_accuracy(csv_file_path: str, csv_models: CsvModels):
    total_records = 0
    matched_and_evaluated_records = 0
    sum_absolute_error = 0.0

    with open(csv_file_path, 'r', newline='') as csvfile:
        reader = csv.reader(csvfile)
        next(reader)  # Skip header

        for i, line in enumerate(reader, start=1):
            io_record = IORecord.parse_io_record(','.join(line))
            
            tightest_model = csv_models.classify(io_record.io_type, io_record.bytes, io_record.sec)

            if tightest_model is not None:
                evaluated_time = tightest_model.evaluate(io_record.bytes)
                absolute_error = abs(evaluated_time - io_record.sec)
                sum_absolute_error += absolute_error
                matched_and_evaluated_records += 1

            # if i % 1000 == 0:
            #     print(f"Progress: {i} lines")

            total_records += 1

    if matched_and_evaluated_records > 0:
        average_absolute_error = sum_absolute_error / matched_and_evaluated_records
        accuracy_percentage = (matched_and_evaluated_records / total_records) * 100

        print(f"Total records processed: {total_records}")
        print(f"Records matched and evaluated with model: {matched_and_evaluated_records}")
        print(f"Percentage of accurately matched records: {accuracy_percentage:.2f}%")
        print(f"Average absolute error: {average_absolute_error:.6f}")
    else:
        print("No records matched with model.")


def main():
    models_directory_path = "./models"
    evaluation_csv_path = "./all_raw_data.csv"
    delimiter = "-" * 10

    csv_models_list = parse_models_from_csvs(models_directory_path)
    print(f"Loaded {len(csv_models_list)} models from {models_directory_path}\n{delimiter}")

    for csv_models in csv_models_list:
        print(f"Evaluating model from file: {csv_models.filename}")
        evaluate_model_accuracy(evaluation_csv_path, csv_models)
        print(delimiter)

if __name__ == "__main__":
    main()

