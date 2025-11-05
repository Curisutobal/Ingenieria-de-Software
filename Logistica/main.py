class Vehiculo:
    marca = "Condorito"
    def __init__(self, name, max_velocity):
        self.name = name
        self.max_velocity = max_velocity
    def setCapacidad(self, capacity):
        self.capacity = capacity
    def ingreso(self,costo):
        self.ingreso = self.capacity*costo

class Bus(Vehiculo):
    pass
    
class Transfer(Vehiculo):
    def ingreso(self,costo, porcentaje):
        self.ingreso = self.capacity*costo*porcentaje


t1 = Transfer("chileo", 100)
print(t1.name)
t1.setCapacidad(15)
print(t1.capacity)
t1.ingreso(1000, 1.2)
print(t1.ingreso)
b = Bus("colorado", 120)
print(b.name)
b.setCapacidad(40)
print(b.capacity)
b.ingreso(200)
print(b.ingreso)
b2 = Bus("risueño", 90)
print(b2.name)
b2.setCapacidad(45)
print(b2.capacity)
b2.ingreso(500)
print(b2.ingreso)


Vehiculo.marca = "Yayita"

print(b.marca)
print(b2.marca)
print(t1.marca)









print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")
print("Oli OwO")

