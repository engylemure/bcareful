import threading

print("Let's use some threads to count")
amount_of_threads = int(input("which amount of threads do you want? "))
thread_count_to = int(input("amount to count to for each thread? "))

counter = 0

def one():
    return int(1)

def increment_counter():
    global counter
    for _ in range(0, int(thread_count_to)):
        counter = counter + one()

def start_counter_thread():
    thread = threading.Thread(target=increment_counter)
    thread.start()
    return thread

threads = [start_counter_thread() for _ in range(0, amount_of_threads)]

for thread in threads:
    thread.join()

print("Final counter value:", counter)
print("expected is:", amount_of_threads * thread_count_to)
