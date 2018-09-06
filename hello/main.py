import rustlib


twenty = rustlib.adder(10,10)

assert twenty == 20

print("10 + 10 == " + str(twenty))