import numpy as np
import threading
import psutil
import time
from pyo3_demos import matrix_point_product_and_sum, matrix_point_product_and_sum_without_gil

def monitor_cpu(interval=1):
    """监控CPU使用率的线程函数"""
    while not stop_monitor:
        cpu_percent = psutil.cpu_percent(interval)
        print(f"CPU占用率: {cpu_percent}%")

def worker(func, arr):
    """工作线程函数"""
    for _ in range(1000):
        func(arr)

# 创建大型测试矩阵
matrix = np.random.rand(1000, 1000)

# 测试无GIL释放的版本
print("测试常规版本(有GIL):")
stop_monitor = False
monitor_thread = threading.Thread(target=monitor_cpu)
monitor_thread.start()

threads = []
for _ in range(4):
    t = threading.Thread(target=worker, args=(matrix_point_product_and_sum, matrix))
    threads.append(t)
    t.start()

for t in threads:
    t.join()

stop_monitor = True
monitor_thread.join()

# 测试有GIL释放的版本
print("\n测试GIL释放版本:")
stop_monitor = False
monitor_thread = threading.Thread(target=monitor_cpu)
monitor_thread.start()

threads = []
for _ in range(4):
    t = threading.Thread(target=worker, args=(matrix_point_product_and_sum_without_gil, matrix))
    threads.append(t)
    t.start()

for t in threads:
    t.join()

stop_monitor = True
monitor_thread.join()