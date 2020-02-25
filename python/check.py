# from increment import *
#
# assert increment_one(3) == 4
# assert Incrementer(5).apply(3) == 8
# print("SUCCESS")

from lobster_rust import ahrs

mw: ahrs.MadgwickP = ahrs.MadgwickP(0.2, 0.2, [1, 0, 0, 0])
mw.update(gyroscope=[1, 0, 0], accelerometer=[1, 0, 0], magnetometer=[1, 0, 0])
print(mw)
