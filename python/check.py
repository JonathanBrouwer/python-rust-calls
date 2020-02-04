from increment import incrementer_one, Incrementer

assert incrementer_one(3) == 4
assert Incrementer(5).apply(3) == 8
print("SUCCESS")
