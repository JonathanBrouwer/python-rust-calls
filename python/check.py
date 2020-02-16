from lobster_rust import increment

assert increment.increment_one(3) == 4
assert increment.Incrementer(5).apply(3) == 8
print("SUCCESS")