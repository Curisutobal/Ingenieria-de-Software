class Asiento:
    def __init__(self, numero):
        self.numero = numero
        self.disponible = True
        self.pagado = False

    def reservate(self):
        self.disponible = False

    def esDisponible(self):
        return self.disponible
    
    def sePago(self):
        self.pagado= True
        self.disponible = False

    def estasPagado(self):
        return self.pagado

    def estasReservado(self):
        return not self.disponible

class AdapterAsiento:
#Crear un método llamado es pagado
#Debe tener el estado y el asiento
  def __init__(self, estado, asiento):
    self.estado = estado
    self.asiento = asiento

  def esPagado(self,estado,asiento):
        self.estado = self.sePago()
        self.asiento = self.getPagados()
        if self.estado == True:
            return 2
            #2 Es pagado
        else:
            return 1
            #1 Es no pagado
    
        

class Vehiculo:
    def __init__(self, capacidad):
        self.asientos = []
        for i in range(capacidad):
            self.asientos.append(Asiento(i+1))

    def getCapacidad(self):
        return len(self.asientos)
    
    def reservarAsiento(self, numero):
        self.asientos[numero-1].reservate()

    def getDisponibilidad(self):
        disponibles = 0
        for i in range(self.getCapacidad()):
            if self.asientos[i].esDisponible():
                disponibles+=1
        return disponibles
    
    def pagoAsiento(self, numero):
        self.asientos[numero-1].sePago()

    def getPagados(self):
        pagados = 0
        for i in range(self.getCapacidad()):
            if self.asientos[i].estasPagado():
                pagados += 1
        return pagados
    

    def getReservados(self):
        reservados = 0
        for i in range(self.getCapacidad()):
            if self.asientos[i].estasReservado():
                reservados += 1
        return reservados

    def esPagados(self, estado, asiento):
      self.estado = self.sePago()
      self.asiento = asiento
      return self.esPagado(estado,asiento)
    


# Funciones que se realizan en la aplicación
v = Vehiculo(10)
v2 = Vehiculo(20)
print(v.getCapacidad())
print(v2.getCapacidad())

# Usuario reserva un asiento
print(v.getDisponibilidad())
v.reservarAsiento(6)
v.pagoAsiento(6)
v.reservarAsiento(7)
v.pagoAsiento(7)
print(v.getDisponibilidad())
print(v.getPagados())
print(v.getReservados())
print(v.getDisponibilidad())
print(v.esPagados())
