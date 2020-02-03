import increment

assert increment.incrementer_one(3) == 4
assert increment.Incrementer(5).increment_by(3) == 8
print("SUCCESS")
